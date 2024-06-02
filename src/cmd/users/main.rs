pub mod user_api {
    use crate::{cmd::users::UserArgs, utils::{config::AppConfig, request::{self, http_client::ApiConfig}}};
    use async_std::task;

    pub fn retrieve_user(_cmd: &UserArgs, cfg: &mut AppConfig) {
        let cfg: ApiConfig = ApiConfig{
            version: "v1".into(),
            entity: "users/".into(),
            token: cfg.notion_api_key.clone(),
        };
        match task::block_on(request::http_client::get(cfg)) {
            Ok(_) => println!("Success"),
            Err(e) => println!("Error: {}", e)
        };
    }

    pub fn list_all_users(_cmd: &UserArgs, cfg: &mut AppConfig) {
        let cfg: ApiConfig = ApiConfig{
            version: "v1".into(),
            entity: "users".into(),
            token: cfg.notion_api_key.clone(),
        };
        match task::block_on(request::http_client::get(cfg)) {
            Ok(_) => println!("Success"),
            Err(e) => println!("Error: {}", e)
        };
    }

    pub fn retrieve_token_bot_user(_cmd: &UserArgs, cfg: &mut AppConfig) {
        let cfg: ApiConfig = ApiConfig{
            version: "v1".into(),
            entity: "users/me".into(),
            token: cfg.notion_api_key.clone(),
        };
        match task::block_on(request::http_client::get(cfg)) {
            Ok(_) => println!("Success"),
            Err(e) => println!("Error: {}", e)
        };
    }
}
