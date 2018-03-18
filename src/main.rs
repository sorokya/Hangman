fn main() {
    let game = Game { mistakes: 10 };;
    game.run();
}

struct Game {
    mistakes: u32
}

impl Game {
    fn run(&self) {
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
        
        print!("\n|\n|\n|");
    }
}
