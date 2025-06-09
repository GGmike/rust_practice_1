use rand::Rng;
use std::io;


#[derive(Debug)]
struct Todo {
    title: String,
    description: String,
    completed: bool,}


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
        \n 0. Quit
        \n"
    );
}

fn ToDoList(){
    let mut todo_list: Vec<Todo> = Vec::new();
    loop{
        println!(" TO DO LIST 
        \n  1. Add a new task
        \n  2. View tasks
        \n  3. Mark task as completed
        \n  4. Remove a task
        \n  0. Back to main menu");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");
        let choice: isize = choice.trim().parse().expect("This is not a Number");

        match choice{
            
        }

    }
}

fn Guessing() {
    let mut maxValue: isize = 100;
    let mut minValue: isize = 1;
    let mut temp: isize = 0;
    let mut attempt: i32 = 0;
    let mut miVal = String::new();
    let mut maVal = String::new();
    println!("Welcome to the Guessing Game!");
    println!("Please enter the minimum value (default is 1):");
    io::stdin().read_line(&mut miVal).expect("Read Line failed");
    if !miVal.trim().is_empty() {
        minValue = miVal.trim().parse().expect("This is not a Number");
    }
    println!("Please enter the maximum value (default is 100):");
    io::stdin().read_line(&mut maVal).expect("Read Line failed");
    if !maVal.trim().is_empty() {
        maxValue = maVal.trim().parse().expect("This is not a Number");
    }
    if minValue >= maxValue {
        println!("Invalid range. Minimum value {} must be less than maximum value {}.", minValue, maxValue);
        println!("Swapping values to correct the range.");
        temp = minValue;
        minValue = maxValue;
        maxValue = temp;
        println!("Range adjusted to: {} - {}", minValue, maxValue);

    }

    let secret = rand::thread_rng().gen_range(minValue..=maxValue);

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
    loop {
        opening();
        let mut menu_num = String::new();
        io::stdin()
            .read_line(&mut menu_num)
            .expect("Read Line failed");
        let menu_num: isize = menu_num.trim().parse().expect("This is not a Number");
        println!("You entered {}", menu_num);
        match menu_num {
            0 => break,
            1 => Guessing(),
            2 => println!("To Do List"),
            3 => println!("Calculator"),
            _ => {
                println!("Invalid Option");
                break;
            }
        }
    }
}