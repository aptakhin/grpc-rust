//extern crate grpc;
extern crate grpc_examples;

use grpc_examples::counter::{Request, Reply};
use grpc_examples::counter_grpc::{CounterService, CounterServiceClient};


fn main() {
    let client = CounterServiceClient::new("localhost", 50051, false).unwrap();
    let req = Request::new();
    println!("Send unfinished: {:?}", req);
    let resp = client.Count(req);
    println!("RESP: {:?}", resp);
    println!("GOT {}!", resp.unwrap().get_status());
}
