fn main() {
    let mut srv = Server::new("Server one".to_string());

    srv.get_status();
    srv.change_status(ServerStatus::Online);
    srv.get_status();
}

enum ServerStatus {
    Online,
    Offline,
    Updating,
}

struct Server {
    name: String,
    status: ServerStatus,
}

impl Server {
    fn new(name: String) -> Self {
        Server {
            name,
            status: ServerStatus::Offline,
        }
    }

    fn get_status(&self) {
        match &self.status {
            ServerStatus::Online => {
                println!("### Server is ready ###")
            }
            ServerStatus::Offline => {
                println!("### Server is shutdown ###")
            }
            ServerStatus::Updating => {
                println!("### Server is currently updating ###")
            }
        }
    }

    fn change_status(&mut self, new_status: ServerStatus) {
        self.status = new_status;
    }
}
