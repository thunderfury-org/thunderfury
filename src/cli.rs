use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start thunderfury server
    Server(ConfigDirArgs),

    /// Apply database changes, used for develop only
    #[command(hide = true)]
    Migrate(ConfigDirArgs),
}

#[derive(Args)]
pub struct ConfigDirArgs {
    /// Server config directory
    #[arg(short, long, default_value_t = String::from("./config"))]
    pub config_dir: String,
}

#[cfg(test)]
mod tests {
    use super::Cli;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Cli::command().debug_assert()
    }
}
