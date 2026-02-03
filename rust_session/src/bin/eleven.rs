struct Utilisateur {
    pseudo: String,
    age: u8,
}

impl Utilisateur {
    fn new(pseudo: String, age: u8) -> Self {
        Utilisateur { pseudo, age }
    }
}

enum TypeDeCompte {
    Courant,
    Epargne,
}

struct Compte {
    owner: Utilisateur,
    solde: f64,
    type_compte: TypeDeCompte,
}

impl Compte {
   
    fn new(owner: Utilisateur, solde: f64, type_compte: TypeDeCompte) -> Self {
        Compte { 
            owner, 
            solde,
            type_compte,
        }
    }

    fn withdraw(&mut self, amount: f64) {
        self.solde -= amount;
    }
}

fn show_info(compte: &Compte) {
    println!("Propriétaire : {}, Solde : {}", compte.owner.pseudo, compte.solde);
    
    match compte.type_compte {
        TypeDeCompte::Courant => println!("Compte courant"),
        TypeDeCompte::Epargne => println!("Compte epargne"),
    }
    
}

fn main() {
    let user = Utilisateur::new("Alex".to_string(), 30);
    
    
    let compte = Compte::new(user, 1000.0, TypeDeCompte::Courant); 
    
    show_info(&compte);
}