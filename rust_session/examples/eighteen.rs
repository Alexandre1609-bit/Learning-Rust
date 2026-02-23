fn main() {

    let rs = divide(5.0, 9.0);
    match rs {
        Ok(nb) => {println!("Résultat : {}", nb)},
        Err(msg) => {println!("Erreur")},
    }
    
}

fn divide(a:f64, b:f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Erreur, b = 0 !".to_string())
    } else {
        Ok(a / b)
    }      
}