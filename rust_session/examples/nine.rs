struct Utilisateur {
    pseudo: String,
    age: u8,
}


impl Utilisateur {

    //Méthode 1
    fn se_presenter(&self) {
        println!("Bonjour, je suis {} et j'ai {} ans", self.pseudo, self.age);
    }    

    //Méthode 2
    fn est_majeur(&self) -> bool {
        self.age >= 18 
    }
}


fn main() {
    let new_user = Utilisateur {
    pseudo: String::from("Alex"),
    age: 25,
    };

    new_user.se_presenter();

    if new_user.est_majeur() {
        println!("Accès autorisé");
    } else {
        println!("Accès refusé")
    }

}

    



