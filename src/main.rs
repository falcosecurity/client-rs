use futures::*;

use grpcio::{ChannelBuilder, ChannelCredentialsBuilder, EnvBuilder};
use protos::output;
use protos::output_grpc;
use std::sync::Arc;

fn main() {
    let ca_crt = include_str!("/tmp/certs/ca.crt").into();
    let client_crt = include_str!("/tmp/certs/client.crt").into();
    let client_key = include_str!("/tmp/certs/client.key").into();

    let env = Arc::new(EnvBuilder::new().build());
    let credentials = ChannelCredentialsBuilder::new()
    // Set the PEM encoded server root cert to verify server's identity
    .root_cert(ca_crt)
    // Set the PEM encoded client side cert and key
    .cert(client_crt, client_key)
    // Create channel credentials
    .build();

    let ch = ChannelBuilder::new(env).secure_connect("localhost:5060", credentials);

    let outputs_client = output_grpc::ServiceClient::new(ch);
    let mut req = output::request::default();
    // Keepalive true means that the client will wait indefinitely for new events to come
    // Use keepalive false if you only want to receive the accumulated events and stop
    req.keepalive = true;

    let mut res = outputs_client.subscribe(&req).unwrap();
    loop {
        let f = res.into_future();
        match f.wait() {
            Ok((Some(element), s)) => {
                res = s;
                println!("{:#?}", element);
            }
            // EOF
            Ok((None, _)) => break,
            // Error
            Err((e, _)) => panic!("error: {:?}", e),
        }
    }

    //todo: read the future here
}
