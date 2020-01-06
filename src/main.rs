use grpcio::{ChannelBuilder, EnvBuilder};
use protos::output_grpc;
use protos::output;
use std::sync::Arc;

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:5060");
    let sc = output_grpc::ServiceClient::new(ch);
    let mut req = output::request::default();
    req.keepalive = true;

    let res = sc.subscribe(&req);

    //todo: read the future here

}
