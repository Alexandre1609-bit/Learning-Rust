use std::io; //module input / output

fn main() {
    println!("### Convertisseur d'âge ###");
    println!("Entrez votre âge");

    let mut entree = String::new(); //On crée un "boîte vide" pour stocker le text

    io::stdin()
        .read_line(&mut entree) //On lit ce que l'utilisateur tape
        .expect("Erreur lors de la lecture"); //Sécurité en cas de problème

    //Transforme le text en nombre
    let age: u32 = entree.trim().parse().expect("Entrez un nombre valide !");

   if age < 18 {
    println!("Vous êtes encore trop jeune");
   } else if age >= 18 && age <= 50 { //Encore mieux : else if (18..=50).contains(&age) {} !!!
    println!("Bievenue à bord !")
   } else {
       println!("Bienvenue capitaine ! ")
   }

}