use anyhow::{Ok, Result};
use clap::Args;
use utils::{config::AppConfig, config::Builder};

use crate::utils;


#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub(crate) struct InitArgs {
    /// the value of the notion api key
    #[arg(short = 'k', help = "The notion api key")]
    notion_api_key: String,
}

pub(crate) fn main(args: &InitArgs, cfg: &mut AppConfig) -> Result<()> {
    if args.notion_api_key == ""{
        println!("the *notion_api_key* must be provided");
        println!("Run the *help* command for help\n");
        return Ok(());
    }
    let mut conf = AppConfig::new();
    conf
        .set_app_name("notion-cli")
        .set_app_conf_path("notion-cli/config")
        .set_notion_api_key(&args.notion_api_key)
        .build();

    cfg.update(&conf);
    Ok(())
}