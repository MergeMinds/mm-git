use crate::context;

use clap::{Parser, Subcommand};
use git2::Repository;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("git error: {0}")]
    Init(#[from] git2::Error),
    #[error("error while removing: {0}")]
    Remove(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Parser)]
struct ShellCli {
    #[arg(short)]
    command: String,
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(external_subcommand)]
    External(Vec<String>),
}

#[derive(Clone, Debug)]
pub struct RepoManager {
    repos_root: PathBuf,
}

impl RepoManager {
    pub fn new(repos_root: PathBuf) -> Self {
        Self { repos_root }
    }

    pub fn init_repo(&self, name: &str) -> Result<git2::Repository> {
        let repo_path = self.repos_root.join(format!("{name}.git"));
        if repo_path.exists() {
            log::debug!("Repo '{}' already exists", repo_path.display());
            Err(Error::Init(git2::Error::from_str("already exists")))
        } else {
            log::debug!("Initialized repo '{}.git'", name);
            Repository::init_bare(repo_path).map_err(|e| Error::Init(e))
        }
    }

    pub fn remove_repo(&self, name: &str) -> Result<()> {
        let repo_path = self.repos_root.join(format!("{name}.git"));
        let repo_path = Path::new(&repo_path);
        if repo_path.exists() {
            log::debug!("Repo '{name}.git' removed");
            std::fs::remove_dir_all(repo_path)
                .map_err(|e| Error::Remove(e.to_string()))
        } else {
            log::debug!("Repo '{}' doesn't exist", repo_path.display());
            Err(Error::Remove("repo doesn't exist".to_string()))
        }
    }
}

pub fn shell(ctx: context::Context) {
    let program_name = std::env::args().next().unwrap();
    let cmdline = match ShellCli::try_parse() {
        Ok(shellcli) => {
            let mut line = shellcli
                .command
                .split(' ')
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            let mut newline = vec![program_name.clone()];
            newline.append(&mut line);
            newline
        }
        Err(_) => {
            Cli::parse_from([program_name, "--help".to_string()]);
            return;
        }
    };

    let cli = Cli::parse_from(cmdline);

    match cli.command {
        Commands::External(opts) => {
            // TODO(guschin): check acl before accepting
            if !["git-receive-pack", "git-upload-pack", "git-upload-archive"]
                .contains(&opts[0].as_str())
            {
                Cli::parse_from([program_name, "--help".to_string()]);
                return;
            }

            let args = opts[1..].iter().map(|s| s.trim_matches(['\'', '"']));
            Command::new(opts[0].clone()).args(args).exec();
        }
    }
}
