use tiny_http::{Server, Response};

fn main() {
    let server = Server::http("127.0.0.1:8765").unwrap();

    for request in server.incoming_requests() {
        println!("received request! method: {:?}, url: {:?}, headers: {:?}",
            request.method(),
            request.url(),
            request.headers()
        );
    
        let response = Response::from_string("hello world");
        let _ = request.respond(response);
    }
}