mod modules;
use std::io::{self, Write};

fn main() {
    loop {
        fn logo() {
            println!(
                r#"
██████   █████  ███    ██ ██████   ██████  ██████   █████  
██   ██ ██   ██ ████   ██ ██   ██ ██    ██ ██   ██ ██   ██ 
██████  ███████ ██ ██  ██ ██   ██ ██    ██ ██████  ███████ 
██      ██   ██ ██  ██ ██ ██   ██ ██    ██ ██   ██ ██   ██ 
██      ██   ██ ██   ████ ██████   ██████  ██   ██ ██   ██ 
                                                           
                                                           
        
                   Welcome to Pandora v0.1.0
            "#
            );
        }

        logo();
        
        println!("============ MENU =============");
        println!("1. Check file type by signature");
        println!("2. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap(); 

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => {
                print!("\x1B[2J\x1B[1;1H");
                if let Err(e) = modules::sgn_checker::search() {
                    eprintln!("Error: {}", e);
                }
            }
            "2" => {
                break;
            }
            _ => println!("Invalid option. Try again."),
        }
    }
}
