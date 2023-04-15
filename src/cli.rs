use clap::Parser; // ValueEnum

#[derive(Debug, Clone, clap::Subcommand)]
pub enum ConfigAction {
    /// list all services
    List,
    /// show a service
    Show {
        /// ID of the service
        #[clap(long)]
        id: String,
    },
    /// add a new service
    Add {
        /// ID of the service
        #[clap(long)]
        id: String,
        /// Title of the service
        #[clap(long)]
        title: String,
        /// Description of the service
        #[clap(long)]
        description: String,
    },
    /// add section to a service
    AddSection {
        /// ID of the service
        #[clap(long)]
        id: String,
        /// section name
        #[clap(long)]
        section: String,
        /// title of the section
        #[clap(long)]
        title: String,
        /// url of the external link
        #[clap(long)]
        url: Option<String>,
    },
    /// remove section from a service
    RemoveSection {
        /// ID of the service
        #[clap(long)]
        id: String,
        /// section name
        #[clap(long)]
        section: String,
    },
    /// remove a service
    Remove {
        /// ID of the service
        #[clap(long)]
        id: String,
    },
}

#[derive(Debug, Clone, clap::Subcommand)]
pub enum Action {
    /// Generate OpenAPI documentation
    OpenApi,
    /// Check installation
    Server {
        /// Net listening address of HTTP server in case of "server" command
        #[clap(long, default_value = "0.0.0.0:8000", env = "LISTEN")]
        listen: String,
        /// folder to serve static files from
        #[clap(long, default_value = "./assets", env = "ASSETS_PATH")]
        assets_path: String,

        /// secret token to manage records using HTTP API
        #[clap(long, default_value = "", env = "SECRET_TOKEN")]
        secret_token: String,
        /// header title
        #[clap(long, default_value = "API DOCS", env = "HEADER")]
        header: String,
        /// footer title
        #[clap(long, default_value = "(c) 2023", env = "FOOTER")]
        footer: String,
    },
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

// struct for clap CLI args
#[derive(Debug, Parser)]
#[clap(version = "0.1")]
pub struct Opts {
    /// Writable storage folder
    #[clap(long, default_value = "./data", env = "STORAGE_PATH")]
    pub storage_path: String,
    /// Action
    #[command(subcommand)]
    pub action: Action,
    /// Log Level
    #[clap(env = "RUST_LOG", default_value = "info")]
    rust_log: Option<String>,
}

pub fn parse() -> Opts {
    Opts::parse()
}
