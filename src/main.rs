use futures::future::{Future, IntoFuture};
use grpcio::{ChannelBuilder, EnvBuilder};
use protos::output;
use protos::output_grpc;
use std::sync::Arc;

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect("localhost:5060");
    let sc = output_grpc::ServiceClient::new(ch);
    let mut req = output::request::default();
    req.keepalive = true;

    let mut res = sc.subscribe(&req);
    loop {
        let f = res.into_future();
        match f.wait() {
            Ok((Some(element), s)) => {
                res = s;
            }
            Ok((None, _)) => break,
        }
    }

    //todo: read the future here
}
