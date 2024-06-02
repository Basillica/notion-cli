use clap::Subcommand;

pub mod init;
pub mod conf;

pub mod block;
pub mod comment;
pub mod database;
pub mod page;
pub mod search;
pub mod users;


#[derive(Subcommand)]
pub enum ApiCommand {
    #[command(subcommand)]
    Block(block::BlockCommand),
    #[command(subcommand)]
    Comment(comment::CommentCommand),
    #[command(subcommand)]
    Database(database::DatabaseCommand),
    #[command(subcommand)]
    Page(page::PageCommand),
    #[command(subcommand)]
    Search(search::SearchCommand),
    #[command(subcommand)]
    Users(users::UserCommand),
}