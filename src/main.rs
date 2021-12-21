use std::error::Error;
use std::env;

mod lib;
use lib::*;

fn main() -> Result<(), Box<dyn Error>> {
    // Recollect the enviroment arguments
    let args: Vec<String> = env::args().collect();
    let command = argparse().unwrap();
    // Before jumping into the match arm, first check if the command is a config command
    if command == Arg::SETKEY {
        // Set the API key
        keys::set_key(&args[3]).unwrap();
        Ok(())
    }
    else if command == Arg::GETKEY {
        let key = keys::get_key()?;
        println!("key: {}", key);
        Ok(())
    }

    // Command is not a config command - match on command and output the response
    else {
        let res: String;
        match command {
            Arg::SFLARE => res = weather::sflare().unwrap(),
            Arg::MAG => res = weather::magnetic().unwrap(),
            Arg::APOD => res = apod().unwrap(),
            Arg::NEO => res = neo().unwrap(),
            // Default to apod if command can't be parsed
            Arg::BADCOMMAND => res = apod().unwrap(),
            _ => panic!()
        }

        if command == Arg::BADCOMMAND {
            println!("\nDefaulted to APOD upon Bad Command\n\n")
        }
        Ok(())
    }


}
