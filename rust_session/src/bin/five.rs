use rand::Rng;
use std::io;
fn main() {
    println!("### Juste Prix ###");

    //Générer un nombre avec rand
    //thread_rng() : initialise le générateur
    //gen_range(1..=100) : génère un nombre entre 1 et 100 INCLUS !!
    let secret_number: i32 = rand::thread_rng().gen_range(1..=100);
    
    let mut guess: String= String::new();

    loop {
        guess.clear(); //Ne pas oublier ! 
        
        println!("Entrez un nombre");
        io::stdin()
            .read_line(&mut guess)
            .expect("Entrez un nombre valide !");

        let nb_guessed: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrez un vrai nombre !");
                continue;
            }
        };

        if nb_guessed == secret_number {
            println!("Juste prix ! Vous avez gagné rien du tout !");
            break;
        } else if nb_guessed > secret_number {
            println!("C'est moins !");
        } else {
            println!("C'est plus !");
        };
    }
}