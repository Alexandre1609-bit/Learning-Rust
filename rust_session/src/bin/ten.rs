struct Utilisateur {
    pseudo: String,
    age: u8,
}

impl Utilisateur {

    //Associate function "new", qui ne prend pas "Self" en paramètre
    fn new(pseudo: String, age: u8) -> Self {
        Utilisateur { pseudo, age }
    }

    //Méthode 1
    fn se_presenter(&self) {
        println!("Bonjour, je suis {} et j'ai {} ans", self.pseudo, self.age);
    }    

    //Méthode 2
    fn est_majeur(&self) -> bool {
        self.age >= 18 
    }
 
}

struct Compte {
    owner: Utilisateur,
    solde: f64,
}

impl Compte {
    
    fn new (owner: Utilisateur, solde: f64) -> Self {
        Compte { owner, solde }
    }
}



fn main() {
    let new_user = Utilisateur::new(String::from("Alex"), 24);

    let new_compte = Compte::new(new_user, 1000.0); 

    println!("Le compte appartient à {} et le solde est de {}", new_compte.owner.pseudo, new_compte.solde);

    new_compte.owner.se_presenter();
}

    



