use std::path::PathBuf;

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate_to, Shell};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {

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

    #[clap(name = "completion")]
    Completion {
        shell: Option<Shell>,
        out_dir: Option<PathBuf>,
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
        RunMode::Completion{shell: Some(shell), out_dir} => {
            let mut cmd = Cli::command();
            let out_dir = if let Some(d) = out_dir {
                d
            } else {
                &PathBuf::from(".")
            };
            match shell {
                clap_complete::Shell::Bash => {
                    generate_to(Shell::Bash, &mut cmd, "nopuma-cli", out_dir).unwrap();
                }
                clap_complete::Shell::Fish => {
                    generate_to(Shell::Fish, &mut cmd, "nopuma-cli", out_dir).unwrap();
                }
                clap_complete::Shell::Zsh => {
                    generate_to(Shell::Zsh, &mut cmd, "nopuma-cli", out_dir).unwrap();
                }
                clap_complete::Shell::PowerShell => {
                    generate_to(Shell::PowerShell, &mut cmd, "nopuma-cli", out_dir).unwrap();
                }
                clap_complete::Shell::Elvish => {
                    generate_to(Shell::Elvish, &mut cmd, "nopuma-cli", out_dir).unwrap();
                },
                &_ => {
                    println!("Unsupported shell");
                }
            }
        },
        RunMode::Completion{shell: None, out_dir: _} => {
            println!("shell not provided");
        }
    }
}
