pub mod comment_api {
    use crate::{cmd::comment::CommentArgs, utils::{config::AppConfig, request}};

    pub fn add_to_discussion(cmd: &CommentArgs, cfg: &mut AppConfig) {
        println!("{:?} and {:?}", cmd, cfg);
        request::http_client::tentative()
    }

    pub fn create(cmd: &CommentArgs, cfg: &mut AppConfig) {
        println!("{:?} and {:?}", cmd, cfg);
        request::http_client::tentative()
    }

    pub fn get(cmd: &CommentArgs, cfg: &mut AppConfig) {
        println!("{:?} and {:?}", cmd, cfg);
        request::http_client::tentative()
    }
}