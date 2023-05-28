use std::cmp::Ordering;
use std::io;
use rand::Rng;
use std::io::Write;
fn main() {
    
    let secret_number = rand::thread_rng().gen_range(1..=100);

    print!("\x1B[2J\x1B[1;1H"); //clear Screen   
    println!("Welcome to the Guessing Game!\n Here we have generated a magic number . You have guessed that  magic number if you guess that in first attempt then then you will get the highest score which is 100.\n Every Wrong gues you will loose 10 points. To\n Let's Play \n ");

    print!("Please enter your name:");
    io::stdout().flush().unwrap();
    
    let mut score = 100;
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Error reading");


    loop {

        println!("Please input your guess ");
        io::stdout().flush().unwrap();
       
        
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) =>num,
            Err(_) => continue,
        };

        println!("\n {} You guessed: {guess} \n",name);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                score = score -10;
                println! ("You lost 10 Points . Your remaining score to achieve is {score}");
            },
            Ordering::Greater =>{ 
                println!("Too big!");
                score = score - 10;
                println!("You lost 10 Points . Your remaining score to achieve is {score}");
             },
            Ordering::Equal => {
                println!(" Congratulations! You win! \n");
                println! ("Your Score is {score}");
                break;
            } 
        }

    }

    
}
