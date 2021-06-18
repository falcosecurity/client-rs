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

const METHOD_SERVICE_VERSION: ::grpcio::Method<super::version::request, super::version::response> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/falco.version.service/version",
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

    pub fn version_opt(&self, req: &super::version::request, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::version::response> {
        self.client.unary_call(&METHOD_SERVICE_VERSION, req, opt)
    }

    pub fn version(&self, req: &super::version::request) -> ::grpcio::Result<super::version::response> {
        self.version_opt(req, ::grpcio::CallOption::default())
    }

    pub fn version_async_opt(&self, req: &super::version::request, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::version::response>> {
        self.client.unary_call_async(&METHOD_SERVICE_VERSION, req, opt)
    }

    pub fn version_async(&self, req: &super::version::request) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::version::response>> {
        self.version_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Service {
    fn version(&mut self, ctx: ::grpcio::RpcContext, req: super::version::request, sink: ::grpcio::UnarySink<super::version::response>);
}

pub fn create_service<S: Service + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let mut instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_SERVICE_VERSION, move |ctx, req, resp| {
        instance.version(ctx, req, resp)
    });
    builder.build()
}
