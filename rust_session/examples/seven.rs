use std::io;

fn main() {

    let mut liste = Vec::new();
    

    loop {
        println!("Que voulez-vous ajouter ?");

        let mut demande: String = String::new();
    
    
        io::stdin()
            .read_line(&mut demande)
            .expect("Erreur de lecture");

        let clean_add = demande.trim();

        if clean_add == "stop" {
            println!("### Arrêt du service... ###");
            break;
        }
        
         liste.push(clean_add.to_string());
    }

    for article in &liste {
        println!("-{}", article);
    }



}