//use std::sync::Arc;
//
//use futures::*;
//use grpcio::{ChannelBuilder, ChannelCredentialsBuilder, Environment};
//
//use protos::output;
//use protos::output_grpc;
//
//use crate::protos;
//
//pub trait Connect {
//    fn connect(&self);
//}
//
//pub struct FalcoConnect {
//    pub env: Arc<Environment>,
//}
//
//impl Connect for FalcoConnect {
//    fn connect(&self) {
//        let outputs_client = output_grpc::ServiceClient::new(ch);
//        let mut req = output::request::default();
//        // Keepalive true means that the client will wait indefinitely for new events to come
//        // Use keepalive false if you only want to receive the accumulated events and stop
//        req.keepalive = true;
//        let mut res = outputs_client.subscribe(&req).unwrap();
//        loop {
//            let f = res.into_future();
//            match f.wait() {
//                Ok((Some(element), s)) => {
//                    res = s;
//                    println!("{:#?}", element);
//                }
//                // EOF
//                Ok((None, _)) => break,
//                // Error
//                Err((e, _)) => panic!("error: {:?}", e),
//            }
//        }
//    }
//}

mod client;