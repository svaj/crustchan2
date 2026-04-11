use std::env;

#[derive(Clone)]
pub struct ApplicationState {
    pub website_name: String,
    pub base_path: String,
    // pub session: AuthSession,
}

pub fn main() -> ApplicationState {
    dotenvy::dotenv().ok();
    let website_name = "Crustchan".to_string();
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let base_path = format!("https://{host}");
    // let session = AuthSession{ id_token: "".to_string(), access_token: "".to_string(), token_type: "".to_string(), refresh_token: None, scope: None, expires: None };
    ApplicationState {
        website_name,
        base_path,
        // session,
    }
}
