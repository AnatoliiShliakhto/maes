use crate::prelude::*;

static CLAIMS: GlobalSignal<Option<Claims>> = Signal::global(|| None);

pub struct AuthService;

impl AuthService {
    pub fn login(
        workspace: impl Into<String>,
        login: impl Into<String>,
        password: impl Into<String>,
    ) {
        api_fetch!(
            POST,
            "/api/v1/auth",
            |body: Claims| {
                ClientService::set_token(&body.token);
                CLAIMS.with_mut(|c| *c = Some(body));
            },
            AuthPayload {
                workspace: workspace.into(),
                login: login.into(),
                password: password.into(),
            }
        );
    }

    pub fn logout() {
        api_call!(DELETE, "/api/v1/auth", || {
            CLAIMS.with_mut(|c| *c = None);
            ClientService::remove_token();
            widgets::ToastManager::info(t!("logout-success"));
        });
    }
}
