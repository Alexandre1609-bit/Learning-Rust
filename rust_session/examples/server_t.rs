fn main() {
    let mut srv = Server::new("Server one".to_string());

    srv.get_status();
    srv.change_status(ServerStatus::Updating);
    match srv.change_status(ServerStatus::Online) {
        Ok(s) => println!("Server status : {}", s),
        Err(msg) => println!("Error"),
    }
    srv.get_status();

    match srv.change_status(ServerStatus::Updating) {
        Ok(s) => println!("Server status : {}", s),
        Err(msg) => println!("Error : {}", msg),
    }

    match srv.change_status(ServerStatus::Offline) {
        Ok(s) => println!("Server status : {}", s),
        Err(msg) => println!("Error : {}", msg),
    }

    match srv.change_status(ServerStatus::Updating) {
        Ok(s) => println!("Server status : {}", s),
        Err(msg) => println!("Erros : {}", msg),
    }
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
                println!("### Server is offline ###")
            }
            ServerStatus::Updating => {
                println!("### Server is currently updating ###")
            }
        }
    }

    fn change_status(&mut self, new_status: ServerStatus) -> Result<String, String> {
        match new_status {
            ServerStatus::Online => {
                self.status = new_status;
                Ok("Online mode set correctly".to_string())
            }
            ServerStatus::Offline => {
                self.status = new_status;
                Ok("Server is now offline".to_string())
            }
            ServerStatus::Updating => match &self.status {
                ServerStatus::Updating => Ok("Server is updating".to_string()),
                ServerStatus::Online => Err("Can not update while online".to_string()),
                ServerStatus::Offline => {
                    self.status = new_status;
                    Ok("Server is now updating".to_string())
                }
            },
        }
    }
}
