pub mod remotecli;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct ServerOptions {
    #[structopt(long, default_value = "127.0.0.1:8000")]
    pub server_listen_addr: String,
}

#[derive(StructOpt, Debug)]
pub struct RemoteCommandOptions {
    #[structopt(long = "server", default_value = "http://127.0.0.1:8000")]
    pub server_addr: String,
    pub command: Vec<String>,
}

#[derive(StructOpt, Debug)]
pub enum SubCommand {
    #[structopt(name = "server")]
    StartServer(ServerOptions),
    #[structopt(setting = structopt::clap::AppSettings::TrailingVarArg)]
    Run(RemoteCommandOptions),
}

#[derive(StructOpt, Debug)]
#[structopt(name = "remotecli")]
struct ApplicationArguments {
    #[structopt(flatten)]
    pub subcommand: SubCommand,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = ApplicationArguments::from_args();

    match args.subcommand {
        SubCommand::StartServer(opts) => {
            println!("Start the server on: {:?}", opts.server_listen_addr);
            remotecli::server::start_server(opts).await?;
        }
        SubCommand::Run(rc_opts) => {
            println!("Run command: '{:?}'", rc_opts.command);
            // remotecli::client::client_run(opts).await?;
        }
    }

    Ok(())
}
