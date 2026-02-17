fn main() {
    let mut comte_test = Compte::new(100.0);
    let ret = comte_test.retirer(50.0);

    match ret {
        Ok(nb) => {
            println!("Nouveau solde : {}", nb)
        }
        Err(e) => {
            println!("Montant invalide")
        }
    }
}

struct Compte {
    solde: f64,
}

impl Compte {
    fn new(solde_initiale: f64) -> Self {
        Compte {
            solde: solde_initiale,
        }
    }

    fn retirer(&mut self, montant: f64) -> Result<f64, String> {
        if montant > self.solde {
            Err("Erreur, solde insufisant".to_string())
        } else {
            self.solde -= montant;
            Ok(self.solde)
        }
    }
}
