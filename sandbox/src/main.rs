use hyper::{Body, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Specify the IPv6 address and port here
    let addr = SocketAddr::from(([0, 0, 0, 0, 0, 0, 0, 1], 8080));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(|_req| async {
            Ok::<_, Infallible>(Response::new(Body::from("Hello World")))
        }))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Starting server at port 8080 on IPv6");

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
