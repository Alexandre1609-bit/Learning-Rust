use std::io;

fn main() {
    let celsius = ask_temp();
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{}°C équivaut à {}°F", celsius, fahrenheit);
}

fn ask_temp() -> f64 {
       let mut ask_temp = String::new();
   
        loop {
            ask_temp.clear();
        
            println!("Entrez une température :");
            io::stdin()
                .read_line(&mut ask_temp)
                .expect("Erreur lecture");

            match ask_temp.trim().parse::<f64>() {
                Ok(num) => return num, //Ici "return" est nécessaire pour casser la boucle et la fonction
                Err(_) => {
                    println!("Erreur, entrez un nombre valide");
                    continue;
            }
        };
    }
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 1.8) + 32.0 //Pas besoin de "return" et ";" car la dernière ligne d'un bloc est automatiquement retournée !
}




    /*
    Point sur le shadowing 

    let x = 5;
    println!("Adresse mémoire 1 : {:p}", &x);

    let x = x + 1; //Premier shadowing;
    println!("valeur finale de x {x}");

    //Test changement de type pour "x"
    let x = "Je suis devenu du texte";
    println!("x est maintenant : {x}");  
    */