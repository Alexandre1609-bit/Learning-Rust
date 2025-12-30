#![allow(unused)]

#[derive(Debug)] //Se renseigner via la doc
struct Lang {
    langauge: String,
    version: String,
}

fn main() {
    let lang = "rust";
    println!("hello {}", lang);
    println!("hello {} {}", lang, lang);
    println!("hello {lang}");

    let x = 2;
    let res = x * x; //Plus propre de calculer le résultat avant pour une meilleure lisibilité
    println!("{x} * {x} = {res}");
    println!("{0} * {0} = {1}", x, x * x);

    let lang = Lang {
        langauge: "rust".to_string(),
        version: "1.83".to_string(),
    };

    //print ma fonction lang (2 méthodes ?)
    //1. debug feature, on doit déclarer un attribut avant (voir la classe lang)
    println!("{:?}", lang);

    //2. utiliser une variation de la 1ère méthode ?
    //Différence : la première méthode va pint lang en tant que "struct"
    //et la 2èeme (celle-ci) va le print mais avec des line breaks
    println!("{:#?}", lang)

    //Comapraison des 2 : 
    //Lang { langauge: "rust", version: "1.83" } 1ère
    //Lang { 
        //langauge: "rust",
        //version: "1.83",
//  } 2ème

    //Cargot fmt
    //Formate le code, fix le code pour corriger l'indentation et le line breaks et autre
}