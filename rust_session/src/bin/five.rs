use rand::Rng;
use std::io;

fn main() {
    println!("### Juste Prix ###");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let nb_guessed = guess_num();

        if nb_guessed == secret_number {
            println!("Juste prix ! Vous avez gagné !");
            break;
        } else if nb_guessed > secret_number {
            println!("C'est moins !");
        } else {
            println!("C'est plus !");
        }
    }
}

// Fonctions définies hors du main, en bas !

fn guess_num() -> i32 {
    let mut guess = String::new();
    
    loop {
        guess.clear();
        println!("Entrez un nombre :");

        io::stdin()
            .read_line(&mut guess)
            .expect("Erreur lecture");

        match guess.trim().parse() {
            Ok(num) => return num, 
            Err(_) => {
                println!("Ceci n'est pas un nombre valide !");
                continue;
            }
        };
    }
}