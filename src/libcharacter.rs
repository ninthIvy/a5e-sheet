use std::io;
use serde::{Serialize, Deserialize};
#[derive(Default, Serialize, Deserialize)]
pub struct Character {
    pub stats: [u8; 6],
    pub name: String,
}

impl Character {
    pub fn init(&mut self) {
        
        let mut buffer = String::new();
        let stdin = io::stdin();
        let choice: u8;

        println!("Enter choice:\n[1] Select Name\n[2] Select Stats");
        
        stdin.read_line(&mut buffer)
        .expect("ruh roh");

        choice = match buffer.trim().parse()
        {
            Ok(choice) => choice,
            Err(_) =>
            {
                println!("Invalid input!");
                return;
            }
        };

        match choice
        {
            1 => self.test(),
            2 => self.test(),
            _ => return,
        }

    }

    pub fn test(&mut self)
    {
        println!("Enter name: ");
        let stdin = io::stdin();
        stdin.read_line(&mut self.name)
        .expect("roh ruh");
    }
}


