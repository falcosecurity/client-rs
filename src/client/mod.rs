use std::sync::Arc;

use futures::*;
use grpcio::{ChannelBuilder, ChannelCredentialsBuilder, Environment};

use protos::output;
use protos::output_grpc;

use crate::protos;

pub trait Connect {
    fn connect(&self);
}

pub struct FalcoConnect {
    pub env: Arc<Environment>,
}

impl Connect for FalcoConnect {
    fn connect(&self) {
        let ca_crt = include_str!("/tmp/certs/ca.crt").into();
        let client_crt = include_str!("/tmp/certs/client.crt").into();
        let client_key = include_str!("/tmp/certs/client.key").into();
        let credentials = ChannelCredentialsBuilder::new()
            // Set the PEM encoded server root cert to verify server's identity
            .root_cert(ca_crt)
            // Set the PEM encoded client side cert and key
            .cert(client_crt, client_key)
            // Create channel credentials
            .build();
        let ch =
            ChannelBuilder::new(self.env.clone()).secure_connect("localhost:5060", credentials);
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
    }
}

//struct FalcoConnectBuilder {
    //env: Arc<Environment>,
    //root: Option<String>,
    //cert_key_pair: Option<(String, String)>,
    //address: Option<String>,
//}

//impl FalcoConnectBuilder {
    ///// Initialize a new [`FalcoConnectBuilder`].
    //pub fn new(env: Arc<Environment>) -> FalcoConnectBuilder {
        //FalcoConnectBuilder {
            //root: None,
            //cert_key_pair: None,
            //address: None,
        //}
    //}

    ///// todo
    //pub fn root_cert_from_file(mut self, root_cert_path: String) -> FalcoConnectBuilder {
        //self.root = Some(include_str!(&root_cert_path).into());
        //self
    //}

    ///// Finalize the [`FalcoConnectBuilder`] and build the [`FalcoConnect`].
    //pub fn build(mut self) -> FalcoConnect {
        //FalcoConnect { env: self.env }
    //}
//}
