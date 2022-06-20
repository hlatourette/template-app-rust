use std::io;

mod utility;

fn main() {
    let mut guess = String::new();
    println!("What would you like to know?");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("{}", utility::deep_thought::meaning_of_life_the_universe_and_everything(&guess));
}
