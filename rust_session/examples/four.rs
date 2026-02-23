use std::io; //module input / output

fn main() {
    println!("### Le perroquet économe ###");
    let mut entree: String = String::new();
    loop {
        
        entree.clear(); //On vide le message précédent (sinon le text s'empile)
        //Avec .clear(), la capacité mémoire reste réservée. Si l'utilisateur tape une phrase de 50 lettres, la String grandit.
        // Au tour suivant, quand on fait .clear(), la longueur revient à 0, mais la "capacité" reste à 50.
        println!("Ecrivez un message (ou 'stop') : ");
        

        io::stdin()
            .read_line(&mut entree)
            .expect("Entrez un message valide");

        println!("Le perroquet répond : {entree}"); //error : format argument must be a string literal je passe la fonction "entree" entre {} pour voir si ça fonctionne

        if entree.trim() == "stop" {
            println!("Au revoir");
            break;
        }
    }
}
