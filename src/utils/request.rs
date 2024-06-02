pub mod http_client {
    use serde::{Deserialize, Serialize};

    const BASE_URL: &str = "https://api.notion.com";

    #[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct ErrorMessage {
        pub object: String,
        pub status: i64,
        pub code: String,
        pub message: String,
        #[serde(rename = "developer_survey")]
        pub developer_survey: String,
        #[serde(rename = "request_id")]
        pub request_id: String,
    }

    #[derive(Debug)]
    pub struct ApiConfig {
        pub version: String,
        pub entity: String,
        pub token: String,
    }
    
    pub async fn post(cfg: ApiConfig, data: impl Serialize) -> Result<(), ureq::Error> {
        let url = format!("{}/{}/{}", BASE_URL, cfg.version, cfg.entity);
        let token = format!("Bearer {}", cfg.token);

        match ureq::post(&url)
            .set("Authorization", &token)
            .set("Content-Type", "application/json")
            .set("Notion-Version", "2022-02-22")
            .send_json(data)?
            .into_string() {
                Ok(v) => println!("user: {:?}", v),
                Err(e) => println!("user: {:?}", e),
            };

        Ok(())
    }

    pub async fn get(cfg: ApiConfig) -> Result<(), ureq::Error> {
        let url = format!("{}/{}/{}", BASE_URL, cfg.version, cfg.entity);
        let token = format!("Bearer {}", cfg.token);
        println!("{url}");

        let body: String = ureq::get(&url)
            .set("Authorization", &token)
            .set("Notion-Version", "2022-02-22")
            .call()?
            .into_string()?;

        println!("user: {}", body);
        Ok(())
    }

    pub async fn patch(cfg: ApiConfig, data: impl Serialize) -> Result<(), ureq::Error> {
        let url = format!("{}/{}/{}", BASE_URL, cfg.version, cfg.entity);
        let token = format!("Bearer {}", cfg.token);

        match ureq::patch(&url)
            .set("Authorization", &token)
            .set("Content-Type", "application/json")
            .set("Notion-Version", "2022-02-22")
            .send_json(data)?
            // .into_json::<QueryResult>() {
            .into_string() {
                Ok(v) => println!("user: {:?}", v),
                Err(e) => println!("user: {:?}", e),
            };

        Ok(())
    }

    pub fn tentative() {}
}