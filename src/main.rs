use std::{error::Error, fs};

use tu4::Machine;

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args().nth(1).unwrap();
    let mut machine: Machine = fs::read_to_string(filename)?.parse()?;
    let input = std::io::stdin().lines().next().unwrap()?;
    machine.set_input(&input);
    machine.execute_with_callback(|tape, state, next| {
        println!("state: {state} => {next}");
        tape.display();
    });
    Ok(())
}
