use crate::{prelude::*, services::*};

static CLAIMS: GlobalSignal<Arc<Claims>> = Signal::global(|| Arc::new(Claims::default()));

pub struct AuthService;

impl AuthService {
    pub fn claims() -> Arc<Claims> {
        CLAIMS.signal()()
    }

    pub fn login(
        workspace: impl Into<String>,
        login: impl Into<String>,
        password: impl Into<String>,
    ) {
        api_fetch!(
            POST,
            "/api/v1/auth",
            AuthPayload {
                workspace: workspace.into(),
                login: login.into(),
                password: password.into(),
            },
            on_success = |body: Claims| {
                if ClientService::set_token(&body.token) { 
                    CLAIMS.with_mut(|c| *c = Arc::new(body));
                    use_app_state().set(AppState::Authorized)
                }
            },
        );
    }

    pub fn logout() {
        api_call!(
            DELETE,
            "/api/v1/auth",
            on_success = || {
                if ClientService::remove_token() {
                    CLAIMS.with_mut(|c| *c = Arc::new(Claims::default()));
                    ToastService::info(t!("logout-success"));
                    use_app_state().set(AppState::Running);
                }
            }
        )
    }
}
