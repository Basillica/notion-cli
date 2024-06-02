use anyhow::Result;
use clap::Args;
use utils::config::AppConfig;


use crate::utils;

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub(crate) struct ConfArgs {}

pub(crate) fn main(_args: &ConfArgs, cfg: &mut AppConfig) -> Result<()> {
    let full_path = confy::get_configuration_file_path("notion-cli", None)?;
    println!("config path: {:?}", full_path);
    println!("notion api key: {:?}", redact_all_but_last_6(&cfg.notion_api_key));
    Ok(())
}


fn redact_all_but_last_6(text: &str) -> String {
    let mut result = String::with_capacity(text.len());
    // Add asterisks for all characters except the last 6 (or less if the string is shorter)
    result.extend((0..std::cmp::max(text.len() - 6, 0)).map(|_| '*'));
    // Add the last 6 characters from the original string
    result.push_str(&text[std::cmp::max(text.len() - 6, 0)..]);
    result
}