mod router;
mod server;

use router::Router;
use server::Server;

fn main() {
    let mut cluster: Vec<Box<dyn Pingable>> = Vec::new();

    let router = Box::new(Router::new("rt_box".to_string(), "10.0.0.2".to_string(), 8));

    cluster.push(router);

    for i in 1..=10 {
        let srv_name = format!("srv{}", i,);
        let srv_ip = format!("192.168.0.{}", i);
        cluster.push(Box::new(Server::new(srv_name, srv_ip))); //Technique à retenir !   

        let rt_name = format!("rt{}", i);
        let rt_ip = format!("10.0.0.{}", i);
        cluster.push(Box::new(Router::new(rt_name, rt_ip, 4)));
    }

    for s in &cluster {
        start_diag(s.as_ref());
    }
}
pub trait Pingable {
    fn ping(&self);
}

fn start_diag(equipment: &dyn Pingable) {
    equipment.ping();
}
