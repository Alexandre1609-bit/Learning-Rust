fn main() {
    let mut mon_texte = String::from("Japon");
    ajouter_suffixe(&mut mon_texte);
}

fn ajouter_suffixe(arg: &mut String) {
    arg.push_str(" est magnifique");
}