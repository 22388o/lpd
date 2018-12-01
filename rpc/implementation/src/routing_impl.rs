use grpc::{rt::ServerServiceDefinition, RequestOptions, SingleResponse};
use interface::routing_grpc::{RoutingServiceServer, RoutingService};
use interface::routing::{
    SignMessageRequest, SignMessageResponse,
    ConnectPeerRequest, PeerList, Info,
    ChannelGraphRequest, ChannelGraph,
    QueryRoutesRequest, RouteList,
};
use interface::common::Void;
use routing::State;
use std::sync::{RwLock, Arc};

pub fn service(state: Arc<RwLock<State>>) -> ServerServiceDefinition {
    RoutingServiceServer::new_service_def(RoutingImpl {
        state: state,
    })
}

struct RoutingImpl {
    state: Arc<RwLock<State>>,
}

impl RoutingService for RoutingImpl {
    fn sign_message(&self, o: RequestOptions, p: SignMessageRequest) -> SingleResponse<SignMessageResponse> {
        let _ = (o, p);
        unimplemented!()
    }

    fn connect_peer(&self, o: RequestOptions, p: ConnectPeerRequest) -> SingleResponse<Void> {
        let _ = (o, p);
        unimplemented!()
    }

    fn list_peers(&self, o: RequestOptions, p: Void) -> SingleResponse<PeerList> {
        let _ = (o, p);
        unimplemented!()
    }

    fn get_info(&self, o: RequestOptions, p: Void) -> SingleResponse<Info> {
        let _ = (o, p);
        unimplemented!()
    }

    fn describe_graph(&self, o: RequestOptions, p: ChannelGraphRequest) -> SingleResponse<ChannelGraph> {
        let state = self.state.read().unwrap();
        let _ = o;

        let (e, n) = state.describe(p.get_include_unannounced());

        let mut r = ChannelGraph::new();
        r.set_edges(e.into());
        r.set_nodes(n.into());
        SingleResponse::completed(r)
    }

    fn query_routes(&self, o: RequestOptions, p: QueryRoutesRequest) -> SingleResponse<RouteList> {
        let _ = (o, p);
        unimplemented!()
    }
}
