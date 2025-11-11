import torch
from sentence_transformers import SentenceTransformer
import torch.nn as nn
import shutil
import os

# 1. Loading the Sentence Transformer model
MODEL_NAME = 'lang-uk/ukr-paraphrase-multilingual-mpnet-base'
ONNX_DIR = os.path.join(os.getcwd(), 'onnx_models')
os.makedirs(ONNX_DIR, exist_ok=True)
ONNX_PATH = os.path.join(ONNX_DIR, 'model.onnx')

# Path to assets for Rust
ASSETS_DIR = os.path.join(os.getcwd(), 'assets')
os.makedirs(ASSETS_DIR, exist_ok=True)

# Load the entire model (architecture and weights)
sbert_model = SentenceTransformer(MODEL_NAME)
sbert_model.eval()

# We only need the first module (Transformer block)
transformer_module = sbert_model[0]

# --- Wrapper for ONNX export ---
class SBERTOnnxWrapper(nn.Module):
    """
Wrapper that integrates the Transformer block and the Pooling layer
to ensure a unified forward-pass for ONNX tracing.
    """
    def __init__(self, model):
        super(SBERTOnnxWrapper, self).__init__()

        # Extracting the Transformer part (0th element)
        self.transformer = model[0].auto_model

        # Extracting the Pooling part (1st element)
        pooling_layer = model[1]

        # Get pooling configuration via get_config_dict
        pooling_config = pooling_layer.get_config_dict()
        self.pooling_mode = pooling_config.get('pooling_mode', {})
        self.word_embedding_dimension = pooling_layer.word_embedding_dimension

        # Create a dummy pooling layer (for tracing)
        # Check if mean pooling is used
        if pooling_config.get('pooling_mode_mean_tokens', False):
            # In Sentence Transformers, this is usually Mean Pooling
            self.pooling = lambda embeddings, mask: self._mean_pooling(embeddings, mask)
        else:
            raise NotImplementedError("Only Mean Pooling is supported for ONNX export.")

    @staticmethod
    def _mean_pooling(model_output, attention_mask):
        """
Mean Pooling calculation (averaging output data)
        """
        # (batch_size, sequence_length, embedding_dim)
        token_embeddings = model_output[0]
        # Expand the mask to [batch_size, sequence_length, 1] for multiplication
        input_mask_expanded = attention_mask.unsqueeze(-1).float()

        # Sum of tokens multiplied by the mask
        sum_embeddings = torch.sum(token_embeddings * input_mask_expanded, 1)

        # Calculation of the number of non-zero tokens
        sum_mask = torch.clamp(input_mask_expanded.sum(1), min=1e-9)

        # Divide the sum by the number of tokens
        return sum_embeddings / sum_mask


    def forward(self, input_ids: torch.Tensor, attention_mask: torch.Tensor, token_type_ids: torch.Tensor = None) -> torch.Tensor:
        """
Ensures one forward-pass: Transformer -> Pooling -> Embedding.
        """
        # 1. Pass through Transformer
        model_output = self.transformer(
            input_ids=input_ids,
            attention_mask=attention_mask,
            token_type_ids=token_type_ids
        )

        # 2. Pass through the Pooling layer (Mean Pooling)
        embeddings = self.pooling(model_output, attention_mask)

        # 3. Normalization (if present in the SBERT model)
        embeddings = nn.functional.normalize(embeddings, p=2, dim=1)

        return embeddings

# Wrapper initialization
onnx_wrapper = SBERTOnnxWrapper(sbert_model)

# 3. Creating dummy input data (Dummy Input)
# Maximum sequence length
MAX_SEQ_LENGTH = 128
# Batch size (1)
BATCH_SIZE = 1

# Creating dummy tensors
# input_ids: token indices
dummy_input_ids = torch.randint(0, transformer_module.tokenizer.vocab_size, (BATCH_SIZE, MAX_SEQ_LENGTH))
# attention_mask: 1 for tokens, 0 for padding
dummy_attention_mask = torch.ones(BATCH_SIZE, MAX_SEQ_LENGTH, dtype=torch.long)
# token_type_ids: needed for some models (like BERT/MPNet)
dummy_token_type_ids = torch.zeros(BATCH_SIZE, MAX_SEQ_LENGTH, dtype=torch.long)

# Tuple of input data for export
dummy_inputs = (dummy_input_ids, dummy_attention_mask, dummy_token_type_ids)

# 4. Setting up dynamic axes,
# We want batch_size and sequence_length to be dynamic
dynamic_axes = {
    'input_ids': {0: 'batch_size', 1: 'sequence_length'},
    'attention_mask': {0: 'batch_size', 1: 'sequence_length'},
    'token_type_ids': {0: 'batch_size', 1: 'sequence_length'},
    'embeddings': {0: 'batch_size'}
}

# 5. Export to ONNX
try:
    torch.onnx.export(
        onnx_wrapper,
        dummy_inputs,
        ONNX_PATH,
        export_params=True,
        opset_version=11,
        do_constant_folding=True,
        input_names=['input_ids', 'attention_mask', 'token_type_ids'],
        output_names=['embeddings'],
        dynamic_axes=dynamic_axes
    )
    print(f"\n✅ ONNX model conversion successful: {ONNX_PATH}")

    # Check if a .data file was created
    data_file = ONNX_PATH + '.data'
    if os.path.exists(data_file):
        print(f"⚠️  Separate file with weights created: {data_file}")

    # 6. Exporting the tokenizer
    print("\n🔄 Exporting tokenizer...")
    tokenizer_temp_dir = os.path.join(ASSETS_DIR, 'tokenizer_temp')
    os.makedirs(tokenizer_temp_dir, exist_ok=True)

    # Save tokenizer to a temporary folder
    sbert_model.tokenizer.save_pretrained(tokenizer_temp_dir)

    # Copy tokenizer.json to assets
    tokenizer_json_path = os.path.join(tokenizer_temp_dir, 'tokenizer.json')
    tokenizer_dest_path = os.path.join(ASSETS_DIR, 'tokenizer.json')

    if os.path.exists(tokenizer_json_path):
        shutil.copy(tokenizer_json_path, tokenizer_dest_path)
        print(f"✅ Tokenizer saved: {tokenizer_dest_path}")
        # Remove the temporary folder
        shutil.rmtree(tokenizer_temp_dir)
    else:
        print(f"❌ tokenizer.json file not found in {tokenizer_temp_dir}")

    # 7. Copying files to assets
    print("\n🔄 Copying files to assets/...")
    shutil.copy(ONNX_PATH, os.path.join(ASSETS_DIR, 'model.onnx'))
    if os.path.exists(data_file):
        shutil.copy(data_file, os.path.join(ASSETS_DIR, 'model.onnx.data'))

    print("\n✅ All files are ready in the assets/ folder:")
    print(f"   - {os.path.join(ASSETS_DIR, 'model.onnx')}")
    if os.path.exists(data_file):
        print(f"   - {os.path.join(ASSETS_DIR, 'model.onnx.data')}")
    print(f"   - {tokenizer_dest_path}")

except Exception as e:
    print(f"\n❌ Error during export: {e}")
    import traceback
    traceback.print_exc()