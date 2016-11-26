extern crate grpc;
extern crate grpc_examples;

use std::sync::{Arc, Mutex};
use std::thread;
use grpc::result::GrpcResult;

use grpc_examples::counter::{Request, Reply};
use grpc_examples::counter_grpc::{CounterService, CounterServiceServer};

struct CounterServiceImpl {
    requests: Arc<Mutex<u64>>,
}

impl CounterServiceImpl {
    fn new() -> CounterServiceImpl {
        CounterServiceImpl { requests: Arc::new(Mutex::new(0)) }
    }
}

impl CounterService for CounterServiceImpl {
    fn Count(&self, req: Request) -> GrpcResult<Reply> {
        println!("Request url: {}", req.get_url());
        *self.requests.lock().unwrap() += 1;
        println!("Request url: {}", *self.requests.lock().unwrap());
        Ok(Reply::new())
    }
}


fn main() {
    let _server = CounterServiceServer::new(50051, CounterServiceImpl::new());

    loop {
        thread::park();
    }
}
