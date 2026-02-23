use crate::Pingable;

pub struct Server {
    name: String,
    ip_add: String,
}

impl Server {
    pub fn new(name: String, ip_add: String) -> Self {
        Server { name, ip_add }
    }
}

impl Pingable for Server {
    fn ping(&self) {
        println!(
            "Sending ping to {} at address {}... OK ",
            self.name, self.ip_add
        );
    }
}
