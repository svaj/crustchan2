use std::{
    convert::Infallible,
    task::{Context, Poll},
};

use futures::future::BoxFuture;
use tuono_lib::axum::{
    Router,
    body::Body,
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::{self, FromFnLayer, Next},
    response::{IntoResponse, Response},
    routing::{Route, get},
};

use chrono::{NaiveDateTime, Utc};
use tower::{
    Layer, Service, ServiceBuilder,
    layer::util::{Identity, Stack},
};

use axum_oidc_client::auth_session::AuthSession;
#[path = "../../app.rs"]
mod tuono_main_state;

#[derive(Clone)]
struct AdminLayer;
impl AdminLayer {
    pub fn new() -> Self {}
}

impl<S> Layer<S> for AdminLayer {
    type Service = AdminMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        AdminMiddleware { inner }
    }
}

#[derive(Clone, Debug)]
struct AdminMiddleware<S> {
    inner: S,
}

impl<S> Service<Request> for AdminMiddleware<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let clone = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, clone);
        Box::pin(async move {
            let Some(session) = request.extensions().get::<AuthSession>() else {
                let mut res = Response::default();
                *res.status_mut() = http::StatusCode::UNAUTHORIZED;
                return Ok(res);
            };
            match (&session.scope) {
                Some(scope) if (scope.contains("admin")) && token_is_valid(&session) => {
                    let future = inner.call(request);
                    // Box::pin(async move {
                    let response: Response = future.await?;
                    Ok(response)
                    // })
                }
                e => {
                    let mut res = Response::default();
                    *res.status_mut() = http::StatusCode::UNAUTHORIZED;
                    return Ok(res);
                }
            }
        })
    }
}

async fn admin_check(
    session: AuthSession,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    match (&session.scope) {
        Some(scope) if (scope.contains("admin")) && token_is_valid(&session) => {
            let response = next.run(request).await;
            Ok(response)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

fn token_is_valid(session: &AuthSession) -> bool {
    let Some(expires) = session.expires.map(|e| e.to_string()) else {
        return false;
    };
    // compare time
    let now = Utc::now().naive_utc();
    if NaiveDateTime::parse_from_str(expires.as_str(), &"%Y-%m-%dT%H:%M:%S")
        .expect("invalid expiry time")
        < now
    {
        return false;
    }
    return true;
}

#[tuono_lib::middleware]
pub fn admin_middleware<F, T>() -> AdminLayer {
    AdminLayer::new()
}
