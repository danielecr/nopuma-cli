use std::{io::Write, path::PathBuf};

use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::{generate_to, Shell};
use serde::{Deserialize, Serialize};
use serde_json;

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

    #[clap(name = "config-show")]
    ConfigShow,
    
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

#[derive(Serialize, Deserialize)]
struct Config {
    server: String,
}

impl Config {
    fn new() -> Self {
        Config {
            server: "https://localhost:8080".to_string(),
        }
    }

    fn config_writer(&mut self, set_server: &str) {
        let config = Config {
            server: set_server.to_string(),
        };
        // get the home directory from the environment
        let home = std::env::var("HOME").unwrap();
        // create the directory startinif it doesn't exist
        std::fs::create_dir_all(format!("{}/.nopuma", &home)).unwrap();
        let mut file = std::fs::File::create(".nopuma/config.json").unwrap();
        file.write(serde_json::to_string(&config).unwrap().as_bytes()).unwrap();
    }

    fn config_reader() -> Self {
        let home = std::env::var("HOME").unwrap();
        match std::fs::File::open(format!("{}/.nopuma/config.json", &home)) {
            Ok(file) => {
                let config: Config = serde_json::from_reader(file).unwrap();
                config
            }
            Err(_) => {
                let config = Config::new();
                let mut file = std::fs::File::create(format!("{}/.nopuma/config.json", &home)).unwrap();
                file.write(serde_json::to_string(&config).unwrap().as_bytes()).unwrap();
                config
            }
        }
    }

}

fn main() {
    let cli = Cli::parse();
    let mut config = Config::config_reader();
    
    match &cli.command {
        RunMode::Config { set_server    } => {
            println!("Config: {:?}, in config file .nopuma/config.yaml", set_server);
            config.config_writer(set_server);
        }
        RunMode::ConfigShow => {
            println!("Config: {:?}", config.server);
        }
        RunMode::ProcDefine => {
            println!("ProcDefine Interactive TUI");
            // This gui allow the user to define a process
            // by adding a step (node) linked to the selected node
            // the selected node is rendered with double border
            // the user can add a step by pressing the 'a' key
            // the user can delete a step by pressing the 'd' key
            // Each step has a name, a description, a role, and a list of roles
            // also each step has an cardinal number identifier
            // the user can save the process definition to a json file
            // the user can commit the process definition to the server

        }
        RunMode::ProcCommit{ name, filename} => {
            println!("Write the json file for the process definition");
            println!("ProcCommit: {:?} {:?}", name, filename);
        }
        RunMode::RoleSubscription => {
            println!("RoleSubscription Interactive TUI");
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
