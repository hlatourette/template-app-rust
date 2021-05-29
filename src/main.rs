use std::io;

fn main() {
    let mut guess = String::new();

    println!("What would you like to know?");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("{}", meaning_of_life_the_universe_and_everything(&guess));
}

// TODO: move into example module
fn meaning_of_life_the_universe_and_everything(_question: &String) -> i32 { // parameter prefixed to suppress warning
    42
}