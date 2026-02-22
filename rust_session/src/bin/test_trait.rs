fn main() {
    let srv_one = Server::new("Server one".to_string(), "192.168.0.1".to_string());
    let router_one = Router::new("Router one".to_string(), "192.168.0.2".to_string(), 4);

    start_diag(&srv_one);
    start_diag(&router_one);
}

trait Pingable {
    fn ping(&self);
}

struct Server {
    name: String,
    ip_add: String,
}

struct Router {
    name: String,
    ip_add: String,
    nb_of_ports: i32,
}

impl Pingable for Server {
    fn ping(&self) {
        println!(
            "Sending ping to {} at address {}... OK ",
            self.name, self.ip_add
        );
    }
}

impl Pingable for Router {
    fn ping(&self) {
        println!(
            "Sending ping to {} at address {}... OK",
            self.name, self.ip_add
        );
    }
}

impl Server {
    fn new(name: String, ip_add: String) -> Self {
        Server { name, ip_add }
    }
}

impl Router {
    fn new(name: String, ip_add: String, nb_of_ports: i32) -> Self {
        Router {
            name,
            ip_add,
            nb_of_ports,
        }
    }
}

fn start_diag(equipment: &impl Pingable) {
    equipment.ping();
}