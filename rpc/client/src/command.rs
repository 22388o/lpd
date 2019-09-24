use super::lightning_address::LightningAddress;
use super::network_graph;

use std::io::Error as IoError;
use std::sync::Arc;

use dependencies::grpc::Error as GrpcError;
use dependencies::grpc::{Client, ClientStub};
use dependencies::httpbis::Error as HttpbisError;
use dependencies::futures::future::Future;
use structopt::StructOpt;

#[derive(Debug)]
pub enum Error {
    Grpc(GrpcError),
    Httpbis(HttpbisError),
    IoError{
        inner: IoError,
        description: String,
    },
}

impl Error {
    pub fn new_io_error(err: IoError, description: &str) -> Self {
        Error::IoError {
            inner: err,
            description: description.to_owned(),
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt()]
pub enum Command{
    /// Get general info
    #[structopt(name="get-info")]
    GetInfo,

    /// Connect to specified peer
    #[structopt(name="connect-peer")]
    ConnectPeer {
        #[structopt()]
        address: LightningAddress,
    },

    /// Report graph info
    #[structopt(name="describe-graph")]
    DescribeGraph,

    /// Report graph info in dot format
    #[structopt(name="describe-graph-dot")]
    DescribeGraphDot,

    /// Print version
    #[structopt(name="get-version")]
    GetVersion,

    /// List peers
    #[structopt(name="list-peers")]
    ListPeers,

    /// Generate a new p2wkh or np2wkh address
    #[structopt(name="new-address")]
    NewAddress {
        #[structopt(long = "address-type")]
        address_type: String,
    },

    #[structopt(name="get-utxo-list")]
    GetUtxoList,

    /// Compute and display the wallet's current balance
    #[structopt(name="get-wallet-balance")]
    GetWalletBalance,

    /// Send bitcoin on-chain to multiple addresses
    #[structopt(name="send-many")]
    SendMany {

    },

    /// Send bitcoin on-chain to an address
    #[structopt(name="send-coins")]
    SendCoins {
        #[structopt(long = "addr")]
        address: String,
        #[structopt(long = "amt")]
        amount: u64,
        #[structopt(long = "lock-coins")]
        lock_coins: bool,
        #[structopt(long = "submit")]
        submit: bool,
        #[structopt(long = "witness-only")]
        witness_only: bool,
    },
}

impl Command {
    pub fn execute(&self, client: Arc<Client>) -> Result<(), Error> {
        use self::Command::*;
        use interface::{
            routing_grpc::{RoutingServiceClient, RoutingService},
            routing::{ConnectPeerRequest, LightningAddress as LightningAddressRPC, ChannelGraphRequest},
            wallet_grpc::{WalletClient, Wallet},
            wallet::{NewAddressRequest, AddressType, GetUtxoListRequest, WalletBalanceRequest, SyncWithTipRequest, SendCoinsRequest},
            common::Void,
        };
        use build_info::get_build_info;

        let routing_service = RoutingServiceClient::with_client(client.clone());
        let wallet_service = WalletClient::with_client(client.clone());
        match self {
            GetInfo => {
                let response = routing_service
                    .get_info(Default::default(), Void::new())
                    .drop_metadata().wait().map_err(Error::Grpc)?;
                println!("{:?}", response);
                Ok(())
            },
            ConnectPeer { address } => {
                let mut request = ConnectPeerRequest::new();

                let mut lightning_address_rpc = LightningAddressRPC::new();
                lightning_address_rpc.set_pubkey(address.pub_key.clone());
                lightning_address_rpc.set_host(format!("{}", address.host));

                request.set_address(lightning_address_rpc);
                let response = routing_service
                    .connect_peer(Default::default(), request)
                    .drop_metadata().wait().map_err(Error::Grpc)?;
                println!("{:?}", response);
                Ok(())
            },
            DescribeGraph => {
                let mut request = ChannelGraphRequest::new();
                request.set_include_unannounced(false);
                let response = routing_service
                    .describe_graph(Default::default(), request)
                    .drop_metadata().wait().map_err(Error::Grpc)?;
                println!("{:?}", response);
                Ok(())
            },
            DescribeGraphDot => {
                let mut request = ChannelGraphRequest::new();
                request.set_include_unannounced(false);
                let response = routing_service
                    .describe_graph(Default::default(), request)
                    .drop_metadata().wait().map_err(Error::Grpc)?;
                let dot = network_graph::dot_format(response);
                println!("{:?}", dot);
                Ok(())
            },
            GetVersion => {
                println!("Client's version:");
                println!("{:#?}", get_build_info!());

                println!("Server's version:");
                let response = routing_service
                    .get_info(Default::default(), Void::new())
                    .drop_metadata().wait().map_err(Error::Grpc)?;
                println!("{}", response.version);
                Ok(())
            },
            ListPeers => {
                let request = Void::new();
                let response = routing_service
                    .list_peers(Default::default(), request)
                    .drop_metadata().wait().map_err(Error::Grpc)?;
                println!("{:?}", response);
                Ok(())
            },
            NewAddress { address_type } => {
                let mut request = NewAddressRequest::new();
                if address_type == "p2wkh" {
                    request.set_addr_type(AddressType::P2WKH);
                } else if address_type == "np2wkh" {
                    panic!("nested address type is not available");
                } else {
                    panic!("unknown address type: {}", address_type);
                }
                let response = wallet_service
                    .new_address(Default::default(), request)
                    .drop_metadata().wait().map_err(Error::Grpc)?;
                println!("{:?}", response);
                Ok(())
            },
            GetUtxoList => {
                let request = GetUtxoListRequest::new();
                let response = wallet_service
                    .get_utxo_list(Default::default(), request)
                    .drop_metadata().wait().map_err(Error::Grpc)?;
                println!("{:?}", response);
                Ok(())
            },
            GetWalletBalance => {
                let response = wallet_service
                    .sync_with_tip(Default::default(), SyncWithTipRequest::new())
                    .drop_metadata().wait().map_err(Error::Grpc)?;
                println!("{:?}", response);
                let request = WalletBalanceRequest::new();
                let response = wallet_service
                    .wallet_balance(Default::default(), request)
                    .drop_metadata().wait().map_err(Error::Grpc)?;
                println!("{:?}", response);
                Ok(())
            },
            SendMany {

            } => {
                unimplemented!()
            },
            SendCoins { address, amount, lock_coins, submit, witness_only } => {
                let mut request = SendCoinsRequest::new();
                request.set_dest_addr(address.clone());
                request.set_amt(amount.clone());
                request.set_lock_coins(lock_coins.clone());
                request.set_submit(submit.clone());
                request.set_witness_only(witness_only.clone());
                let response = wallet_service
                    .send_coins(Default::default(), request)
                    .drop_metadata().wait().map_err(Error::Grpc)?;
                println!("{:?}", response);
                Ok(())
            },
        }
    }
}
