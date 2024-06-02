use clap::{Args, Subcommand};

pub mod main;


#[derive(Subcommand)]
pub enum SearchCommand {
    Query(SearchArgs),
}

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub(crate) struct SearchArgs {
    /// the query for the search
    #[arg(short = 'q', help = "The query string")]
    query: String,
    /// the sql database name
    #[arg(short = 'd', default_value = "ascending", help = "The direction of the sorting")]
    direction: String,
    /// the sql database username
    #[arg(default_value = "last_edited_time", short = 't', help = "The default timestamp for sorting")]
    timestamp: String,
}