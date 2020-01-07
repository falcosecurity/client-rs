use futures::*;

use grpcio::{ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder, ChannelCredentials};
use log;
use protos::output;
use protos::output_grpc;
use std::sync::Arc;

fn main() {
    let env = Arc::new(EnvBuilder::new().build());
    let mut cb = ChannelCredentialsBuilder::new();
    // Set the PEM encoded server root cert to verify server's identity
    cb.root_cert();
    // Set the PEM encoded client side cert and key
    cb.cert();
    let cc = cb.build();
    let ch = ChannelBuilder::new(env).secure_connect("localhost:5060", cc);
    let sc = output_grpc::ServiceClient::new(ch);
    let mut req = output::request::default();
    req.keepalive = true;

    let mut res = sc.subscribe(&req).unwrap();
    loop {
        let f = res.into_future();
        match f.wait() {
            Ok((Some(feature), s)) => {
                res = s;
                log::info!("Thing {}", feature.get_rule());
            }
            Ok((None, _)) => break,
            Err((e, _)) => panic!("List things failed: {:?}", e),
        }
    }

    //todo: read the future here
}
