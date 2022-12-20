use std::net::SocketAddr;

use clap::{ArgEnum, Parser};
use client::run_client;
use hydroflow::tokio;
use hydroflow::util::{bind_udp_bytes, ipv4_resolve};
use server::run_server;

mod client;
mod helpers;
mod protocol;
mod server;

#[derive(Clone, ArgEnum, Debug)]
enum Role {
    Client,
    Server,
}
#[derive(Clone, ArgEnum, Debug)]
enum GraphType {
    Mermaid,
    Dot,
    Json,
}

#[derive(Parser, Debug)]
struct Opts {
    #[clap(arg_enum, long)]
    role: Role,
    #[clap(long, value_parser = ipv4_resolve)]
    addr: SocketAddr,
    #[clap(long, value_parser = ipv4_resolve)]
    server_addr: Option<SocketAddr>,
    #[clap(arg_enum, long)]
    graph: Option<GraphType>,
}

#[tokio::main]
async fn main() {
    let opts = Opts::parse();

    match opts.role {
        Role::Client => {
            let (outbound, inbound, bound_addr) = bind_udp_bytes(opts.addr).await;
            println!("Client is bound to {}", bound_addr);
            println!(
                "Attempting to connect to server at {}",
                opts.server_addr.unwrap()
            );
            let server_addr = opts.server_addr.unwrap();
            run_client(outbound, inbound, server_addr, opts.graph).await;
        }
        Role::Server => {
            let (outbound, inbound, bound_addr) = bind_udp_bytes(opts.addr).await;
            println!("Listening on {}", bound_addr);
            run_server(outbound, inbound, opts.graph).await;
        }
    }
}
