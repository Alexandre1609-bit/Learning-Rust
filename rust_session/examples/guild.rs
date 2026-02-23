fn main() {
    let alex = Aventurier::new("Alexandre".to_string(), 50, Some("le Brave".to_string()));
    let mitty = Aventurier::new("Mitty".to_string(), 50, Some("le chat".to_string()));
    let ugo = Aventurier::new("Ugo".to_string(), 35, None);

    let mut groupe = vec![alex, mitty, ugo];

    for av in &groupe {
        av.se_presenter();
    }

    for member in &mut groupe {
        match member.subir_attaque(40) {
            Ok(pv_restant) => {
                println!(
                    "{} est en vie ! Il lui reste {} point de vie",
                    member.nom, pv_restant
                )
            }
            Err(message) => {
                println!("{} est tombé au combat", member.nom)
            }
        }
    }
}

struct Aventurier {
    nom: String,
    point_de_vie: i32,
    titre: Option<String>,
}

impl Aventurier {
    fn new(nom: String, point_de_vie: i32, titre: Option<String>) -> Self {
        Aventurier {
            nom,
            point_de_vie,
            titre,
        }
    }

    fn se_presenter(&self) {
        match &self.titre {
            Some(t) => println!(
                "Je suis {} {} et j'ai {} pv",
                self.nom, t, self.point_de_vie
            ),
            None => println!(
                "Je suis {}, un simple novice, et j'ai {} pv",
                self.nom, self.point_de_vie
            ),
        }
    }

    fn subir_attaque(&mut self, degat: i32) -> Result<i32, String> {
        if degat >= self.point_de_vie {
            self.point_de_vie = 0;
            Err("Vous êtes tombé au combat".to_string())
        } else {
            self.point_de_vie -= degat;
            Ok(self.point_de_vie)
        }
    }
}
