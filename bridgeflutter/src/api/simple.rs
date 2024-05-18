
use anyhow;
use kaspa_rpc_core::api::rpc;
use std::str::FromStr;
use futures::executor::block_on;
use kaspa_bip32::Prefix;
use kaspa_consensus_core::network::{NetworkId, NetworkType};
use tokio::runtime::Runtime;
pub use std::sync::Arc;

use kaspa_rpc_core::GetServerInfoResponse;

pub use kaspa_wrpc_client::{
    client::{ConnectOptions, ConnectStrategy},
    KaspaRpcClient, WrpcEncoding, Resolver
};

// Connect to a Kaspa node via wRPC and returns server info.
async fn node_connect() -> anyhow::Result<String,()>{

    let mut queue = Arc::new("".to_string());

    let rt = Runtime::new().unwrap();
    let result = rt.block_on(async {
        println!("try connecting to node {}", "ws://192.168.11.11:17110");

        // Enter a node IP address in the local network if needed.
        let url = Some("ws://192.168.11.11:17110".to_string());

        let resolver = Some(Resolver::default());

        let mut collect_info : Vec<String> = vec![];
        if let Ok(client) = KaspaRpcClient::new_with_args(
            WrpcEncoding::Borsh, url.as_deref(), resolver,
            Some(NetworkId::from_str("mainnet").unwrap()), None){

            let rpc_client = Arc::new(client);
            let options = ConnectOptions { block_async_connect: true, ..Default::default() };

            if rpc_client.connect(Some(options.clone())).await.is_ok(){

                println!("...connected");

                if rpc_client.rpc_api().ping().await.is_ok() {
                    println!("Ping ok!");
                } else {
                    println!("Ping error");
                }

                let GetServerInfoResponse {
                    server_version,
                    network_id,
                    has_utxo_index,
                    is_synced,
                    virtual_daa_score,
                    rpc_api_version,
                } = rpc_client.rpc_api().get_server_info().await.unwrap();

                let rpc_version = rpc_api_version.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(".");

                println!("Version {}", server_version);
                println!("Network {}", network_id);
                println!("UTXO index {}", has_utxo_index);
                println!("Synced {}", is_synced);
                println!("DAA score {}", virtual_daa_score);
                println!("RPC API version {}", rpc_version);

                collect_info.push(format!("Version: {}", server_version));
                collect_info.push(format!("Network: {}", network_id));
                collect_info.push(format!("UTXO index: {}", has_utxo_index));
                collect_info.push(format!("Synced: {}", is_synced));
                collect_info.push(format!("DAA score: {}", virtual_daa_score));
                collect_info.push(format!("RPC API version: {}", rpc_version));
            }
        }
        queue = Arc::new(collect_info.join("\r\n"));
    });

    let message = format!("Returned from tokio runtime WRPC call: {queue}");
    println!("{message}");

    Ok(message)
}

#[flutter_rust_bridge::frb(sync)]
pub fn greet(_name: String) -> String {
    let mut message: String = "".to_string();

    // First test using Kaspa components and printing network data,
    block_on(async {

        let mut convention: Vec<String> = vec![];
        convention.push("Kaspa!\r\nLocal rust info:\r\n".to_string());
        for i in NetworkId::iter() {
            let rpc_port = NetworkType::from(i).default_rpc_port();
            let pair = format!("{} → {} - RPC port: {}", i, Prefix::from(i), rpc_port);
            convention.push(pair); 
        }

        // Second test connecting to a Kaspa node.
        if let Ok(server_info) = node_connect().await {
            convention.push(format!("Node connection successful!\r\n{server_info}")); 
        }
        message = format!("Hello, {}", convention.join("\r\n"));
    });
    message
}


#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    // Default utilities - feel free to customize
    flutter_rust_bridge::setup_default_user_utils();
}
