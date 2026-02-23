use crate::Pingable;

pub struct Router {
    name: String,
    ip_add: String,
    nb_of_ports: i32,
}

impl Router {
    pub fn new(name: String, ip_add: String, nb_of_ports: i32) -> Self {
        Router {
            name,
            ip_add,
            nb_of_ports,
        }
    }
}

impl Pingable for Router {
    fn ping(&self) {
        println!(
            "Sending ping to {} (with {} ports) at address {}... OK",
            self.name, self.nb_of_ports, self.ip_add
        );
    }
}
