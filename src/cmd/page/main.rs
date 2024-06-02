pub mod page_api {
    use crate::{cmd::page::PageArgs, utils::{config::AppConfig, request}};

    pub fn archive(cmd: &PageArgs, cfg: &mut AppConfig) {
        println!("{:?} and {:?}", cmd, cfg);
        request::http_client::tentative()
    }

    pub fn create(cmd: &PageArgs, cfg: &mut AppConfig) {
        println!("{:?} and {:?}", cmd, cfg);
        request::http_client::tentative()
    }

    pub fn create_with_content(cmd: &PageArgs, cfg: &mut AppConfig) {
        println!("{:?} and {:?}", cmd, cfg);
        request::http_client::tentative()
    }

    pub fn get(cmd: &PageArgs, cfg: &mut AppConfig) {
        println!("{:?} and {:?}", cmd, cfg);
        request::http_client::tentative()
    }

    pub fn get_props(cmd: &PageArgs, cfg: &mut AppConfig) {
        println!("{:?} and {:?}", cmd, cfg);
        request::http_client::tentative()
    }

    pub fn update_props(cmd: &PageArgs, cfg: &mut AppConfig) {
        println!("{:?} and {:?}", cmd, cfg);
        request::http_client::tentative()
    }
}