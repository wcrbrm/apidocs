pub mod cli;
pub mod endpoints;
pub mod logging;
pub mod svc;

use std::net::SocketAddr;
use tracing::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    color_eyre::install().unwrap();
    logging::start();

    let args = cli::parse();
    debug!("args: {:#?}", args);

    match args.action {
        cli::Action::OpenApi => {
            let openapi = endpoints::openapi::openapi();
            println!("{}", serde_json::to_string_pretty(&openapi).unwrap());
        }
        cli::Action::Config { action } => match &action {
            cli::ConfigAction::List => {
                let config = svc::Config::from_path(&args.storage_path);
                println!("{}", serde_json::to_string_pretty(&config).unwrap());
            }
            cli::ConfigAction::Add {
                id,
                title,
                description,
            } => {
                let mut config = svc::Config::from_path(&args.storage_path);
                config.add(svc::Entry::new(
                    id.clone(),
                    title.clone(),
                    description.clone(),
                ));
                config.save(&args.storage_path)?;
            }
            cli::ConfigAction::Remove { id } => {
                let mut config = svc::Config::from_path(&args.storage_path);
                config.remove(id);
                config.save(&args.storage_path)?;
            }
            cli::ConfigAction::Show { id } => {
                let config = svc::Config::from_path(&args.storage_path);
                let entry = config.get(id).unwrap();
                println!("{}", serde_json::to_string_pretty(&entry).unwrap());
            }
            cli::ConfigAction::AddSection {
                id,
                section,
                title,
                url,
                // TODO: use local file
            } => {
                let mut config = svc::Config::from_path(&args.storage_path);
                let entry = config.get_mut(id).unwrap();
                let section = svc::DocSection {
                    section: section.clone(),
                    title: title.clone(),
                    link: svc::DocSectionLink::from_url_opt(url),
                };
                entry.add_section(section);
                config.save(&args.storage_path)?;
            }
            cli::ConfigAction::RemoveSection { id, section } => {
                let mut config = svc::Config::from_path(&args.storage_path);
                let entry = config.get_mut(id).unwrap();
                entry.remove_section(section);
                config.save(&args.storage_path)?;
            }
        },
        cli::Action::Server {
            listen,
            assets_path,

            header,
            footer,
            secret_token,
        } => {
            let socket_addr: SocketAddr = listen.parse().unwrap();
            endpoints::run(
                socket_addr,
                args.storage_path,
                assets_path,
                header,
                footer,
                secret_token,
            )
            .await
            .unwrap();
        }
    }

    Ok(())
}
