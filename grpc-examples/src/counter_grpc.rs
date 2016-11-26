// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]


// interface

pub trait CounterService {
    fn Count(&self, p: super::counter::Request) -> ::grpc::result::GrpcResult<super::counter::Reply>;
}

pub trait CounterServiceAsync {
    fn Count(&self, p: super::counter::Request) -> ::grpc::futures_grpc::GrpcFutureSend<super::counter::Reply>;
}

// sync client

pub struct CounterServiceClient {
    async_client: CounterServiceAsyncClient,
}

impl CounterServiceClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        CounterServiceAsyncClient::new(host, port, tls).map(|c| {
            CounterServiceClient {
                async_client: c,
            }
        })
    }
}

impl CounterService for CounterServiceClient {
    fn Count(&self, p: super::counter::Request) -> ::grpc::result::GrpcResult<super::counter::Reply> {
        let p = ::futures::Future::wait(self.async_client.Count(p));
        match p {
            Ok(a) => {
                println!("OK {:?}", a);
                Ok(a)
            },
            Err(a) => {
                println!("ERR {}", a);
                Err(a)
            },
        }
    }
}

// async client

pub struct CounterServiceAsyncClient {
    grpc_client: ::grpc::client::GrpcClient,
    method_Count: ::std::sync::Arc<::grpc::method::MethodDescriptor<super::counter::Request, super::counter::Reply>>,
}

impl CounterServiceAsyncClient {
    pub fn new(host: &str, port: u16, tls: bool) -> ::grpc::result::GrpcResult<Self> {
        ::grpc::client::GrpcClient::new(host, port, tls).map(|c| {
            CounterServiceAsyncClient {
                grpc_client: c,
                method_Count: ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                    name: "/counter.CounterService/Count".to_string(),
                    streaming: ::grpc::method::GrpcStreaming::Unary,
                    req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                }),
            }
        })
    }
}

impl CounterServiceAsync for CounterServiceAsyncClient {
    fn Count(&self, p: super::counter::Request) -> ::grpc::futures_grpc::GrpcFutureSend<super::counter::Reply> {
        println!("CounterServiceAsync::Count");
        let r = self.grpc_client.call_unary(p, self.method_Count.clone());
        println!("CounterServiceAsync::Count[END]");
        r
    }
}

// sync server

pub struct CounterServiceServer {
    async_server: CounterServiceAsyncServer,
}

struct CounterServiceServerHandlerToAsync {
    handler: ::std::sync::Arc<CounterService + Send + Sync>,
    cpupool: ::futures_cpupool::CpuPool,
}

impl CounterServiceAsync for CounterServiceServerHandlerToAsync {
    fn Count(&self, p: super::counter::Request) -> ::grpc::futures_grpc::GrpcFutureSend<super::counter::Reply> {
        let h = self.handler.clone();
        ::grpc::rt::sync_to_async_unary(&self.cpupool, p, move |p| {
            h.Count(p)
        })
    }
}

impl CounterServiceServer {
    pub fn new<H : CounterService + Send + Sync + 'static>(port: u16, h: H) -> Self {
        let h = CounterServiceServerHandlerToAsync {
            cpupool: ::futures_cpupool::CpuPool::new_num_cpus(),
            handler: ::std::sync::Arc::new(h),
        };
        CounterServiceServer {
            async_server: CounterServiceAsyncServer::new(port, h),
        }
    }
}

// async server

pub struct CounterServiceAsyncServer {
    grpc_server: ::grpc::server::GrpcServer,
}

impl CounterServiceAsyncServer {
    pub fn new<H : CounterServiceAsync + 'static + Sync + Send + 'static>(port: u16, h: H) -> Self {
        let handler_arc = ::std::sync::Arc::new(h);
        let service_definition = ::grpc::server::ServerServiceDefinition::new(
            vec![
                ::grpc::server::ServerMethod::new(
                    ::std::sync::Arc::new(::grpc::method::MethodDescriptor {
                        name: "/counter.CounterService/Count".to_string(),
                        streaming: ::grpc::method::GrpcStreaming::Unary,
                        req_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                        resp_marshaller: Box::new(::grpc::grpc_protobuf::MarshallerProtobuf),
                    }),
                    {
                        let handler_copy = handler_arc.clone();
                        ::grpc::server::MethodHandlerUnary::new(move |p| handler_copy.Count(p))
                    },
                ),
            ],
        );
        CounterServiceAsyncServer {
            grpc_server: ::grpc::server::GrpcServer::new(port, service_definition),
        }
    }
}
