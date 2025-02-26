use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    name: Option<String>,
    version: Option<String>,
    about: Option<String>,
    long_about: Option<String>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: RunMode,
}

#[derive(Subcommand)]
enum RunMode {
    #[clap(name = "config")]
    Config {
        #[arg(short, long)]
        set_server: String,
    },

    #[clap(name = "proc-define")]
    ProcDefine,

    #[clap(name = "proc-commit")]
    ProcCommit{
        name: String,
        #[arg(short, long)]
        filename: PathBuf,
    },

    #[clap(name = "role-subscription")]
    RoleSubscription, // interactive TUI mode

    #[clap(name = "role-subrequest")]
    RoleSubrequest {
        filename: PathBuf,
        keyfilename: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        RunMode::Config { set_server    } => {
            println!("Config: {:?}", set_server);
        }
        RunMode::ProcDefine => {
            println!("ProcDefine");
        }
        RunMode::ProcCommit{ name, filename} => {
            println!("ProcCommit: {:?} {:?}", name, filename);
        }
        RunMode::RoleSubscription => {
            println!("RoleSubscription");
        }
        RunMode::RoleSubrequest{ filename, keyfilename} => {
            println!("RoleSubrequest: {:?} {:?}", filename, keyfilename);
        }
    }

}