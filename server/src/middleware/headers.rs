use ::axum::{
    body::Body,
    http::{HeaderName, HeaderValue, Request, header::*},
    response::Response,
};
use ::futures::future::BoxFuture;
use ::std::task;
use ::tower::{Layer, Service};

#[derive(Clone)]
pub struct StaticHeadersLayer {
    cache_control: HeaderValue,
    keep_alive: HeaderValue,
}

impl StaticHeadersLayer {
    pub fn new(cache_control: &'static str, keep_alive: &'static str) -> Self {
        Self {
            cache_control: HeaderValue::from_static(cache_control),
            keep_alive: HeaderValue::from_static(keep_alive),
        }
    }
}

impl<S> Layer<S> for StaticHeadersLayer {
    type Service = StaticHeadersMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        StaticHeadersMiddleware {
            inner,
            cache_control: self.cache_control.clone(),
            keep_alive: self.keep_alive.clone(),
        }
    }
}

#[derive(Clone)]
pub struct StaticHeadersMiddleware<S> {
    inner: S,
    cache_control: HeaderValue,
    keep_alive: HeaderValue,
}

impl<S> Service<Request<Body>> for StaticHeadersMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Into<axum::BoxError>,
{
    type Response = Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Response, S::Error>>;

    fn poll_ready(&mut self, cx: &mut task::Context<'_>) -> task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let mut svc = self.inner.clone();
        let cache_control = self.cache_control.clone();
        let keep_alive = self.keep_alive.clone();

        Box::pin(async move {
            let mut res = svc.call(req).await?;
            let headers = res.headers_mut();

            headers.insert(CACHE_CONTROL, cache_control);
            headers.insert(CONNECTION, HeaderValue::from_static("keep-alive"));
            headers.insert(HeaderName::from_static("keep-alive"), keep_alive);
            headers.insert(PRAGMA, HeaderValue::from_static("no-cache"));

            Ok(res)
        })
    }
}
