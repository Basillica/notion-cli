use std::collections::HashMap;

use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};

pub mod main;

#[derive(Subcommand)]
pub enum DatabaseCommand {
    Get(DatabaseArgs),
    Query(DatabaseArgs),
    Sort(DatabaseArgs),
    Filter(DatabaseArgs),
    Create(DatabaseArgs),
    // Update(DatabaseArgs),
    PropUpdate(DatabaseArgs),
}

/////////////////////////////////
/// Database main args
#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub(crate) struct DatabaseArgs {
    #[arg(short = 'i', help = "The id of a database")]
    database_id: String,
    #[arg(short = 'p', help = "A database property for query")]
    property: Option<String>,
    #[arg(short = 'v', help = "The value of the property")]
    value: Option<String>,
    ////////////
    #[arg(short = 'K', help = "A database property for query")]
    attr_key: Option<String>,
    #[arg(short = 'V', help = "The value of the property")]
    attr_val: Option<String>,
}

/////////////////////////////////
#[derive(Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Query {
    pub filter: Filter,
    pub sorts: Option<Vec<Sort>>,
}

#[derive(Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
    pub property: String,
    pub select: Select,
}

#[derive(Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Select {
    pub equals: String,
}

#[derive(Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sort {
    pub property: String,
    pub direction: String,
}

#[derive(Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FilterRoot {
    pub filter: Filter,
}

#[derive(Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SortRoot {
    pub sorts: Vec<Sort>,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateRoot {
    pub properties: HashMap<String, HashMap<String, String>>,
}
