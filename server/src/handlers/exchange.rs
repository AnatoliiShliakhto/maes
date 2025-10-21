use crate::{common::*, middleware::*, services::*};
use ::axum::Json;
use ::shared::{common::*, payloads::*};

pub async fn export(session: Session, Json(payload): Json<ExchangeExportPayload>) -> Result<()> {
    tokio::spawn(async move {
        let result = if payload.entities.is_empty() {
            ExchangeService::export_workspace(&session.workspace, payload.path).await
        } else {
            ExchangeService::export(&session.workspace, payload.entities, payload.path).await
        };

        match result {
            Ok(_) => {
                State::dispatcher().task_send(DispatcherTask::Finished);
                State::dispatcher().msg_send(DispatcherMessage::Info("export-success".into()))
            }
            Err(Error::Server(StatusCode::INTERNAL_SERVER_ERROR, _)) | Err(Error::Common(_)) => {
                State::dispatcher().task_send(DispatcherTask::Failed);
                State::dispatcher().msg_send(DispatcherMessage::Error("export-failed".into()))
            }
            Err(Error::Server(_, msg)) => {
                State::dispatcher().task_send(DispatcherTask::Failed);
                State::dispatcher().msg_send(DispatcherMessage::Error(msg))
            }
        }
    });

    Ok(())
}

pub async fn import(connection: Connection, Json(payload): Json<ExchangeImportPayload>) -> Result<()> {
    connection.checked()?;
    tokio::spawn(async move {
        match ExchangeService::import(&payload.path).await {
            Ok(_) => {
                State::dispatcher().task_send(DispatcherTask::Finished);
                State::dispatcher().msg_send(DispatcherMessage::Info("import-success".into()))
            }
            Err(Error::Server(StatusCode::INTERNAL_SERVER_ERROR, _)) | Err(Error::Common(_)) => {
                State::dispatcher().task_send(DispatcherTask::Failed);
                State::dispatcher().msg_send(DispatcherMessage::Error("import-failed".into()))
            }
            Err(Error::Server(_, msg)) => {
                State::dispatcher().task_send(DispatcherTask::Failed);
                State::dispatcher().msg_send(DispatcherMessage::Error(msg))
            }
        }
    });
    Ok(())
}
