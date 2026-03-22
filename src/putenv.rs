mod utils;
use clap::{Parser, ValueEnum};
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(name = "putenv")]
pub struct Cli {
    #[arg(value_name = "key")]
    key: String,
    #[arg(value_name = "value")]
    value: String,
    #[arg(
        short = 's',
        long = "scope",
        value_enum,
        default_value_t = Scope::User
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
            if !utils::set_process_env_safe(&args.key, &args.value) { return ExitCode::FAILURE; }
            return ExitCode::SUCCESS;
        },
        Scope::User => {
            if !utils::set_user_env_safe(&args.key, &args.value) { return ExitCode::FAILURE; }

            #[cfg(windows)]
            { utils::notify_system(); }

            return ExitCode::SUCCESS;
        },
        Scope::System => {
            if !utils::set_system_env_safe(&args.key, &args.value) { return ExitCode::FAILURE; }

            #[cfg(windows)]
            { utils::notify_system(); }

            return ExitCode::SUCCESS;
        }
    }
}