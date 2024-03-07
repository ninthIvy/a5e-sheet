use easy_repl::{command, CommandStatus, Repl};
mod libcharacter;
use libcharacter::Character;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::path::PathBuf;
use bincode::{serialize, deserialize};

fn main() {
    let my_character = Arc::new(Mutex::new(Character::default()));
    let my_character_ref = Arc::clone(&my_character);

    let mut repl = Repl::builder()
        .add("echo", command! 
        {
            "echo contents to screen",  
            (contents: String) => |contents| 
            {
                println!("{}", contents);
                Ok(CommandStatus::Done)
            }
        })
        .add("create", command! 
        {
            "Create a character",
            (strength: u8, dex: u8, con: u8, wis: u8, intel: u8, cha: u8) => |strength, dex, con, wis, intel, cha| 
            {
                let mut character = my_character_ref.lock().unwrap();
                character.init(strength, dex, con, wis, intel, cha);
                println!("Stats created: {:?}", character.stats);
                Ok(CommandStatus::Done)
            }
        })
        .add("save", 
        {
            let my_character_ref = Arc::clone(&my_character);
            command! 
            {
                "save character to file",
                (name: String) => |name:String| 
                {
                    let mut stringbean = String::from(name);
                    stringbean.push_str(".chr");
                    let mut file = File::create(&stringbean).expect("Failed to create file.");

                    let character = my_character_ref.lock().unwrap();
                    

                    let encoded: Vec<u8> = serialize(&*character).expect("Serialization failed");
                    file.write_all(&encoded).expect("failed write");
                    println!("Character data has been written to {}", stringbean);
                    

                    Ok(CommandStatus::Done)
                }
            }
        })
        .add("load", 
        {
            let my_character_ref = Arc::clone(&my_character);
            command! 
            {
                "save character to file",
                (name: String) => |name:String| 
                {
                    let mut stringbean = String::from(name);
                    stringbean.push_str(".chr");
                    let mut file = File::open(&stringbean).expect("Failed to create file.");

                    //let character = my_character_ref.lock().unwrap();
                    

                    let mut decoded = Vec::new();
                    file.read_to_end(&mut decoded).expect("failed write");
                    let character: Character = bincode::deserialize(&decoded).expect("Deserialization failed");

                    let mut character_lock = my_character_ref.lock().unwrap();
                    *character_lock = character;

                    println!("Character data has been written to {}", stringbean);
                    

                    Ok(CommandStatus::Done)
                }
            }
        })
        .add("stats", 
        {
            let my_character_ref = Arc::clone(&my_character);
            command! 
            {
                "read your stats",
                //arguments go below, but are purposefully left empty
                () => || 
                {
                    let character = my_character_ref.lock().unwrap();
                    for (index, &stat) in character.stats.iter().enumerate() 
                    {
                        println!("Stat {}: {}", index + 1, stat);
                    }
                    Ok(CommandStatus::Done)
                } 
            }
        })
        .build().expect("Failed to create repl");

    repl.run().expect("Critical REPL error");
}
