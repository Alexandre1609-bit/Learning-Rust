fn main() {
    let cp1 = Compte::new(100.0);
    let cp2 = Compte::new(200.0);
    let cp3 = Compte::new(500.0);
    let mut banque = vec![cp1, cp2, cp3];

    for i in &mut banque {
        i.ajouter_interets(10.0);
    }

    println!("### Bilan après intérêts ###");

    for cp in &banque {
        println!("Solde final : {}", cp.solde)
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

    fn ajouter_interets(&mut self, bonus: f64) {
        self.solde += bonus
    }
}
