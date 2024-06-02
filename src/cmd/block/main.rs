pub mod block_api {
    use crate::{cmd::block::BlockArgs, utils::{config::AppConfig, request}};

    pub fn append_child(cmd: &BlockArgs, cfg: &mut AppConfig) {
        println!("{:?} and {:?}", cmd, cfg);
        request::http_client::tentative()
    }

    pub fn delete(cmd: &BlockArgs, cfg: &mut AppConfig) {
        println!("{:?} and {:?}", cmd, cfg);
        request::http_client::tentative()
    }

    pub fn get(cmd: &BlockArgs, cfg: &mut AppConfig) {
        println!("{:?} and {:?}", cmd, cfg);
        request::http_client::tentative()
    }

    pub fn get_kids(cmd: &BlockArgs, cfg: &mut AppConfig) {
        println!("{:?} and {:?}", cmd, cfg);
        request::http_client::tentative()
    }

    pub fn update(cmd: &BlockArgs, cfg: &mut AppConfig) {
        println!("{:?} and {:?}", cmd, cfg);
        request::http_client::tentative()
    }
}