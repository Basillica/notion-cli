use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct AppConfig {
    app_name: String,
    default_path: PathBuf,
    pub notion_api_key: String,
}

impl ::std::default::Default for AppConfig {
    fn default() -> Self {
        let app_name: &'static str = env!("CARGO_PKG_NAME");
        Self {
            app_name: app_name.to_string(),
            default_path: "notion-cli/config".into(),
            notion_api_key: "".into(),
        }
    }
}

impl AppConfig{
    pub fn update(&mut self, cfg: &AppConfig) {
        match confy::store(&self.app_name, None, cfg){
            Ok(_) => println!("app config successfully updated"),
            Err(e) => println!("there was an error updating app config. error: {:?}", e),
        };
    }

    pub fn _config_path(&mut self) -> &PathBuf {
        &self.default_path
    }
}

pub trait Builder{
    fn new() -> Self;
    fn set_app_name(&mut self, app_name: &str) -> &mut Self;
    fn set_app_conf_path(&mut self, app_conf_path: &str) -> &mut Self;
    fn set_notion_api_key(&mut self, host: &str) -> &mut Self;
    fn build(&self) -> AppConfig;
}

impl Builder for AppConfig {
    fn new() -> Self {
        AppConfig {
            app_name: String::new(),
            default_path: "".into(),
            notion_api_key: String::new()
        }
    }

    fn set_app_name(&mut self, app_name: &str) -> &mut Self {
        self.app_name = app_name.to_owned();
        self
    }


    fn set_app_conf_path(&mut self, app_conf_path: &str) -> &mut Self {
        self.default_path = app_conf_path.into();
        self
    }


    fn set_notion_api_key(&mut self, api_key: &str) -> &mut Self {
        self.notion_api_key = api_key.to_owned();
        self
    }

    fn build(&self) -> AppConfig {
        AppConfig {
            app_name: self.app_name.clone(),
            default_path: self.default_path.clone(),
            notion_api_key: self.notion_api_key.clone(),
        }
    }
}