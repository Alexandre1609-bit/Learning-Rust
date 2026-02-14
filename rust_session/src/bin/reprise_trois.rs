fn main() {
    let mon_texte = String::from("Japon");
    let taille = calculer_taille(&mon_texte);
    println!("Le mot {} fait {} lettres", mon_texte, taille)   
}

fn calculer_taille(arg: &String) -> usize {
    arg.len()
}
