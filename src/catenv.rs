mod utils;
use clap::{Parser, ValueEnum};
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(name = "catenv")]
pub struct Cli {
    #[arg(value_name = "key")]
    key: String,
    #[arg(
        short = 's',
        long = "scope",
        value_enum,
        default_value_t = Scope::Process
    )]
    scope: Scope,
}

#[derive(Clone, ValueEnum, Debug)]
pub enum Scope {
    Process,
    User,
    System,
}

fn main() -> ExitCode {
    let args = Cli::parse();
    match args.scope {
        Scope::Process => {
            let res = match utils::get_process_env(&args.key) {
                Ok(e) => e,
                Err(_) => return ExitCode::FAILURE
            };
            print!("{}", res);
            return ExitCode::SUCCESS;
        },
        Scope::User => {
            let res = match utils::get_user_env(&args.key) {
                Ok(e) => e,
                Err(_) => return ExitCode::FAILURE
            };
            print!("{}", res);
            return ExitCode::SUCCESS;
        },
        Scope::System => {
            let res = match utils::get_system_env(&args.key) {
                Ok(e) => e,
                Err(_) => return ExitCode::FAILURE
            };
            print!("{}", res);
            return ExitCode::SUCCESS;
        }
        
    }
}