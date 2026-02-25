mod router;
mod server;

use router::Router;
use server::Server;
use std::env;

fn main() {
    let mut cluster: Vec<Box<dyn Pingable>> = Vec::new();
    let env_one = env::var("MODE_INFRA").unwrap_or("DEV".to_string());
    let is_robot = env::var("AUTO_APPROVE").unwrap_or("false".to_string());

    if env_one == "PROD" {
        let mut auth_deploy = false;

        if is_robot == "true" {
            println!(" ### Access authorized, ready for full CI/CD check ###");
            auth_deploy = true;
        } else {
            println!(
                "### Warning you will deploy to prod, are you sure ? Type 'yes' in order to conitnue"
            );
            if clean_input() == "yes" {
                auth_deploy = true;
            }
        }

        if auth_deploy {
            for i in 1..=10 {
                let srv_name = format!("srv{}", i,);
                let srv_ip = format!("192.168.0.{}", i);
                cluster.push(Box::new(Server::new(srv_name, srv_ip))); //Technique à retenir !   

                let rt_name = format!("rt{}", i);
                let rt_ip = format!("10.0.0.{}", i);
                cluster.push(Box::new(Router::new(rt_name, rt_ip, 4)));
            }
        }
    } else {
        for i in 1..3 {
            let srv_name = format!("srv{}", i,);
            let srv_ip = format!("192.168.0.{}", i);
            cluster.push(Box::new(Server::new(srv_name, srv_ip)));

            let rt_name = format!("rt{}", i);
            let rt_ip = format!("10.0.0.{}", i);
            cluster.push(Box::new(Router::new(rt_name, rt_ip, 4)));
        }
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

fn clean_input() -> String {
    let mut inp_one = String::new();
    std::io::stdin()
        .read_line(&mut inp_one)
        .expect("Error occured while reading");
    inp_one.trim().to_string()
}
