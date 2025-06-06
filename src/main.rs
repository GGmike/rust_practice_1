use rand::Rng;
use std::io;

fn opening() {
    println!(
        "\n
        Hello \n
        This is Rust With VIM practice \n
        By Mike"
    );

    println!(
        "\n
        \n 1. Guessing Game
        \n 2. To Do List
        \n 3. Calculator
        \n"
    );
}

fn Guessing() {
    let secret = rand::thread_rng().gen_range(1..=100);
    let mut maxValue: isize = 100;
    let mut minValue: isize = 1;
    let mut attempt: i32 = 0;
    loop {
        println!("Guess The number {} - {}", minValue, maxValue);
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();

        //    println!("You guess: {}",guess);
        let guess: isize = guess.trim().parse().expect("This is not a Number");

        attempt += 1;

        if guess < minValue || guess > maxValue {
            println!("Out of bound");
            continue;
        }

        //let num2: isize = (guess).try_into().unwrap();
        //  println!("The final Answer: {}",secret);
        if guess > secret {
            maxValue = guess;
        //    println!
        } else if guess < secret {
            minValue = guess;
        } else {
            println!("Correct! Total Attempts: {}", attempt);
            println!("The secret number was: {}", secret);
            println!("Do you want to play again? (y/n)");
            let mut play_again = String::new();
            io::stdin()
                .read_line(&mut play_again)
                .expect("Read Line failed");
            let play_again = play_again.trim().to_lowercase();
            if play_again == "y" {
                Guessing();
            } else {
                println!("Thanks for playing!");
                break;
            }
            break;
        }
    }
}

fn main() {
    opening();
    let mut menu_num = String::new();
    io::stdin()
        .read_line(&mut menu_num)
        .expect("Read Line failed");
    let menu_num: isize = menu_num.trim().parse().expect("This is not a Number");
    println!("You entered {}", menu_num);
    match menu_num {
        1 => Guessing(),
        2 => println!("To Do List"),
        3 => println!("Calculator"),
        _ => println!("Invalid Option"),
    }
}
