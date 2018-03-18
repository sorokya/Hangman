fn main() {
    let game = Game { mistakes: 0 };;
    game.run();
}

struct Game {
    mistakes: u32
}

impl Game {
    fn run(&self) {
        println!("-----------\n|       |\n|	O\n|      /|\\\n|       /\\\n|\n|");
        println!("-----------");
        println!("| |");

        print!("|");
        if self.mistakes > 0 {
            print!("    O");
        }
        print!("\n|");
        if self.mistakes > 1 {
        
        }
    }
}
