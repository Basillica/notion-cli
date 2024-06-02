use clap::{Args, Subcommand};

pub mod main;

#[derive(Subcommand)]
pub enum UserCommand {
    GetUser(UserArgs),
    GetUsers(UserArgs),
    TokenBotUser(UserArgs),
}


#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub(crate) struct UserArgs {
    // /// the value of the database server host
    // /// -s myserv-sqlsrv-prd.database.windows.net -n myserv-sqldb-prd -p Vrbsaq6Eg4lwJfWVDrVzqu79dSowFCsnL9lK
    // #[arg(short = 's', help = "The sql server host name")]
    // sql_db_host: String,
    // /// the sql database name
    // #[arg(short = 'n', help = "The sql server database name")]
    // sql_db_name: String,
    // /// the sql database username
    // #[arg(default_value = "dbadmin", short = 'u', help = "The sql database username")]
    // sql_db_username: String,
    // /// the sql database password
    // #[arg(short = 'p', help = "The sql database password")]
    // sql_db_password: String,
}