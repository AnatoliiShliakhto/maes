use crate::{prelude::*, services::*};
use ::reqwest::{
    Client, Response, Url,
    header::*,
};
use ::serde::{Serialize, de::DeserializeOwned};
use ::std::{string::ToString, sync::LazyLock, time::Duration};
pub use ::reqwest::Method;

static HTTP: LazyLock<Http> = LazyLock::new(Http::new);

struct Http {
    client: Client,
    base: Url,
    headers: Arc<RwLock<HeaderMap>>,
}

impl Http {
    fn new() -> Self {
        let config = ConfigService::read();

        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(&config.server.ident)
                .unwrap_or_else(|_| HeaderValue::from_static("maes-client")),
        );
        headers.insert(CACHE_CONTROL, HeaderValue::from_static("no-cache, no-store, must-revalidate"));
        headers.insert(PRAGMA, HeaderValue::from_static("no-cache"));

        let (_scheme, _host, port) = parse_scheme_host_port(&config.server.host)
            .unwrap_or_else(|_| ("".to_string(), "".to_string(), 4583));
        let base = Url::parse(&format!("http://127.0.0.1:{port}")).unwrap();

        let client = Client::builder()
            .pool_idle_timeout(Duration::from_secs(30))
            .pool_max_idle_per_host(4)
            .tcp_keepalive(Duration::from_secs(30))
            .build().unwrap_or_default();

        Self {
            client,
            base,
            headers: Arc::new(RwLock::new(headers)),
        }
    }

    fn url(&self, path: impl AsRef<str>) -> Url {
        self.base.join(path.as_ref().trim_end_matches("/")).unwrap()
    }
}

#[derive(Copy, Clone)]
pub struct ClientService;

impl ClientService {
    pub fn set_token(token: impl AsRef<str>) -> bool {
        let Ok(hv) =
            HeaderValue::from_str(&format!("Bearer {token}", token = token.as_ref().trim()))
        else {
            return false;
        };
        let Ok(mut guard) = HTTP.headers.try_write() else {
            return false;
        };
        guard.insert(AUTHORIZATION, hv);
        true
    }

    pub fn remove_token() -> bool {
        let Ok(mut guard) = HTTP.headers.try_write() else {
            return false;
        };
        guard.remove(AUTHORIZATION);
        true
    }

    #[inline]
    fn request_with_headers(method: Method, url: Url) -> reqwest::RequestBuilder {
        if let Ok(headers) = HTTP.headers.try_read() {
            let headers = headers.clone();
            return HTTP.client.request(method, url).headers(headers);
        }
        HTTP.client.request(method, url)
    }

    async fn handle_response(
        response: std::result::Result<Response, reqwest::Error>,
    ) -> Result<Response> {
        let response = response.map_err(|_| "network-error")?;
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let message = if let Ok(text) = response.text().await
                && !text.is_empty()
            {
                text
            } else {
                format!("status-code-{status}")
            };
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

    fn build_request(method: Method, endpoint: impl AsRef<str>) -> (Url, Method) {
        let url = HTTP.url(endpoint);
        (url, method)
    }

    pub async fn execute_request(
        method: Method,
        endpoint: impl AsRef<str>,
        payload: Option<impl Serialize>,
    ) -> Result<()> {
        let (url, method) = Self::build_request(method, endpoint);
        let mut request = Self::request_with_headers(method, url);
        if let Some(payload) = payload {
            request = request.json(&payload);
        }
        Self::handle_response(request.send().await).await?;
        Ok(())
    }

    pub async fn execute_request_with_json<T: DeserializeOwned + 'static>(
        method: Method,
        endpoint: impl AsRef<str>,
        payload: Option<impl Serialize>,
    ) -> Result<T> {
        let (url, method) = Self::build_request(method, endpoint);
        let mut request = Self::request_with_headers(method, url);
        if let Some(payload) = payload {
            request = request.json(&payload);
        }
        Self::handle_json_response(request.send().await).await
    }

    pub fn execute_request_with_callbacks(
        method: Method,
        endpoint: impl AsRef<str>,
        payload: Option<impl Serialize + Clone + 'static>,
        on_success: impl FnOnce() + 'static,
        on_error: impl FnOnce(Error) + 'static,
    ) {
        let (url, method) = Self::build_request(method, endpoint);
        let mut request = Self::request_with_headers(method, url);
        if let Some(payload) = payload {
            request = request.json(&payload);
        }
        spawn(async move {
            match Self::handle_response(request.send().await).await {
                Ok(_) => on_success(),
                Err(e) => on_error(e),
            }
        });
    }

    pub fn execute_request_with_json_callbacks<T: DeserializeOwned + 'static>(
        method: Method,
        endpoint: impl AsRef<str>,
        payload: Option<impl Serialize + Clone + 'static>,
        on_success: impl FnOnce(T) + 'static,
        on_error: impl FnOnce(Error) + 'static,
    ) {
        let (url, method) = Self::build_request(method, endpoint);
        let mut request = Self::request_with_headers(method, url);
        if let Some(payload) = payload {
            request = request.json(&payload);
        }
        spawn(async move {
            match Self::handle_json_response(request.send().await).await {
                Ok(body) => on_success(body),
                Err(e) => on_error(e),
            }
        });
    }
}

pub fn api_error_handler(e: Error) {
    ToastService::error(t!(e.to_string()))
}

#[macro_export]
macro_rules! __api_async_dispatch {
    ($exec:path, $method:ident, $endpoint:expr $(, $payload:expr)? $(,)?) => {{
        let payload = None::<()>;
        $(let payload = Some($payload);)?
        $exec(
            $crate::services::Method::$method,
            $endpoint,
            payload,
        )
    }};
}

#[macro_export]
macro_rules! api_call_async {
    ($method:ident, $endpoint:expr $(,)?) => {
        $crate::__api_async_dispatch!(
            $crate::services::ClientService::execute_request,
            $method,
            $endpoint
        )
    };
    ($method:ident, $endpoint:expr, $payload:expr $(,)?) => {
        $crate::__api_async_dispatch!(
            $crate::services::ClientService::execute_request,
            $method,
            $endpoint,
            $payload
        )
    };
}

#[macro_export]
macro_rules! api_fetch_async {
    ($method:ident, $endpoint:expr $(,)?) => {
        $crate::__api_async_dispatch!(
            $crate::services::ClientService::execute_request_with_json::<_>,
            $method,
            $endpoint
        )
    };
    ($method:ident, $endpoint:expr, $payload:expr $(,)?) => {
        $crate::__api_async_dispatch!(
            $crate::services::ClientService::execute_request_with_json::<_>,
            $method,
            $endpoint,
            $payload
        )
    };
}

#[macro_export]
macro_rules! __api_dispatch {
    ($exec:path, $method:ident, $endpoint:expr, payload = $payload:expr, on_success = $on_success:expr $(, on_error = $on_error:expr)? $(,)?) => {{
        let payload = Some($payload);
        let on_success_cb = $on_success;
        #[allow(unused_variables)]
        let on_error_cb = $crate::services::api_error_handler;
        $(let on_error_cb = $on_error;)?
        $exec($crate::services::Method::$method, $endpoint, payload, on_success_cb, on_error_cb)
    }};
    ($exec:path, $method:ident, $endpoint:expr, on_success = $on_success:expr $(, on_error = $on_error:expr)? $(,)?) => {{
        let payload = None::<()>;
        let on_success_cb = $on_success;
        #[allow(unused_variables)]
        let on_error_cb = $crate::services::api_error_handler;
        $(let on_error_cb = $on_error;)?
        $exec($crate::services::Method::$method, $endpoint, payload, on_success_cb, on_error_cb)
    }};
}

#[macro_export]
macro_rules! api_call {
    ($method:ident, $endpoint:expr, on_success = $on_success:expr $(, on_error = $on_error:expr)? $(,)?) => {
        $crate::__api_dispatch!(
            $crate::services::ClientService::execute_request_with_callbacks,
            $method, $endpoint,
            on_success = $on_success
            $(, on_error = $on_error)?
        )
    };
    ($method:ident, $endpoint:expr, on_error = $on_error:expr $(,)?) => {
        $crate::__api_dispatch!(
            $crate::services::ClientService::execute_request_with_callbacks,
            $method, $endpoint,
            on_success = || (),
            on_error = $on_error
        )
    };
    ($method:ident, $endpoint:expr, $payload:expr, on_success = $on_success:expr $(, on_error = $on_error:expr)? $(,)?) => {
        $crate::__api_dispatch!(
            $crate::services::ClientService::execute_request_with_callbacks,
            $method, $endpoint,
            payload = $payload,
            on_success = $on_success
            $(, on_error = $on_error)?
        )
    };
    ($method:ident, $endpoint:expr, $payload:expr, on_error = $on_error:expr $(,)?) => {
        $crate::__api_dispatch!(
            $crate::services::ClientService::execute_request_with_callbacks,
            $method, $endpoint,
            payload = $payload,
            on_success = || (),
            on_error = $on_error
        )
    };

    ($method:ident, $endpoint:expr $(,)?) => {
        $crate::__api_dispatch!(
            $crate::services::ClientService::execute_request_with_callbacks,
            $method, $endpoint,
            on_success = || ()
        )
    };
    ($method:ident, $endpoint:expr, $payload:expr $(,)?) => {
        $crate::__api_dispatch!(
            $crate::services::ClientService::execute_request_with_callbacks,
            $method, $endpoint,
            payload = $payload,
            on_success = || ()
        )
    };
}

#[macro_export]
macro_rules! api_fetch {
    ($method:ident, $endpoint:expr, on_success = $on_success:expr $(, on_error = $on_error:expr)? $(,)?) => {
        $crate::__api_dispatch!(
            $crate::services::ClientService::execute_request_with_json_callbacks,
            $method, $endpoint,
            on_success = $on_success
            $(, on_error = $on_error)?
        )
    };
    ($method:ident, $endpoint:expr, on_error = $on_error:expr $(,)?) => {
        $crate::__api_dispatch!(
            $crate::services::ClientService::execute_request_with_json_callbacks,
            $method, $endpoint,
            on_success = |_| (),
            on_error = $on_error
        )
    };
    ($method:ident, $endpoint:expr, $payload:expr, on_success = $on_success:expr $(, on_error = $on_error:expr)? $(,)?) => {
        $crate::__api_dispatch!(
            $crate::services::ClientService::execute_request_with_json_callbacks,
            $method, $endpoint,
            payload = $payload,
            on_success = $on_success
            $(, on_error = $on_error)?
        )
    };
    ($method:ident, $endpoint:expr, $payload:expr, on_error = $on_error:expr $(,)?) => {
        $crate::__api_dispatch!(
            $crate::services::ClientService::execute_request_with_json_callbacks,
            $method, $endpoint,
            payload = $payload,
            on_success = |_| (),
            on_error = $on_error
        )
    };

    ($method:ident, $endpoint:expr $(,)?) => {
        $crate::__api_dispatch!(
            $crate::services::ClientService::execute_request_with_json_callbacks,
            $method, $endpoint,
            on_success = |_| ()
        )
    };
    ($method:ident, $endpoint:expr, $payload:expr $(,)?) => {
        $crate::__api_dispatch!(
            $crate::services::ClientService::execute_request_with_json_callbacks,
            $method, $endpoint,
            payload = $payload,
            on_success = |_| ()
        )
    };
}
