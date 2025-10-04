use crate::prelude::*;
use ::reqwest::{
    Client, Response, Url,
    header::{AUTHORIZATION, HeaderMap, HeaderValue, USER_AGENT},
};
use ::serde::{Serialize, de::DeserializeOwned};
use ::std::{
    string::ToString,
    sync::{Arc, LazyLock},
};
use ::tokio::sync::RwLock;

pub use ::reqwest::Method;
use crate::components::widgets;

static HTTP: LazyLock<Http> = LazyLock::new(Http::new);

struct Http {
    client: Client,
    base: Url,
    headers: Arc<RwLock<HeaderMap>>,
}

impl Http {
    fn new() -> Self {
        let config = service::ConfigService::read();
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(&config.server.ident).unwrap_or_else(|_| HeaderValue::from_static("maes-client")),
        );
        let (_scheme, _host, port) = parse_scheme_host_port(&config.server.host)
            .unwrap_or_else(|_| ("".to_string(), "".to_string(), 4583));
        let base = Url::parse(&format!("http://127.0.0.1:{port}")).unwrap();
        let client = Client::new();
        Self {
            client,
            base,
            headers: Arc::new(RwLock::new(headers)),
        }
    }

    fn url(&self, path: impl AsRef<str>) -> Url {
        self.base.join(path.as_ref()).unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct ClientService;

impl ClientService {
    pub fn set_token(token: impl AsRef<str>) {
        let token = format!("Bearer {token}", token = token.as_ref().trim());
        spawn(async move {
            if let Ok(hv) = HeaderValue::from_str(&token) {
                HTTP.headers.write().await.insert(AUTHORIZATION, hv);
            }
        });
    }

    pub fn remove_token() {
        spawn(async move {
            HTTP.headers.write().await.remove(AUTHORIZATION);
        });
    }

    pub async fn set_token_async(token: impl AsRef<str>) {
        if let Ok(hv) = HeaderValue::from_str(&format!("Bearer {token}", token = token.as_ref().trim())) {
            HTTP.headers.write().await.insert(AUTHORIZATION, hv);
        }
    }

    pub async fn remove_token_async() {
        HTTP.headers.write().await.remove(AUTHORIZATION);
    }

    #[inline]
    async fn request_with_headers(method: Method, url: Url) -> reqwest::RequestBuilder {
        let headers = HTTP.headers.read().await.clone();
        HTTP.client.request(method, url).headers(headers)
    }

    async fn handle_response(
        response: std::result::Result<Response, reqwest::Error>,
    ) -> Result<Response> {
        let response = response.map_err(|_| "network-error")?;
        if !response.status().is_success() {
            let message = response.text().await.map_err(|_| "http-error")?;
            Err(message)?
        } else {
            Ok(response)    
        }
    }

    async fn handle_json_response<T: DeserializeOwned>(
        response: std::result::Result<Response, reqwest::Error>,
    ) -> Result<T> {
        let response = Self::handle_response(response).await?;
        response
            .json::<T>()
            .await
            .map_err(|_| "deserialize-error".into())
    }

    fn build_request(
        method: Method,
        endpoint: impl AsRef<str>,
    ) -> (Url, Method) {
        let url = HTTP.url(endpoint);
        (url, method)
    }

    async fn execute_request(
        method: Method,
        endpoint: impl AsRef<str>,
        payload: Option<impl Serialize>,
    ) -> Result<()> {
        let (url, method) = Self::build_request(method, endpoint);
        let mut request = Self::request_with_headers(method, url).await;
        if let Some(payload) = payload {
            request = request.json(&payload);
        }
        Self::handle_response(request.send().await).await?;
        Ok(())
    }

    async fn execute_request_with_json<T: DeserializeOwned + 'static>(
        method: Method,
        endpoint: impl AsRef<str>,
        payload: Option<impl Serialize>,
    ) -> Result<T> {
        let (url, method) = Self::build_request(method, endpoint);
        let mut request = Self::request_with_headers(method, url).await;
        if let Some(payload) = payload {
            request = request.json(&payload);
        }
        Self::handle_json_response(request.send().await).await
    }

    fn execute_request_with_callbacks(
        method: Method,
        endpoint: impl AsRef<str>,
        payload: Option<impl Serialize + Clone + 'static>,
        on_success: impl FnOnce() + 'static,
        on_error: impl FnOnce(Error) + 'static,
    ) {
        let url = HTTP.url(endpoint);
        spawn(async move {
            let headers = HTTP.headers.read().await.clone();
            let mut request = HTTP.client.request(method, url).headers(headers);
            if let Some(payload) = payload {
                request = request.json(&payload);
            }
            match Self::handle_response(request.send().await).await {
                Ok(_) => on_success(),
                Err(e) => on_error(e),
            }
        });
    }

    fn execute_request_with_json_callbacks<T: DeserializeOwned + 'static>(
        method: Method,
        endpoint: impl AsRef<str>,
        payload: Option<impl Serialize + Clone + 'static>,
        on_success: impl FnOnce(T) + 'static,
        on_error: impl FnOnce(Error) + 'static,
    ) {
        let url = HTTP.url(endpoint);
        spawn(async move {
            let headers = HTTP.headers.read().await.clone();
            let mut request = HTTP.client.request(method, url).headers(headers);
            if let Some(payload) = payload {
                request = request.json(&payload);
            }
            match Self::handle_json_response(request.send().await).await {
                Ok(body) => on_success(body),
                Err(e) => on_error(e),
            }
        });
    }
}

pub fn api_error_handler(e: Error) {
    widgets::ToastManager::error(t!(e.to_string()))
}

#[macro_export]
macro_rules! __api_async_dispatch {
    ($exec:path, $method:ident, $endpoint:expr $(, $payload:expr)? $(,)?) => {{
        let payload = None::<_>;
        $(let payload = Some($payload);)?
        $exec(
            $crate::service::Method::$method,
            $endpoint,
            payload,
        )
    }};
}

#[macro_export]
macro_rules! api_call_async {
    ($method:ident, $endpoint:expr $(,)?) => {
        $crate::__api_async_dispatch!(
            $crate::service::ClientService::execute_request,
            $method, $endpoint
        )
    };
    ($method:ident, $endpoint:expr, $payload:expr $(,)?) => {
        $crate::__api_async_dispatch!(
            $crate::service::ClientService::execute_request,
            $method, $endpointoad $payload
        )
    };
}

#[macro_export]
macro_rules! api_fetch_async {
    ($method:ident, $endpoint:expr $(,)?) => {
        $crate::__api_async_dispatch!(
            $crate::service::ClientService::execute_request_with_json::<_>,
            $method, $endpoint
        )
    };
    ($method:ident, $endpoint:expr, $payload:expr $(,)?) => {
        $crate::__api_async_dispatch!(
            $crate::service::ClientService::execute_request_with_json::<_>,
            $method, $endpoint, $payload
        )
    };
}

#[macro_export]
macro_rules! __api_dispatch {
    ($exec:path, $method:ident, $endpoint:expr, $on_success:expr $(, $payload:expr)? $(, $on_error:expr)? $(,)?) => {{
        let payload = None::<_>;
        $(let payload = Some($payload);)?

        let on_error_cb = $crate::service::api_error_handler;
        $(let on_error_cb = $on_error;)?

        $exec(
            $crate::service::Method::$method,
            $endpoint,
            payload,
            $on_success,
            on_error_cb,
        )
    }};
}

#[macro_export]
macro_rules! api_call {
    ($method:ident, $endpoint:expr, $on_success:expr $(, $payload:expr)? $(, $on_error:expr)? $(,)?) => {
        $crate::__api_dispatch!(
            $crate::service::ClientService::execute_request_with_callbacks,
            $method, $endpoint, $on_success $(, $payload)? $(, $on_error)?
        )
    };
}

#[macro_export]
macro_rules! api_fetch {
    ($method:ident, $endpoint:expr, $on_success:expr $(, $payload:expr)? $(, $on_error:expr)? $(,)?) => {
        $crate::__api_dispatch!(
            $crate::service::ClientService::execute_request_with_json_callbacks,
            $method, $endpoint, $on_success $(, $payload)? $(, $on_error)?
        )
    };
}