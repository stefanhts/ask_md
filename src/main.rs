use inquire::{error::InquireError, Select, Text};
use std::{fmt, process::exit};

enum Source {
    GPT,
    Google,
    Internal,
}

enum Method {
    Output,
    Search,
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Source::GPT => write!(f, "GPT"),
            Source::Google => write!(f, "Google"),
            Source::Internal => write!(f, "Internal"),
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Method::Output => write!(f, "Explain Output"),
            Method::Search => write!(f, "Search"),
        }
    }
}

struct Response {
    val: String,
    temp: i32,
    source: String,
}

struct GoogleResult {
    question: String,
    answer: Option<Response>,
    related_links: Option<Vec<String>>,
}

struct InternalResult {
    question: String,
    matches: Option<Vec<Response>>,
}

struct GPTResult {
    question: String,
    answer: Response,
}

fn main() {
    let source: Source;
    let strategy: Method;

    let methods: Vec<Method> = vec![Method::Output, Method::Search];

    let method: Result<Method, InquireError> =
        Select::new("What type of question do you have?", methods).prompt();

    match method {
        Ok(method) => {
            strategy = method;
            println!("⚠️ DEBUG INFO: You asked for: {}", strategy);
        }
        Err(_) => {
            strategy = Method::Search;
            println!("🚨 There was an error, assuming {}", strategy);
        }
    }

    let action_opts: Vec<Source> = vec![Source::GPT, Source::Google, Source::Internal];

    let action: Result<Source, InquireError> =
        Select::new("How do you want to ask?", action_opts).prompt();

    match action {
        Ok(action) => {
            source = action;
            println!("⚠️ DEBUG INFO: 🔎 You want to search by: {}", &source);
        }
        Err(_) => {
            println!("🚨 There was an error ");
            exit(1);
        }
    }

    let question: String;
    match strategy {
        Method::Search => {
            let message = format!("What is your question for {}?", &source);
            let message = message.as_str();
            match Text::new(&message).prompt() {
                Ok(input) => question = input,
                Err(_) => {
                    println!("🚨 There was a fatal error. Exiting...");
                    exit(1);
                }
            }
            search(&question, source);
        }
        Method::Output => {
            let message = "temp result";
            explain(message, source);
        }
    }
}

fn search(question: &str, source: Source) {}
fn explain(question: &str, source: Source) {}

fn searchGoogle(term: &str) {}
