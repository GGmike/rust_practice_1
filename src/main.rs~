use std::io;

fn opening(){
    println!("\n
        Hello \n
        This is Rust With VIM practice \n
        By Mike");
}

fn main() {
    opening();
    println!("Input your guess!");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guess: {}",guess);
    let guess: isize = guess
        .trim()
        .parse()
        .expect("This is not a Number");
    let num2: isize = (guess*-2).try_into().unwrap();
    println!("The final Answer: {}",num2);
    


}
