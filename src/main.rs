use std::{collections::HashMap, io::BufRead};

use chrono::Local;
use clap::Parser;
use enigo::{self, KeyboardControllable};
use regex::{self, Regex};

/// A small utility for writing text via a virtual keyboard.
#[derive(Parser)]
struct App {
    /// This is a seris of string that use the enigo syntax along with our
    /// fmt statements to allow you to write formatted string that are then
    /// converted into keyboard output.
    statments: Vec<String>,
}

///The enviroment variable resolver takes the value passed as an ar and
/// Asks the operating system is there any string with that value.
fn env_resolver(args: String) -> Option<String> {
    return match std::env::var(args) {
        Ok(value) => Some(value),
        Err(_) => None,
    };
}

///takes a format string formmatted as specifed in
///https://docs.rs/chrono/0.4.24/chrono/format/strftime/index.html#specifiers
///Returns a string formatted to that specification.
fn now_resolver(args: String) -> Option<String> {
    let date = Local::now();
    return Some(date.format(&args).to_string());
}

fn parse_statement(statment: &String) -> String {
    // Lets make use of rusts regex capture groups to define the pattern we
    // want to match and pull out the relavent infomation.
    let re = Regex::new(r"\?(?P<resolver>.*)\((?P<args>.*)\)").unwrap();
    let mut return_value = statment.clone();

    let mut replacement_map: HashMap<String, String> = HashMap::new();
    let mut resolver_map: HashMap<String, fn(args: String) -> Option<String>> = HashMap::new();

    resolver_map.insert("env".to_string(), env_resolver);
    resolver_map.insert("now".to_string(), now_resolver);

    for capture in re.captures_iter(&statment) {
        let orginal_name = format!(
            "?{}({})",
            capture.name("resolver").unwrap().as_str(),
            capture.name("args").unwrap().as_str()
        );

        if let Some(resolver) = resolver_map.get(
            &capture
                .name("resolver")
                .unwrap()
                .as_str()
                .to_ascii_lowercase(),
        ) {
            let replacement_text =
                match resolver(capture.name("args").unwrap().as_str().to_string().clone()) {
                    Some(text) => text,
                    None => orginal_name.clone(),
                };

            replacement_map.insert(orginal_name, replacement_text);
        }

        for (key, value) in replacement_map.iter() {
            return_value = return_value.replace(key, value);
        }
    }
    return return_value;
}

fn main() -> Result<(), String> {
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
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        }
    }

    let mut enigo = enigo::Enigo::new();

    for txt in text {
        if let Err(e) = enigo.key_sequence_parse_try(&txt) {
            return Err(e.to_string());
        }
    }

    return Ok(());
}

#[cfg(test)]
mod test {

    /// NOTE: Due to the fact that the follow resolvers are simple pass throughs
    /// to other implmenteations we are not going to bother testing it unless
    /// for some reason they turn out to be so buggy we really need to
    /// 
    /// 1. env
    /// 2. now
    /// 

    #[test]
    fn parse_statement() {
        let test_cases = [
            ///Testing that the parse statement function does not modify text it not meant to
            ("Hello world!".to_string(), ("Hello world!".to_string())),
            /// Testing that in cases where text matches the pattern but is not
            /// meant to be resolved it is left as it was found
            (
                "Hello world? (this could be a joke)".to_string(),
                "Hello world? (this could be a joke)".to_string(),
            ),
            ///Testing that when we want it to resolve a value it does so as expected
            (
                "Hello world ?env(TEST)".to_string(),
                "Hello world TEST".to_string(),
            ),
            ///Testing that when it can not resolve a value it leaves it as it finds it
            (
                "Hello world ?env(UNDEFINED_TEST)".to_string(),
                "Hello world ?env(UNDEFINED_TEST)".to_string(),
            ),
            ///Testing that the now resolve is correctly located and called.
            (
                "Hello world ?now(%D)".to_string(),
                format!("Hello world {}",chrono::Local::now().format("%D"))
            ),
        ];
        std::env::set_var("TEST", "TEST");
        for test_case in test_cases {
            let (input, expected_ouput) = test_case;
            assert_eq!(crate::parse_statement(&input), expected_ouput)
        }
    }
}
