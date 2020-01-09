// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_SERVICE_SUBSCRIBE: ::grpcio::Method<super::output::request, super::output::response> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/falco.output.service/subscribe",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

#[derive(Clone)]
pub struct ServiceClient {
    client: ::grpcio::Client,
}

impl ServiceClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        ServiceClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn subscribe_opt(&self, req: &super::output::request, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::output::response>> {
        self.client.server_streaming(&METHOD_SERVICE_SUBSCRIBE, req, opt)
    }

    pub fn subscribe(&self, req: &super::output::request) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::output::response>> {
        self.subscribe_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Service {
    fn subscribe(&mut self, ctx: ::grpcio::RpcContext, req: super::output::request, sink: ::grpcio::ServerStreamingSink<super::output::response>);
}

pub fn create_service<S: Service + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_SERVICE_SUBSCRIBE, move |ctx, req, resp| {
        instance.subscribe(ctx, req, resp)
    });
    builder.build()
}
