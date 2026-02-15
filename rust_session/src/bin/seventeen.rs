fn main() {
    let u1 = Utilisateur::new(Some("Alex".to_string()), 23);
    let u2 = Utilisateur::new(None, 19);
    let u3 = Utilisateur::new(Some("Margot".to_string()), 22);

    let mut registre = vec![u1, u2, u3];

    for entry in &registre {
        entry.se_presenter()
    }
}

struct Utilisateur {
    pseudo: Option<String>,
    age: u8,
}

impl Utilisateur {
    fn new(pseudo: Option<String>, age: u8) -> Self {
        Utilisateur { pseudo, age }
    }

    fn se_presenter(&self) {
        match &self.pseudo {
            Some(p) => println!("Bonjour je suis {} et j'ai {} ans.", p, self.age),
            None => println!("Utilisateur anonyme"),
        }
    }
}
