use std::io;
use serde::{Serialize, Deserialize};
#[derive(Default, Serialize, Deserialize)]
pub struct Character {
    pub stats: [u8; 6],
    pub name: String,
}

impl Character {
    pub fn pass(&mut self)
    {
        return;
    }

    pub fn set_stat(&mut self, stat_number:usize)
    {
        let mut attribute = String::new();
        match stat_number
        {
            1 => attribute = String::from("Strength"),
            2 => attribute = String::from("Dexterity"),
            3 => attribute = String::from("Constitution"),
            4 => attribute = String::from("Intelligence"),
            5 => attribute = String::from("Wisdom"),
            6 => attribute = String::from("Charisma"),
            _ => self.pass(),
        }
        println!("Enter stat for {attribute}: ");
        let mut buffer = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut buffer)
        .expect("ruh roh");

        let stat: u8;
        stat = match buffer.trim().parse()
        {
            Ok(stat) => stat,
            Err(_) =>
            {
                return;
            }
        };

        self.stats[stat_number-1] = stat;
    }

    pub fn init(&mut self) {

        let mut i:u8 = 1;
        
        while i <= 100
        {     

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
                    //println!("Invalid input!");
                    return;
                }
            };

            match choice
            {
                1 => self.set_name(),
                2 => self.set_stats(),
                _ => return,
            }
            i += 1;
        }

    }

    pub fn set_name(&mut self)
    {
        println!("Enter name: ");
        let stdin = io::stdin();
        stdin.read_line(&mut self.name)
        .expect("roh ruh");
    }

    pub fn set_stats(& mut self)
    {
        let mut i: u8 = 1;
        while i <= 100
        {
            let mut buffer = String::new();
            let stdin = io::stdin();
            let choice: usize;

            println!("[1] Str\n[2] Dex\n[3] Con\n[4] Int\n[5] Wis\n[6] Cha\n");
            
            stdin.read_line(&mut buffer).expect("ruh roh");

            choice = match buffer.trim().parse()
            {
                Ok(choice) => choice,
                Err(_) =>
                {
                    //println!("Invalid input!");
                    return;
                }
            };

            self.set_stat(choice);
            i += 1;
        }

        println!("setting stats")
    }
}


