use std::io;

fn main() {
    // get the value form user and save it guss variable.
    println!("Please enter the number: ");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to get the value");
    println!("Your guess is {}", guess);
}
