fn main() {
    let nombre = 5.0;
    let mut compteur = 0;
    println!("Le carre de 5 est {}", carre(nombre));
    compteur += 10;
    println!(
        "Valeurs finales compteur : {}, carre : {}",
        compteur,
        carre(nombre)
    )
}

fn carre(nb: f64) -> f64 {
    nb * nb
}
