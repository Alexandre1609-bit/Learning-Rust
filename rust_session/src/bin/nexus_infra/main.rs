mod router;
mod server;

use router::Router;
use server::Server;

fn main() {
    let mut cluster: Vec<Server> = Vec::new();

    for i in 1..=10 {
        let srv_name = format!("srv{}", i,);

        let srv_ip = format!("192.168.0.{}", i);

        cluster.push(Server::new(srv_name, srv_ip)); //Technique à retenir !
    }

    for s in &cluster {
        start_diag(s);
    }
}

pub trait Pingable {
    fn ping(&self);
}

fn start_diag(equipment: &impl Pingable) {
    equipment.ping();
}
