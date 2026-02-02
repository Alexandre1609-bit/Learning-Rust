struct Utilisateur {
    pseudo: String,
    age: u8,
}

impl Utilisateur {
    fn new(pseudo: String, age: u8) -> Self {
        Utilisateur { pseudo, age }
    }
}



struct Compte {
    owner: Utilisateur,
    solde: f64,
    /
}

impl Compte {
   
    fn new(owner: Utilisateur, solde: f64) -> Self {
        Compte { 
            owner, 
            solde,
            /
        }
    }

    fn withdraw(&mut self, amount: f64) {
        self.solde -= amount;
    }
}

fn show_info(compte: &Compte) {
    println!("Propriétaire : {}, Solde : {}", compte.owner.pseudo, compte.solde);
    
    
}

fn main() {
    let user = Utilisateur::new("Alex".to_string(), 30);
    
    
    let mut compte = Compte::new(user, 1000.0); 
    
    show_info(&compte);
}