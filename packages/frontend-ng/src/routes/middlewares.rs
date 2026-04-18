use axum_gate::gate::{Gate, bearer};
use axum_gate::gate::bearer::BearerGate;
use axum_gate::groups::Group;
use axum_gate::prelude::{Account,  JsonWebToken, JwtClaims};
use axum_gate::roles::Role;
use tower::Layer;
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_service::Service;
use tower_http::trace::{self, TraceLayer};
use tracing::{Level};
use tuono_lib::axum::Router;
use tuono_lib::axum::{
     http::StatusCode,
};
use std::pin::Pin;
use std::task::{Poll, Context};
use std::fmt;
use std::error::Error;

#[tuono_lib::middleware]
pub fn root_tracing_middleware() ->TraceLayer<SharedClassifier<ServerErrorsAsFailures>>
where TraceLayer<SharedClassifier<ServerErrorsAsFailures>>: Clone+Send+Sync+'static {
 TraceLayer::new_for_http().make_span_with(trace::DefaultMakeSpan::new()
                    .level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new()
                    .level(Level::INFO))
}


use crate::tuono_main_state::app::STATE;
use crate::tuono_main_state::jwt::get_jwt;
use crate::tuono_main_state::state::AppState;
// use crate::tuono_main_state::tuono_state::{AppState};

// #[derive(Clone,Debug)]
// pub struct AuthLayer {
//     state: AppState,
// }




// impl<S> Layer<S> for AuthService<S>
// where S:  Service<tuono_lib::axum::extract::Request, Error = S> + Clone + Send + Sync + 'static,
// S::Response: IntoResponse + 'static,
// T::Future: Send + 'static{
//     type Service = Route<S>;

//     fn layer(&self, service: S) -> BoxCloneSyncService<AuthService<S>>::Service {
//         new_route::<S,Infallible>(BoxCloneSyncService::new(AuthService::<S>{service,state::default()}))
//     }
// }



//  impl<S> FromRef<AuthService<S>> for AppState      
//    where
//      AuthService<S>: Clone +Send +Sync,
//      S: Clone +Send + Sync,
//      {
//      fn from_ref(layer: &AuthService<S>) -> Self {
//           let a = layer.state.clone();
//           a
//      }
//  }

//  impl<S> FromRequestParts<S> for AuthService<AuthLayer>
//         where
//      AppState: FromRef<S>+Clone +Send +Sync,
//      S: HasState+ Send + Sync +Clone,
//         {
//             type Rejection = std::convert::Infallible;
//     async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//          // get a `AppState` from a reference to the state
//          println!("In FromRequestParts AuthService");
//          let state = AppState::from_ref(state);
//          dbg!(&state);
//         Ok(Self{state: state.clone(), service:AuthLayer{state:state.clone()}})
//     }
// }
// #[async_trait]
//  impl<S> FromRequestParts<S> for AuthLayer
//         where
//      Arc<AppState>: FromRef<S>,
//      S: Send + Sync +Clone,
//         {
//             type Rejection = std::convert::Infallible;
//     async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//          // get a `AppState` from a reference to the state
//          println!("In FromRequestParts AuthLayer");
//          let state = Arc::from_ref(state);
//          dbg!(&state);

//         Ok(Self{state: state.clone(),})
//     }
// }


// This service implements the Log behavior
// #[derive(Clone,Debug)]
// pub struct AuthService {
//     layer: AuthLayer,
//     state: AppState
// }



// impl<R> Service<R> for AuthService
// where
//     R: Sync+ Send+ 'static,
// {
//     type Response = S::Response;
//     type Error = S::Error;
//     type Future=  Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
//     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         self.service.poll_ready(cx)
//     }

//     fn call(&mut self, request: R) -> Self::Future {
//         // do something with state.routes!
        
//         // Insert log statement here or other functionality
//         println!("AuthService call");
//         dbg!(&self.state);
//         let clone = self.service.clone();
//         // take the service that was ready
//         let mut inner = std::mem::replace(&mut self.service, clone);
//         let gate_clone = self.state.routes.clone().unwrap_or(Router::new());
//         let inner_gate = (std::mem::replace(&mut self.state.routes, Some(gate_clone))).unwrap();
//         dbg!(&inner_gate);
//         Box::pin(async move {
//             println!("SENDING IT TO THE GATE!");
//              let res = Router::new().merge(inner_gate).into_make_service().call(&request).await;
//              dbg!(&res);
//              println!("Sending it down the line...");
//              inner.call(request).await
//         })
//     }
// }

// impl IntoResponse for AuthService {
//     fn into_response(self) -> axum::response::Response {
//         let (status, message) = match self {
//             ApiError::JsonExtractorRejection(json_rejection) => {
//                 (json_rejection.status(), json_rejection.body_text())
//             }
//         };

//         let payload = json!({
//             "message": message,
//             "origin": "with_rejection"
//         });

//         (status, Json(payload)).into_response()
//     }
// }




#[derive(Clone,Debug)]
pub struct Wrap<S> 
where S: Clone + Send +Sync +'static{
    inner: S,
}

impl<S> Wrap<S> 
where S: Clone +Send + Sync+ 'static{
    pub const fn new(inner: S) -> Wrap<S> {
        Wrap {
            inner
        }
    }
}

// // The error returned if processing a request timed out
#[derive(Debug,Clone)]
pub struct Expired;

impl fmt::Display for Expired {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "expired")
    }
}

impl Error for Expired {}

impl<S, R> Service<R> for Wrap<S>
where
     S: Service<R> + Clone +Sync+Send+ 'static,
    R: Sync+ Send+ 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future=  Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // Our timeout service is ready if the inner service is ready.
        // This is how backpressure can be propagated through a tree of nested services.
       self.inner.poll_ready(cx).map_err(Into::into)
    }

    fn call(&mut self, req: R) -> Self::Future {
        
        // let state = ApiState::from_ref();
        
        // Create a future that completes after `self.timeout`
        // let timeout = tokio::time::sleep(self.timeout);

        // Call the inner service and get a future that resolves to the response
        // let fut self.original_service.call(req);
                let clone = self.inner.clone();
        // take the service that was ready
        let mut inner = std::mem::replace(&mut self.inner, clone);
        Box::pin(async move {
             inner.call(req).await
        })

        // Wrap those two futures in another future that completes when either one completes
        //
        // If the inner service is too slow the `sleep` future will complete first
        // And an error will be returned and `fut` will be dropped and not polled again
        //
        // We have to box the errors so the types match

    }
}





#[derive(Clone,Debug)]
pub struct ServiceWrapperLayer;

impl ServiceWrapperLayer {
    pub const fn new() -> Self {
        ServiceWrapperLayer {
            
        }
    }
}

impl<S> Layer<S> for ServiceWrapperLayer
    where S: Clone +Send +Sync +'static{
    type Service = Wrap<S>;

    fn layer(&self, service: S) -> Self::Service {
        Wrap::new(service)
    }
}

// pub struct RoutesAndHandlers {
//     routes:HashMap<String,MethodRouter>
// }
// pub struct OAuth2HandlerState<R, G>
// where
//     R: AccessHierarchy + Eq + Display + Send + Sync + 'static,
//     G: Eq + Clone + Send + Sync + 'static,
// {
//     // Raw OAuth2 config; client is constructed in handlers
//     auth_url: String,
//     token_url: String,
//     client_id: String,
//     client_secret: Option<String>,
//     redirect_url: String,
//     scopes: Vec<String>,

//     state_cookie_template: CookieTemplate,
//     pkce_cookie_template: CookieTemplate,

//     // Session issuance
//     auth_cookie_template: CookieTemplate,
//     post_login_redirect: Option<String>,
//     mapper: Option<AccountMapperFn<R, G>>,
//     account_inserter: Option<AccountPersistFn<R, G>>,
//     jwt_encoder: Option<AccountEncoderFn<R, G>>,
// }



// /// Type alias for an account encoding function.
// type AccountEncoderFn<R, G> = Arc<dyn Fn(Account<R, G>) -> OAuth2Result<String> + Send + Sync>;
// /// Type alias for an account mapper function.
// type AccountMapperFn<R, G> = Arc<
//     dyn for<'a> Fn(
//             &'a StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>,
//         )
//             -> Pin<Box<dyn Future<Output = OAuth2Result<Account<R, G>>> + Send + 'a>>
//         + Send
//         + Sync,
// >;
// /// Type alias for an async account persistence function invoked before JWT issuance.
// ///
// /// This closure should persist or load the account (idempotently), and return the account
// /// that should be encoded into the first‑party JWT (typically with a stable `account_id`).
// type AccountPersistFn<R, G> = Arc<
//     dyn Fn(Account<R, G>) -> Pin<Box<dyn Future<Output = OAuth2Result<Account<R, G>>> + Send>>
//         + Send
//         + Sync,
// >;
// pub trait PubLayersAndRoutes<R, G>   where
//     R: AccessHierarchy + Eq + Display + Send + Sync + 'static,
//     G: Eq + Clone + Send + Sync + 'static,
//     Role: AccessHierarchy + Eq + Display + Send + Sync + 'static,
//     Group:Eq + Clone + Send + Sync + 'static, 
//  {
     
//     // Required method
//       fn get_layer(&self, base: &str ) -> impl Layer<OAuth2HandlerState<R,G>>;
//       fn get_routes_and_handler(&self) -> RoutesAndHandlers;
//       async fn callback_handler(st: Extension<Arc<OAuth2HandlerState<Role, Group>>>,
//     jar: CookieJar,
//     q: Query<CallbackQuery>,
// ) -> impl IntoResponse;
// async fn login_handler(
//     st: Extension<Arc<OAuth2HandlerState<Role, Group>>>,
//     jar: CookieJar,
// ) -> impl IntoResponse;
// }

// /// Query parameters delivered by the provider to the redirect/callback endpoint.
// #[derive(Deserialize, Debug)]
// struct CallbackQuery {
//     code: Option<String>,
//     state: Option<String>,
//     error: Option<String>,
//     error_description: Option<String>,
// }


// impl<R, G> PubLayersAndRoutes<R,G> for OAuth2Gate<R, G>
// where
//     R: AccessHierarchy + Eq + Display + Send + Sync + 'static,
//     G: Eq + Clone + Send + Sync + 'static,
//     Role: AccessHierarchy + Eq + Display + Send + Sync + 'static,
//     Group:Eq + Clone + Send + Sync + 'static, 
// {
//     pub fn get_layer(&self, base: &str ) -> impl Layer<OAuth2HandlerState<R,G>> {

//         // Validate presence of required config and store raw values in handler state
//         let auth_url = self
//             .auth_url
//             .clone()
//             .ok_or_else(|| OAuth2Error::missing("auth_url"))?;
//         let token_url = self
//             .token_url
//             .clone()
//             .ok_or_else(|| OAuth2Error::missing("token_url"))?;
//         let client_id = self
//             .client_id
//             .clone()
//             .ok_or_else(|| OAuth2Error::missing("client_id"))?;
//         let redirect_url = self
//             .redirect_url
//             .clone()
//             .ok_or_else(|| OAuth2Error::missing("redirect_url"))?;

//         // Validate cookie templates to prevent insecure SameSite=None + Secure=false, etc.
//         self.state_cookie_template
//             .validate()
//             .map_err(|e| OAuth2Error::cookie_invalid(OAuth2CookieKind::State, e.to_string()))?;
//         self.pkce_cookie_template
//             .validate()
//             .map_err(|e| OAuth2Error::cookie_invalid(OAuth2CookieKind::Pkce, e.to_string()))?;
//         self.auth_cookie_template
//             .validate()
//             .map_err(|e| OAuth2Error::cookie_invalid(OAuth2CookieKind::Auth, e.to_string()))?;

//         let handler_state = Arc::new(OAuth2HandlerState::<R, G> {
//             auth_url,
//             token_url,
//             client_id,
//             client_secret: self.client_secret.clone(),
//             redirect_url,
//             scopes: self.scopes.clone(),
//             state_cookie_template: self.state_cookie_template.clone(),
//             pkce_cookie_template: self.pkce_cookie_template.clone(),
//             auth_cookie_template: self.auth_cookie_template.clone(),
//             post_login_redirect: self.post_login_redirect.clone(),
//             mapper: self.mapper.clone(),
//             account_inserter: self.account_inserter.clone(),
//             jwt_encoder: self.jwt_encoder.clone(),
//         });
//         Extension(handler_state)
//     }


//     pub fn get_routes_and_handler(&self) -> RoutesAndHandlers {
//                 let login_path = format!("{base}/login");
//         let callback_path = format!("{base}/callback");

//         RoutesAndHandlers{
//             routes: {&login_path: login_handler::<R, G> ,
//             &callback_path: callback_handler::<R, G>}
//         }
//     }

    
// /// Generates PKCE/state cookies and redirects to the provider's authorization endpoint.
// async fn login_handler(
//     Extension(st): Extension<Arc<OAuth2HandlerState<Role, Group>>>,
//     jar: CookieJar,
// ) -> impl IntoResponse
// where
//     Role: AccessHierarchy + Eq + Display + Send + Sync + 'static,
//     Group: Eq + Clone + Send + Sync + 'static,
// {
//     let auth_url = match AuthUrl::new(st.auth_url.clone()) {
//         Ok(u) => u,
//         Err(e) => {
//             {
//                 let err = self::errors::OAuth2Error::invalid_url("auth_url", e.to_string());
//                 error!(
//                     "{}",
//                     crate::errors::UserFriendlyError::developer_message(&err)
//                 );
//             }
//             return (StatusCode::INTERNAL_SERVER_ERROR, "OAuth2 misconfigured").into_response();
//         }
//     };
//     let token_url = match TokenUrl::new(st.token_url.clone()) {
//         Ok(u) => u,
//         Err(e) => {
//             {
//                 let err = self::errors::OAuth2Error::invalid_url("token_url", e.to_string());
//                 error!(
//                     "{}",
//                     crate::errors::UserFriendlyError::developer_message(&err)
//                 );
//             }
//             return (StatusCode::INTERNAL_SERVER_ERROR, "OAuth2 misconfigured").into_response();
//         }
//     };
//     let redirect_url = match RedirectUrl::new(st.redirect_url.clone()) {
//         Ok(u) => u,
//         Err(e) => {
//             {
//                 let err = self::errors::OAuth2Error::invalid_url("redirect_url", e.to_string());
//                 error!(
//                     "{}",
//                     crate::errors::UserFriendlyError::developer_message(&err)
//                 );
//             }
//             return (StatusCode::INTERNAL_SERVER_ERROR, "OAuth2 misconfigured").into_response();
//         }
//     };
//     let mut client = BasicClient::new(ClientId::new(st.client_id.clone()))
//         .set_auth_uri(auth_url)
//         .set_token_uri(token_url)
//         .set_redirect_uri(redirect_url);
//     if let Some(secret) = &st.client_secret {
//         client = client.set_client_secret(ClientSecret::new(secret.clone()));
//     }

//     // CSRF state
//     let csrf = CsrfToken::new_random();
//     // PKCE challenge + verifier
//     let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

//     let mut req = client
//         .authorize_url(|| csrf.clone())
//         .set_pkce_challenge(pkce_challenge);

//     for s in &st.scopes {
//         req = req.add_scope(Scope::new(s.clone()));
//     }

//     let (auth_url, csrf_token) = req.url();

//     // Prepare cookies using configured templates (short-lived by default)
//     let state_cookie = st
//         .state_cookie_template
//         .build_with_value(csrf_token.secret());

//     let pkce_cookie = st
//         .pkce_cookie_template
//         .build_with_value(pkce_verifier.secret());

//     let jar = jar.add(state_cookie).add(pkce_cookie);

//     (jar, Redirect::to(auth_url.as_str())).into_response()
// }

// /// Validates state and PKCE, exchanges code for tokens, optionally mints a first-party JWT,
// /// installs auth cookie, clears ephemeral cookies, and redirects (if configured).
// pub async fn callback_handler(
//     Extension(st): Extension<Arc<OAuth2HandlerState<Role, Group>>>,
//     jar: CookieJar,
//     Query(q): Query<CallbackQuery>,
// ) -> impl IntoResponse
// where
//     Role: AccessHierarchy + Eq + std::fmt::Display + Send + Sync + 'static,
//     Group: Eq + Clone + Send + Sync + 'static,
// {
//     // Load state + pkce verifier from cookies
//     let state_cookie = jar.get(st.state_cookie_template.cookie_name_ref());
//     let pkce_cookie = jar.get(st.pkce_cookie_template.cookie_name_ref());

//     let Some(state_cookie) = state_cookie else {
//         error!("Missing state cookie");
//         let state_removal = st.state_cookie_template.build_removal();
//         let pkce_removal = st.pkce_cookie_template.build_removal();
//         let jar = jar.add(state_removal).add(pkce_removal);
//         return (jar, (StatusCode::BAD_REQUEST, "Missing state")).into_response();
//     };

//     let Some(pkce_cookie) = pkce_cookie else {
//         error!("Missing PKCE cookie");
//         let state_removal = st.state_cookie_template.build_removal();
//         let pkce_removal = st.pkce_cookie_template.build_removal();
//         let jar = jar.add(state_removal).add(pkce_removal);
//         return (jar, (StatusCode::BAD_REQUEST, "Missing PKCE")).into_response();
//     };

//     // If provider returned an error, clear cookies and return a safe error.
//     if let Some(err) = q.error.as_deref() {
//         let state_removal = st.state_cookie_template.build_removal();
//         let pkce_removal = st.pkce_cookie_template.build_removal();
//         let jar = jar.add(state_removal).add(pkce_removal);
//         error!(
//             "OAuth2 provider returned error: {err} {:?}",
//             q.error_description.as_deref()
//         );
//         return (
//             jar,
//             (StatusCode::BAD_REQUEST, "OAuth2 authorization failed"),
//         )
//             .into_response();
//     }

//     // Compare state from query and cookie; require state param
//     match q.state.as_deref() {
//         Some(state) if state_cookie.value() == state => {}
//         _ => {
//             error!("State missing or mismatch");
//             let state_removal = st.state_cookie_template.build_removal();
//             let pkce_removal = st.pkce_cookie_template.build_removal();
//             let jar = jar.add(state_removal).add(pkce_removal);
//             return (
//                 jar,
//                 (StatusCode::BAD_REQUEST, "OAuth2 authorization failed"),
//             )
//                 .into_response();
//         }
//     }

//     let Some(code_str) = q.code.clone() else {
//         let state_removal = st.state_cookie_template.build_removal();
//         let pkce_removal = st.pkce_cookie_template.build_removal();
//         let jar = jar.add(state_removal).add(pkce_removal);
//         return (
//             jar,
//             (StatusCode::BAD_REQUEST, "OAuth2 authorization failed"),
//         )
//             .into_response();
//     };
//     let code = AuthorizationCode::new(code_str);
//     let pkce_verifier = PkceCodeVerifier::new(pkce_cookie.value().to_string());

//     // Exchange code for tokens
//     let auth_url = match AuthUrl::new(st.auth_url.clone()) {
//         Ok(u) => u,
//         Err(e) => {
//             {
//                 let err = self::errors::OAuth2Error::invalid_url("auth_url", e.to_string());
//                 error!(
//                     "{}",
//                     crate::errors::UserFriendlyError::developer_message(&err)
//                 );
//             }
//             let state_removal = st.state_cookie_template.build_removal();
//             let pkce_removal = st.pkce_cookie_template.build_removal();
//             let jar = jar.add(state_removal).add(pkce_removal);
//             return (
//                 jar,
//                 (StatusCode::INTERNAL_SERVER_ERROR, "OAuth2 misconfigured"),
//             )
//                 .into_response();
//         }
//     };
//     let token_url = match TokenUrl::new(st.token_url.clone()) {
//         Ok(u) => u,
//         Err(e) => {
//             {
//                 let err = self::errors::OAuth2Error::invalid_url("token_url", e.to_string());
//                 error!(
//                     "{}",
//                     crate::errors::UserFriendlyError::developer_message(&err)
//                 );
//             }
//             let state_removal = st.state_cookie_template.build_removal();
//             let pkce_removal = st.pkce_cookie_template.build_removal();
//             let jar = jar.add(state_removal).add(pkce_removal);
//             return (
//                 jar,
//                 (StatusCode::INTERNAL_SERVER_ERROR, "OAuth2 misconfigured"),
//             )
//                 .into_response();
//         }
//     };
//     let redirect_url = match RedirectUrl::new(st.redirect_url.clone()) {
//         Ok(u) => u,
//         Err(e) => {
//             {
//                 let err = self::errors::OAuth2Error::invalid_url("redirect_url", e.to_string());
//                 error!(
//                     "{}",
//                     crate::errors::UserFriendlyError::developer_message(&err)
//                 );
//             }
//             let state_removal = st.state_cookie_template.build_removal();
//             let pkce_removal = st.pkce_cookie_template.build_removal();
//             let jar = jar.add(state_removal).add(pkce_removal);
//             return (
//                 jar,
//                 (StatusCode::INTERNAL_SERVER_ERROR, "OAuth2 misconfigured"),
//             )
//                 .into_response();
//         }
//     };
//     let mut client = BasicClient::new(ClientId::new(st.client_id.clone()))
//         .set_auth_uri(auth_url)
//         .set_token_uri(token_url)
//         .set_redirect_uri(redirect_url);
//     if let Some(secret) = &st.client_secret {
//         client = client.set_client_secret(ClientSecret::new(secret.clone()));
//     }

//     match client
//         .exchange_code(code)
//         .set_pkce_verifier(pkce_verifier)
//         .request_async(&|req: oauth2::HttpRequest| async move {
//             let client = reqwest::Client::builder()
//                 .timeout(std::time::Duration::from_secs(10))
//                 .build()?;
//             let url = req.uri().to_string();
//             let builder = client.request(req.method().clone(), url);
//             let resp = builder
//                 .headers(req.headers().clone())
//                 .body(req.body().clone())
//                 .send()
//                 .await?;
//             let status = resp.status();
//             let headers = resp.headers().clone();
//             let body = resp.bytes().await?.to_vec();
//             let mut resp_out = http::Response::new(body);
//             *resp_out.status_mut() = status;
//             *resp_out.headers_mut() = headers;
//             Ok::<http::Response<Vec<u8>>, reqwest::Error>(resp_out)
//         })
//         .await
//     {
//         Ok(token_resp) => {
//             debug!(
//                 "OAuth2 token response received (scopes: {:?})",
//                 token_resp.scopes()
//             );

//             // Clear ephemeral cookies (state/pkce) using configured templates
//             let state_removal = st.state_cookie_template.build_removal();
//             let pkce_removal = st.pkce_cookie_template.build_removal();

//             let mut jar = jar.add(state_removal).add(pkce_removal);

//             // Try session issuance if configured
//             if let (Some(mapper), Some(jwt_encoder)) = (&st.mapper, &st.jwt_encoder) {
//                 // 1) Map provider tokens -> Account<R, G>
//                 match (mapper)(&token_resp).await {
//                     Ok(mapped_account) => {
//                         // 2) Optionally persist/load account before JWT issuance (to get stable account_id)
//                         let account_result = if let Some(inserter) = &st.account_inserter {
//                             (inserter)(mapped_account).await
//                         } else {
//                             Ok(mapped_account)
//                         };

//                         // 3) Encode JWT using the (possibly persisted) account
//                         match account_result.and_then(|account| jwt_encoder(account)) {
//                             Ok(token) => {
//                                 // Prepare auth cookie using template flags
//                                 let auth_cookie = st.auth_cookie_template.build_with_value(&token);

//                                 jar = jar.add(auth_cookie);

//                                 if let Some(url) = &st.post_login_redirect {
//                                     return (jar, Redirect::to(url)).into_response();
//                                 } else {
//                                     return (jar, (StatusCode::OK, "OAuth2 login OK"))
//                                         .into_response();
//                                 }
//                             }
//                             Err(e) => {
//                                 error!(
//                                     "OAuth2 session issuance failed [{}]: {}",
//                                     crate::errors::UserFriendlyError::support_code(&e),
//                                     crate::errors::UserFriendlyError::developer_message(&e),
//                                 );
//                                 return (
//                                     jar,
//                                     (StatusCode::BAD_GATEWAY, "OAuth2 session issuance failed"),
//                                 )
//                                     .into_response();
//                             }
//                         }
//                     }
//                     Err(e) => {
//                         error!(
//                             "OAuth2 account mapping failed [{}]: {}",
//                             crate::errors::UserFriendlyError::support_code(&e),
//                             crate::errors::UserFriendlyError::developer_message(&e),
//                         );
//                         return (
//                             jar,
//                             (StatusCode::BAD_GATEWAY, "OAuth2 account mapping failed"),
//                         )
//                             .into_response();
//                     }
//                 }
//             }

//             // If no session issuance configured, return OK
//             (jar, (StatusCode::OK, "OAuth2 callback OK")).into_response()
//         }
//         Err(err) => {
//             let oe = self::errors::OAuth2Error::token_exchange(err.to_string());
//             error!(
//                 "OAuth2 token exchange failed [{}]: {}",
//                 crate::errors::UserFriendlyError::support_code(&oe),
//                 crate::errors::UserFriendlyError::developer_message(&oe),
//             );
//             let state_removal = st.state_cookie_template.build_removal();
//             let pkce_removal = st.pkce_cookie_template.build_removal();
//             let jar = jar.add(state_removal).add(pkce_removal);
//             (
//                 jar,
//                 (StatusCode::BAD_GATEWAY, "OAuth2 token exchange failed"),
//             )
//                 .into_response()
//         }
//     }
// }
// }

// #[tuono_lib::middleware]
// pub fn root_auth_middleware<R>(oauth2_gate: Option<OAuth2Gate<Role,Group>>) -> Router {
//     let Some(gate) = oauth2_gate else {panic!("no oauth gate");};
//     let Ok(gate_router) = gate.routes("/auth") else {panic!("NO OAUTH ROUTES");};
//     dbg!(&gate_router);
//     let router_svc = gate_router;
//     dbg!(&router_svc);
//     // let svc = ServiceBuilder::new().service(router_svc);
//     let inner = &router_svc;
//     dbg!(&inner);
//     ServiceBuilder.new().service(router_svc).into_inner()
// }

async fn handle_anyhow_error(err: anyhow::Error) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Something went wrong: {err}")
    )
}
/*

// #[tuono_lib::middleware]
pub fn root_gate_extractor_routes_middleware() ->ServiceBuilder<Stack<LayerFn<IntoMakeService<Router>>, tower_layer::Identity>>
{

    // let Some(gate_routes) = routes else { panic!("No gate routes");};
    // gate_routes
    // let state = Arc::new(state);
    println!("in middleware setup fn.");
    // dbg!(state);
    // from_extractor_with_state::<AuthService<AuthLayer>,AppState>(state)
    let svc = Router::<()>::new().nest("/", STATE.clone().routes).into_make_service();
     ServiceBuilder::new().layer(layer_fn(svc))
}
*/

#[tuono_lib::middleware] 
pub fn root_gate_optional_anon_routes_middleware() -> BearerGate<JsonWebToken<JwtClaims<Account<Role, Group>>>, Role, Group, bearer::JwtConfig<Role,Group>>
{
 Gate::bearer::<JsonWebToken<JwtClaims<Account<Role, Group>>>, Role, Group>("Crustchan", get_jwt()).allow_anonymous_with_optional_user()
}

// #[tuono_lib::middleware]
// pub fn root_gate_routes_middleware() -> Stack<LayerFn<ServiceFn<Router>>, Identity>

// {

//     // let Some(gate_routes) = routes else { panic!("No gate routes");};
//     // gate_routes
//     // let state = Arc::new(state);
//     // let state = 
//     println!("in middleware gates setup fn.");
    
//     // let layr = STATE.clone().oauth2_gate.get_layer("/auth");
//     // let layr = ServiceBuilder::new().layer(layer_fn(svc)).into_inner();
//     layr
//     // Router::new().nes
//     // layer_fn(svc)
//     // dbg!(state);
//     // Route::<AuthService>{service:AuthLayer{state:state.clone()},state:state.clone()}
// }


pub fn get_router() -> Router {
  return STATE.router.clone();
}