mod router;
mod server;
use router::Router;
use server::Server;

fn main() {
    let srv_one = Server::new("Server one".to_string(), "192.168.0.1".to_string());
    let router_one = Router::new("Router one".to_string(), "192.168.0.2".to_string(), 4);

    start_diag(&srv_one);
    start_diag(&router_one);
}

pub trait Pingable {
    fn ping(&self);
}

fn start_diag(equipment: &impl Pingable) {
    equipment.ping();
}
