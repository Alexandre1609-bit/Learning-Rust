use std::io;



fn main() {

    let mut ask_pseudo = String::new();
    let mut ask_age = String::new();
    //let mut test = Vec::new();

    println!("Choisissez un pseudo :");
    io::stdin()
        .read_line(&mut ask_pseudo)
        .expect("Erreur de lecture");

    println!("Entrez votre âge :");
    io::stdin()
        .read_line(&mut ask_age)
        .expect("Erreur de lecture");

    let age_num = match ask_age.trim().parse::<u8>() {
        Ok(age) => age,
        Err(_) => {
            println!("Erreur lors de la lecture de l'âge");
            return;
        }
    };

    let clean_psd = ask_pseudo.trim();

    let new_user = Utilisateur {
        pseudo: clean_psd.to_string(),
        age: age_num,
    }; 

    //test.push(new_user); 
    //println!("Liste des utilisateurs : {:?}", test);
    println!("Bienvenue {}, vous avez {} ans ! ", new_user.pseudo, new_user.age);


}

#[derive(Debug)] // Génère automatiquement le code pour l'affichage de la struct en mode debug
struct Utilisateur {
    pseudo: String,
    age: u8,
}