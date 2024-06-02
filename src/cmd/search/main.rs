
pub mod search_api {
    use serde::Serialize;
    use async_std::task;
    use crate::{cmd::search::SearchArgs, utils::{config::AppConfig, request::{self, http_client::ApiConfig}}};

    #[derive(Debug, Serialize)]
    struct Sort<'a> {
        direction: &'a str,
        timestamp: &'a str,
    }

    #[derive(Debug, Serialize)]
    struct Query<'a> {
        query: &'a str,
        sort: &'a Sort<'a>
    }

    pub fn query(cmd: &SearchArgs, cfg: &mut AppConfig) {
        let q = Query{
            query: &cmd.query,
                sort: &Sort{
                    direction: &cmd.direction,
                    timestamp: &cmd.timestamp,
                }
            };
        match serde_json::to_value(&q) {
            Ok(v) => {
                let cfg: ApiConfig = ApiConfig{
                    version: "v1".into(),
                    entity: "search".into(),
                    token: cfg.notion_api_key.clone(),
                };
                match task::block_on(request::http_client::post(cfg, v)) {
                    Ok(_) => println!("Success"),
                    Err(e) => println!("Error: {}", e)
                };
            },
            Err(e) => println!("error: {:?}",e)
        };
        request::http_client::tentative()
    }
}