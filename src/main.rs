use std::io;
use rand::Rng;


fn opening(){
    println!("\n
        Hello \n
        This is Rust With VIM practice \n
        By Mike");
}

fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);

    opening();
    println!("Input your guess!(1-100)");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guess: {}",guess);
    let guess: isize = guess
        .trim()
        .parse()
        .expect("This is not a Number");
    let num2: isize = (guess).try_into().unwrap();
    println!("The final Answer: {}",num2);
    println!("The Correct Answer is : {}",secret);
    println!("You are {}!", secret==num2);
    


}
