pub mod remotecli_proto { tonic::include_proto!("remotecli"); }
use remotecli_proto::remote_cli_client::RemoteCliClient;
use remotecli_proto::CommandInput;
use crate::RemoteCommandOptions;

pub async fn client_run(rc_opts: RemoteCommandOptions) 
-> Result<(), Box<dyn std::error::Error>> {
    let mut client = RemoteCliClient::connect(rc_opts.server_addr).await?;

    let request = tonic::Request::new(CommandInput{
        command: rc_opts.command[0].clone().into(),
        args: rc_opts.command[1..].to_vec(),
    });

    let response = client.shell(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
