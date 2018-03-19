extern crate rand;
use rand::Rng;
use std::io;

fn main() {
    let mut game = Game::new();
    game.run();
}

struct Game {
    mistakes: u32,
    word: String,
    complete: bool
}

impl Game {
    fn new() -> Game {
        Game {
            mistakes: 0,
            word: String::from(""),
            complete: false
        }
    }

    fn set_word(&mut self) {
       let num = rand::thread_rng().gen_range(0, 9); 
       self.word = match num {
           0 => String::from("apple"),
           1 => String::from("watermelon"),
           2 => String::from("tomato"),
           3 => String::from("mango"),
           4 => String::from("apricot"),
           5 => String::from("orange"),
           6 => String::from("avocado"),
           7 => String::from("peach"),
           8 => String::from("tangerine"),
           9 => String::from("strawberry"),
           _ => panic!("No word found for index {}", num)
       }
    }

    fn draw(&self) {
        println!("-----------");
        println!("|     |");

        print!("|");
        if self.mistakes > 0 {
            print!("     O");
        }
        print!("\n|");
        if self.mistakes > 1 {
            print!("    /");
        }

        if self.mistakes > 2 {
            print!("|");
        }

        if self.mistakes > 3 {
            print!("\\");
        }

        print!("\n|");

        if self.mistakes > 4 {
            print!("     /");
        }

        if self.mistakes > 5 {
            print!("\\");
        }
        
        print!("\n|\n|\n|\n");

        for character in self.word.chars() {
            print!("_ ");
        }
        
        println!("\nGuess?");
    }

    fn run(&mut self) {
       self.set_word();

       while !self.complete {
           self.draw();
           
           let mut guess = String::new();
           io::stdin().read_line(&mut guess).unwrap();
       }
    }
}
