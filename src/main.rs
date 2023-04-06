use std::io::BufRead;

use clap::Parser;
use enigo::{self, KeyboardControllable};
/// A small utility for writing text via a virtual keyboard.
#[derive(Parser)]
struct App {
    /// This is a seris of string that use the enigo syntax along with our
    /// fmt statements to allow you to write formatted string that are then 
    /// converted into keyboard output.
    statments: Vec<String>
}

fn parse_statement(statment: &String) -> String {
    todo!("add string parsing")
}

fn main() -> Result<(),String> {
    let app = App::parse();
    let mut text = Vec::new();
    if app.statments.len() > 0 {
        for statement in app.statments.iter() {
            text.push(parse_statement(statement));
        }
    } else {
        let stdin = std::io::stdin();
        let mut lines = stdin.lock().lines();
        while let Some(line) = lines.next() {
            match line {
                Ok(statement) => {
                    text.push(parse_statement(&statement));
                }
                Err(e) =>{
                    return Err(e.to_string());
                }
            }
        }
    }
    
    let mut enigo = enigo::Enigo::new();

    for txt in text {
        if let Err(e) = enigo.key_sequence_parse_try(&txt) {
            return Err(e.to_string())
        }
    }

    return Ok(());
}
