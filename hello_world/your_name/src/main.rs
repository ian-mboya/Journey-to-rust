use std::io;

fn main() {
    println!("Hello there champ, i need your name");
    
    println!("Enter your name");


    let mut name = String::new();

    io::stdin()
	.read_line(&mut name)
	.expect("We can't get your name");


 println!("Here is your name: {}", name);
}
