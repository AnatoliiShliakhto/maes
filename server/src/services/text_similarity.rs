use ::onnxruntime::{
    GraphOptimizationLevel,
    environment::Environment,
    ndarray::{Array2, IxDyn},
    session::Session,
    tensor::OrtOwnedTensor,
};
use ::shared::common::*;
use ::std::{env, path::Path, sync::Mutex};
use ::tokenizers::{Tokenizer, tokenizer::EncodeInput};
use ::tokio::sync::{mpsc, oneshot};
use ::tracing::error;

const MAX_LEN: usize = 256;

#[derive(Copy, Clone)]
pub struct TextSimilarityService;

type AsyncJob = (String, String, oneshot::Sender<Result<usize>>);

use std::sync::mpsc as std_mpsc;
type SyncReply = std_mpsc::SyncSender<usize>;

enum AnyJob {
    Async(AsyncJob),
    Sync(String, String, SyncReply),
}

static SENDER_ANY: Mutex<Option<mpsc::UnboundedSender<AnyJob>>> = Mutex::new(None);

impl TextSimilarityService {
    pub async fn init() -> Result<()> {
        let mut guard = SENDER_ANY.lock().unwrap();
        if guard.is_some() {
            return Err(Error::from("text-similarity already initialized"));
        }

        let exe = env::current_exe().map_err(map_log_err)?;
        let exe_dir = exe.parent().ok_or_else(|| Error::from("text-similarity: exe dir not found"))?;
        let model_dir = exe_dir.join("assets");
        let (tx_any, mut rx_any) = mpsc::unbounded_channel::<AnyJob>();
        *guard = Some(tx_any.clone());
        drop(guard);

        std::thread::spawn(move || {
            if let Err(e) = worker_loop_any(&model_dir, &mut rx_any) {
                error!("text-similarity worker failed: {e}");
            }
        });

        Ok(())
    }

    pub async fn compare(a: impl Into<String>, b: impl Into<String>) -> Result<usize> {
        let tx = {
            let guard = SENDER_ANY.lock().unwrap();
            guard
                .as_ref()
                .cloned()
                .ok_or_else(|| Error::from("text-similarity not initialized"))?
        };

        let (resp_tx, resp_rx) = oneshot::channel();
        tx.send(AnyJob::Async((a.into(), b.into(), resp_tx)))
            .map_err(|_| Error::from("worker channel closed"))?;
        resp_rx.await.map_err(|_| Error::from("worker dropped"))?
    }

    pub fn compare_sync(a: impl Into<String>, b: impl Into<String>) -> usize {
        let tx = {
            let guard = SENDER_ANY.lock().unwrap();
            match guard.as_ref().cloned() {
                Some(tx) => tx,
                None => return 0,
            }
        };

        let (reply_tx, reply_rx) = std_mpsc::sync_channel(1);
        let _ = tx.send(AnyJob::Sync(a.into(), b.into(), reply_tx));
        reply_rx.recv().unwrap_or(0)
    }
}

fn worker_loop_any(model_dir: &Path, rx: &mut mpsc::UnboundedReceiver<AnyJob>) -> Result<()> {
    let env = Environment::builder()
        .with_name("maes")
        .build()
        .map_err(map_log_err)?;
    let env: &'static Environment = Box::leak(Box::new(env));

    let tokenizer_path = model_dir.join("tokenizer.json");
    let model_path = model_dir.join("model.onnx");

    if !tokenizer_path.exists() || !model_path.exists() {
        error!("text-similarity assets not found in {model_dir:?}");
        return Err(Error::from("text-similarity assets missing"));
    }
    let tokenizer = Tokenizer::from_file(&tokenizer_path).map_err(map_log_err)?;
    let mut session = env
        .new_session_builder()
        .map_err(map_log_err)?
        .with_optimization_level(GraphOptimizationLevel::All)
        .map_err(map_log_err)?
        .with_model_from_file(model_path)
        .map_err(map_log_err)?;

    while let Some(job) = rx.blocking_recv() {
        match job {
            AnyJob::Async((a, b, reply)) => {
                let res = compare_impl(&tokenizer, &mut session, &a, &b).map_err(|e| {
                    error!("compare_impl failed: {e}");
                    e
                });
                let _ = reply.send(res);
            }
            AnyJob::Sync(a, b, reply) => {
                let res = compare_impl(&tokenizer, &mut session, &a, &b)
                    .map(|v| v)
                    .unwrap_or_default();
                let _ = reply.send(res);
            }
        }
    }

    Ok(())
}

fn compare_impl(
    tokenizer: &Tokenizer,
    session: &mut Session<'static>,
    a: &str,
    b: &str,
) -> Result<usize> {
    let embeddings = encode(tokenizer, session, &[a, b], MAX_LEN)?;
    let sim = cosine_similarity(&embeddings[0], &embeddings[1]);
    Ok((sim * 100f32).round() as usize)
}

fn encode(
    tokenizer: &Tokenizer,
    session: &mut Session<'static>,
    texts: &[&str],
    max_len: usize,
) -> Result<Vec<Vec<f32>>> {
    let mut input_ids = Vec::<i64>::new();
    let mut attention_mask = Vec::<i64>::new();
    let mut token_type_ids = Vec::<i64>::new();

    for text in texts {
        let enc = tokenizer
            .encode(EncodeInput::Single((*text).to_string().into()), true)
            .map_err(map_log_err)?;

        let mut ids: Vec<i64> = enc.get_ids().iter().map(|&v| v as i64).collect();
        let mut mask: Vec<i64> = enc.get_attention_mask().iter().map(|&v| v as i64).collect();
        let mut type_ids: Vec<i64> = enc.get_type_ids().iter().map(|&v| v as i64).collect();

        ids.truncate(max_len);
        mask.truncate(max_len);
        type_ids.truncate(max_len);
        if ids.len() < max_len {
            ids.resize(max_len, 0);
            mask.resize(max_len, 0);
            type_ids.resize(max_len, 0);
        }
        input_ids.extend(ids);
        attention_mask.extend(mask);
        token_type_ids.extend(type_ids);
    }

    let batch = texts.len();
    let seq_len = max_len;

    let input_ids_arr = Array2::from_shape_vec((batch, seq_len), input_ids).map_err(map_log_err)?;
    let attention_mask_arr =
        Array2::from_shape_vec((batch, seq_len), attention_mask.clone()).map_err(map_log_err)?;
    let token_type_ids_arr =
        Array2::from_shape_vec((batch, seq_len), token_type_ids.clone()).map_err(map_log_err)?;

    let input_info = session.inputs.iter().map(|i| i.name.clone()).collect::<Vec<_>>();
    if !(input_info.len() == 2 || input_info.len() == 3) {
        error!("Unexpected number of model inputs: {input_info:?}. Expected 2 or 3");
        Err("onnx-error")?
    }
    use onnxruntime::ndarray::ArrayD;
    let mut by_index: Vec<Option<ArrayD<i64>>> = vec![None; input_info.len()];
    for (i, name) in input_info.iter().enumerate() {
        match name.as_str() {
            "input_ids" => by_index[i] = Some(input_ids_arr.clone().into_dyn()),
            "attention_mask" => by_index[i] = Some(attention_mask_arr.clone().into_dyn()),
            "token_type_ids" => by_index[i] = Some(token_type_ids_arr.clone().into_dyn()),
            other => {
                error!("Unknown input name: {other}");
                Err("onnx-error")?
            }
        }
    }
    let inputs_vec: Vec<ArrayD<i64>> = by_index
        .into_iter()
        .map(|o| {
            o.ok_or_else(|| {
                error!("Failed to bind all inputs");
                "onnx-error".into()
            })
        })
        .collect::<Result<_>>()?;

    let outputs: Vec<OrtOwnedTensor<f32, IxDyn>> = session.run(inputs_vec).map_err(map_log_err)?;

    let out = outputs
        .into_iter()
        .next()
        .ok_or_else(|| Error::from("onnx-error"))?;
    let view = out.view();
    let shape = view.shape();
    if shape.len() != 3 {
        error!("Unexpected output shape: {shape:?}");
        Err("onnx-error")?
    }
    let hidden = shape[2];

    let last_data = view.as_slice().ok_or_else(|| {
        error!("Output tensor is not contiguous");
        Error::from("onnx-error")
    })?;

    let mut result = Vec::with_capacity(batch);
    for b in 0..batch {
        let offset = b * seq_len * hidden;
        let row = &last_data[offset..offset + seq_len * hidden];
        let attn_row = &attention_mask[b * seq_len..(b + 1) * seq_len];
        let pooled = mean_pool(row, attn_row, seq_len, hidden);
        let mut norm: f32 = pooled.iter().map(|v| v * v).sum::<f32>().sqrt();
        if norm == 0.0 {
            norm = 1.0;
        }
        let pooled = pooled.into_iter().map(|v| v / norm).collect::<Vec<_>>();
        result.push(pooled);
    }
    Ok(result)
}

fn mean_pool(last_hidden: &[f32], attn: &[i64], seq_len: usize, hidden: usize) -> Vec<f32> {
    let mut sum = vec![0f32; hidden];
    let mut count = 0f32;
    for t in 0..seq_len {
        if attn[t] == 0 {
            continue;
        }
        let row = &last_hidden[t * hidden..(t + 1) * hidden];
        for i in 0..hidden {
            sum[i] += row[i];
        }
        count += 1.0;
    }
    if count > 0.0 {
        for i in 0..hidden {
            sum[i] /= count;
        }
    }
    sum
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let na = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let nb = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if na == 0.0 || nb == 0.0 {
        0.0
    } else {
        dot / (na * nb)
    }
}
