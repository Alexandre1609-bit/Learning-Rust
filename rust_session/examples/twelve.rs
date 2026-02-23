struct Utilisateur {
    pseudo: Option<String>,
    age: u8,
}

impl Utilisateur {
    fn new(pseudo: Option<String>, age: u8) -> Self {
        Utilisateur{ pseudo, age }
    }

    fn se_presenter(&self) {

        match &self.pseudo {
            Some(p) => println!("bonjour, je suis {} et j'ai {} ans", p, self.age),
            None => println!("bonjour, je suis {} et j'ai {} ans", "Utilisateur anonyme", self.age),

            }
        }    
    }


fn main() {

    let u1 = Utilisateur::new(Some("Alex".to_string()), 24);

    let u2 = Utilisateur::new(None, 24);

    u1.se_presenter();
    u2.se_presenter();

}