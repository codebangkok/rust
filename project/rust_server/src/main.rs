use rust_server::http::Server;

fn main() {
    let server = Server::new("127.0.0.1:8000".to_string());
    if let Err(e) = server.run() {
        println!("{}", e);
    }
}
