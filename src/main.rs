use anyhow::Result;
use clap::{Parser, Subcommand};
use cmd::{
    block::{BlockCommand, main::block_api}, 
    comment::{CommentCommand, main::comment_api}, 
    database::{DatabaseCommand, main::database_api}, 
    page::{PageCommand, main::page_api}, 
    search::{SearchCommand, main::search_api}, 
    users::{UserCommand, main::user_api}, ApiCommand
};

mod cmd;
mod utils;


/// notion cli containing utility functions for performing operation on Myserv app
#[derive(Parser)]
#[command(version, about, long_about = None )]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// The init command for initializing the application. This set's up the config file
    /// The further information: notion-cli init -h
    /// Eg:
    ///     notion-cli init -s database-server -n database-name -u database-user -p database-password
    Init(cmd::init::InitArgs),
    /// The info command shows the content of the config file and its file path.
    /// The further information: notion-cli info -h
    /// Eg:
    ///     notion-cli info
    Info(cmd::conf::ConfArgs),
    ///
    #[command(subcommand)]
    Api(cmd::ApiCommand),
}


fn main() -> Result<()> {
    env_logger::init();
    let mut cfg: utils::config::AppConfig = confy::load("notion-cli", None)?;
    println!("Notion cli v0.0.1 \n");
    let cli = Cli::parse();
    match &cli.command {
        Commands::Init(args) => {
            cmd::init::main(args, &mut cfg)?;
        },
        Commands::Info(args) => {
            cmd::conf::main(args, &mut cfg)?;
        },
        Commands::Api(args) => {
            match args {
                ApiCommand::Block(args) => {
                    match args {
                        BlockCommand::AppendChild(args) => {
                            block_api::append_child(args, &mut cfg)
                        },
                        BlockCommand::Delete(args) => {
                            block_api::delete(args, &mut cfg)
                        },
                        BlockCommand::Get(args) => {
                            block_api::get(args, &mut cfg)
                        },
                        BlockCommand::GetKids(args) => {
                            block_api::get_kids(args, &mut cfg)
                        },
                        BlockCommand::Update(args) => {
                            block_api::update(args, &mut cfg)
                        }
                    }
                },
                ApiCommand::Comment(args) => {
                    match args {
                        CommentCommand::AddToDiscussion(args) => {
                            comment_api::add_to_discussion(args, &mut cfg)
                        },
                        CommentCommand::Create(args) => {
                            comment_api::create(args, &mut cfg)
                        },
                        CommentCommand::Get(args) => {
                            comment_api::get(args, &mut cfg)
                        }
                    }
                },
                ApiCommand::Database(args) => {
                    match args {
                        DatabaseCommand::Create(args) => {
                            database_api::create(args, &mut cfg)
                        },
                        DatabaseCommand::Filter(args) => {
                            database_api::filter(args, &mut cfg)
                        },
                        DatabaseCommand::Get(args) => {
                            database_api::get(args, &mut cfg)
                        },
                        DatabaseCommand::PropUpdate(args) => {
                            database_api::update_prop(args, &mut cfg)
                        },
                        DatabaseCommand::Query(args) => {
                            database_api::query(args, &mut cfg)
                        },
                        DatabaseCommand::Sort(args) => {
                            database_api::sort(args, &mut cfg)
                        }
                        // DatabaseCommand::Update(args) => {
                        //     database_api::update(args, &mut cfg)
                        // }
                    }
                },
                ApiCommand::Page(args) => {
                    match args {
                        PageCommand::Archive(args) => {
                            page_api::archive(args, &mut cfg)
                        },
                        PageCommand::Create(args) => {
                            page_api::create(args, &mut cfg)
                        },
                        PageCommand::CreateWithContent(args) => {
                            page_api::create_with_content(args, &mut cfg)
                        },
                        PageCommand::Get(args) => {
                            page_api::get(args, &mut cfg)
                        },
                        PageCommand::GetProp(args) => {
                            page_api::get_props(args, &mut cfg)
                        },
                        PageCommand::UpdateProps(args) => {
                            page_api::update_props(args, &mut cfg)
                        }
                    }
                },
                ApiCommand::Search(args) => {
                    match args {
                        SearchCommand::Query(args) => {
                            search_api::query(args, &mut cfg)
                        }
                    }
                },
                ApiCommand::Users(args) => {
                    match args {
                        UserCommand::GetUser(args) => {
                            user_api::retrieve_user(args, &mut cfg)
                        },
                        UserCommand::GetUsers(args) => {
                            user_api::list_all_users(args, &mut cfg)
                        },
                        UserCommand::TokenBotUser(args) => {
                            user_api::retrieve_token_bot_user(args, &mut cfg)
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

// curl --location 'https://api.notion.com/v1/users/' --header 'Notion-Version: 2022-02-22' --header 'Authorization: Bearer secret_WKX2xc9s9xiH0mrwb4V5Xep4U7BtJzaDwrnPcKiIKNr'