// This file is generated. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_SERVICE_SUB: ::grpcio::Method<super::outputs::request, super::outputs::response> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Duplex,
    name: "/falco.outputs.service/sub",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_SERVICE_GET: ::grpcio::Method<super::outputs::request, super::outputs::response> = ::grpcio::Method {
    ty: ::grpcio::MethodType::ServerStreaming,
    name: "/falco.outputs.service/get",
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

    pub fn sub_opt(&self, opt: ::grpcio::CallOption) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::outputs::request>, ::grpcio::ClientDuplexReceiver<super::outputs::response>)> {
        self.client.duplex_streaming(&METHOD_SERVICE_SUB, opt)
    }

    pub fn sub(&self) -> ::grpcio::Result<(::grpcio::ClientDuplexSender<super::outputs::request>, ::grpcio::ClientDuplexReceiver<super::outputs::response>)> {
        self.sub_opt(::grpcio::CallOption::default())
    }

    pub fn get_opt(&self, req: &super::outputs::request, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::outputs::response>> {
        self.client.server_streaming(&METHOD_SERVICE_GET, req, opt)
    }

    pub fn get(&self, req: &super::outputs::request) -> ::grpcio::Result<::grpcio::ClientSStreamReceiver<super::outputs::response>> {
        self.get_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Service {
    fn sub(&mut self, ctx: ::grpcio::RpcContext, stream: ::grpcio::RequestStream<super::outputs::request>, sink: ::grpcio::DuplexSink<super::outputs::response>);
    fn get(&mut self, ctx: ::grpcio::RpcContext, req: super::outputs::request, sink: ::grpcio::ServerStreamingSink<super::outputs::response>);
}

pub fn create_service<S: Service + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_duplex_streaming_handler(&METHOD_SERVICE_SUB, move |ctx, req, resp| {
        instance.sub(ctx, req, resp)
    });
    let mut instance = s.clone();
    builder = builder.add_server_streaming_handler(&METHOD_SERVICE_GET, move |ctx, req, resp| {
        instance.get(ctx, req, resp)
    });
    builder.build()
}
