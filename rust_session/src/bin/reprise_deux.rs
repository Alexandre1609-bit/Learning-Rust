fn main() {

    for i in 1..11 {
        if est_pair(i) {
            println!("Paire")
        } else {
            println!("Impair")
        }
    };
}

fn est_pair(nb: i32) -> bool {
    nb % 2 == 0
    }
