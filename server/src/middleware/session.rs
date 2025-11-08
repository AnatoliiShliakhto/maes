use crate::services::Store;
use ::axum::{
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use ::dashmap::{DashMap, DashSet};
use ::serde::{Deserialize, Serialize};
use ::shared::{common::*, models::*, utils::*};
use ::std::sync::{Arc, LazyLock};

static SESSIONS: LazyLock<DashMap<String, Arc<ClientSession>>> = LazyLock::new(DashMap::new);

static ID_INDEX: LazyLock<DashMap<String, DashSet<String>>> = LazyLock::new(DashMap::new);

pub struct SessionService;

impl SessionService {
    pub async fn add_session(session: ClientSession) -> Claims {
        let token = safe_nanoid!();
        let arc = Arc::new(session);

        let claims = Claims {
            id: arc.id.clone(),
            ws_id: arc.workspace.clone(),
            username: arc.username.clone(),
            workspace: arc.workspace_name.clone(),
            version: arc.workspace_version.clone(),
            node: arc.node.clone(),
            role: arc.role.clone(),
            token: token.clone(),
        };

        SESSIONS.insert(token.clone(), Arc::clone(&arc));
        ID_INDEX
            .entry(claims.id.clone())
            .or_insert_with(DashSet::new)
            .insert(token);

        claims
    }

    pub async fn get_session(token: &str) -> Option<Arc<ClientSession>> {
        SESSIONS.get(token).map(|e| Arc::clone(e.value()))
    }

    pub async fn remove_session(id_or_token: impl AsRef<str>) {
        let key = id_or_token.as_ref();

        if let Some((token, sess)) = SESSIONS.remove(key) {
            if let Some(set) = ID_INDEX.get(&sess.id) {
                set.remove(&token);
                let empty = set.is_empty();
                drop(set);
                if empty {
                    ID_INDEX.remove(&sess.id);
                }
            }
            return;
        }

        if let Some((_id, set)) = ID_INDEX.remove(key) {
            let tokens: Vec<String> = set.iter().map(|t| t.clone()).collect();
            for t in tokens {
                SESSIONS.remove(&t);
            }
        }
    }
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
pub struct ClientSession {
    pub id: String,
    pub workspace: String,
    pub workspace_name: String,
    pub workspace_version: String,
    pub username: String,
    pub node: String,
    pub path: String,
    pub role: WorkspaceRole,
}

pub struct Session {
    pub token: String,
    pub inner: Arc<ClientSession>,
}

impl Session {
    pub fn checked_admin(&self) -> Result<()> {
        if self.role != WorkspaceRole::Admin {
            Err((StatusCode::FORBIDDEN, "forbidden"))?
        } else {
            Ok(())
        }
    }

    pub fn checked_supervisor(&self) -> Result<()> {
        if self.role == WorkspaceRole::Admin || self.role == WorkspaceRole::Supervisor {
            Ok(())
        } else {
            Err((StatusCode::FORBIDDEN, "forbidden"))?
        }
    }

    pub async fn nodes(&self) -> Result<Option<Vec<String>>> {
        if self.node.is_empty() {
            return Ok(None);
        }

        let nodes = Store::find::<Workspace>(&self.workspace, &self.workspace)
            .await?
            .read()
            .await
            .unit_tree
            .node_descendants(&self.node);
        Ok(Some(nodes))
    }
}

impl std::ops::Deref for Session {
    type Target = Arc<ClientSession>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<S> FromRequestParts<S> for Session
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let auth = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or((StatusCode::UNAUTHORIZED, "unauthorized"))?;

        let token = auth
            .strip_prefix("Bearer ")
            .or_else(|| auth.strip_prefix("bearer "))
            .ok_or((StatusCode::UNAUTHORIZED, "unauthorized"))?
            .trim()
            .to_string();

        let session = SessionService::get_session(&token)
            .await
            .ok_or((StatusCode::UNAUTHORIZED, "unauthorized"))?;

        Ok(Session {
            token,
            inner: session,
        })
    }
}
