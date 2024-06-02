pub mod database_api {
    use std::collections::HashMap;

    use crate::{cmd::database::{DatabaseArgs, Filter, FilterRoot, Query, Select, Sort, SortRoot, UpdateRoot}, utils::{config::AppConfig, request::{self, http_client::ApiConfig}}};
    use async_std::task;

    pub fn create(cmd: &DatabaseArgs, cfg: &mut AppConfig) {
        println!("{:?} and {:?}", cmd, cfg);
        request::http_client::tentative()
    }

    pub fn filter(cmd: &DatabaseArgs, cfg: &mut AppConfig) {
        let q = FilterRoot{
            filter: Filter{
                property: cmd.property.clone().unwrap(),
                select: Select{
                    equals: cmd.value.clone().unwrap(),
                }
            }
        };

        match serde_json::to_value(&q) {
            Ok(v) => {
                println!("{:?}", v);
                let cfg: ApiConfig = ApiConfig{
                    version: "v1".into(),
                    entity: format!("databases/{}/query", cmd.database_id),
                    token: cfg.notion_api_key.clone(),
                };
                match task::block_on(request::http_client::post(cfg, v)) {
                    Ok(_) => println!("Success"),
                    Err(e) => println!("Error: {}", e)
                };
            },
            Err(_) => println!()
        }
    }

    pub fn get(cmd: &DatabaseArgs, cfg: &mut AppConfig) {
        let cfg: ApiConfig = ApiConfig{
            version: "v1".into(),
            entity: format!("databases/{}", cmd.database_id),
            token: cfg.notion_api_key.clone(),
        };
        match task::block_on(request::http_client::get(cfg)) {
            Ok(_) => println!("Success"),
            Err(e) => println!("Error: {}", e)
        };
    }

    pub fn query(cmd: &DatabaseArgs, cfg: &mut AppConfig) {
        let q = Query{
            filter: Filter{
                property: cmd.property.clone().unwrap(),
                select: Select{
                    equals: cmd.value.clone().unwrap(),
                }
            },
            sorts: Some(vec![
                Sort{
                    property: cmd.property.clone().unwrap(),
                    direction: "ascending".into(),
                }
            ])
        };

        match serde_json::to_value(&q) {
            Ok(v) => {
                println!("{:?}", v);
                let cfg: ApiConfig = ApiConfig{
                    version: "v1".into(),
                    entity: format!("databases/{}/query", cmd.database_id),
                    token: cfg.notion_api_key.clone(),
                };
                match task::block_on(request::http_client::post(cfg, v)) {
                    Ok(_) => println!("Success"),
                    Err(e) => println!("Error: {}", e)
                };
            },
            Err(_) => println!()
        }
    }

    pub fn update_prop(cmd: &DatabaseArgs, cfg: &mut AppConfig) {
        let mut q = UpdateRoot::default();
        q.properties.insert(cmd.attr_key.clone().unwrap(), HashMap::from([
            ("name".to_string(), cmd.attr_val.clone().unwrap())
        ]));
        
        match serde_json::to_value(&q) {
            Ok(v) => {
                println!("{:?}", v);
                let cfg: ApiConfig = ApiConfig{
                    version: "v1".into(),
                    entity: format!("databases/{}", cmd.database_id),
                    token: cfg.notion_api_key.clone(),
                };
                match task::block_on(request::http_client::patch(cfg, v)) {
                    Ok(_) => println!("Success"),
                    Err(e) => println!("Error: {}", e)
                };
            },
            Err(_) => println!()
        }
    }

    pub fn sort(cmd: &DatabaseArgs, cfg: &mut AppConfig) {
        let q = SortRoot{
            sorts: vec![Sort{
                property: cmd.property.clone().unwrap(),
                direction: "ascending".into(),
            }]
        };

        match serde_json::to_value(&q) {
            Ok(v) => {
                println!("{:?}", v);
                let cfg: ApiConfig = ApiConfig{
                    version: "v1".into(),
                    entity: format!("databases/{}/query", cmd.database_id),
                    token: cfg.notion_api_key.clone(),
                };
                match task::block_on(request::http_client::post(cfg, v)) {
                    Ok(_) => println!("Success"),
                    Err(e) => println!("Error: {}", e)
                };
            },
            Err(_) => println!()
        }
    }

}
