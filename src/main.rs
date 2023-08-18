use inquire::{error::InquireError, Select, Text};
use std::io;
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
enum Confirm {
    Yes,
    No,
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

impl fmt::Display for Confirm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Confirm::Yes => write!(f, "Yes"),
            Confirm::No => write!(f, "No"),
        }
    }
}

struct Response {
    val: String,
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
            println!("🚨 There was a fatal error ");
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
            question = String::from("temp");
            explain(message, source);
        }
    }

    search_gpt(question);
}

fn search(question: &str, source: Source) {}
fn explain(question: &str, source: Source) {}
fn ask_question_gpt(q: &String) -> String {
    return format!("This is my answer to {}", q);
}
fn search_google(term: &str) {}
fn search_internal(term: &str) {}
fn search_gpt(term: String) {
    let mut question = &term;
    // TODO send the question
    let mut reply = ask_question_gpt(question);
    let mut ans = GPTResult {
        question: term,
        answer: Response {
            val: reply,
            source: Source::GPT.to_string(),
        },
    };
    loop {
        println!("{} Says:\n\t{}", ans.answer.source, ans.answer.val);
        let _ask = Text::new("").prompt();
        match _ask {
            Ok(_a) => reply = ask_question_gpt(&_a),
            Err(_) => {
                println!("Exiting...");
                exit(0);
            }
        }
        ans.answer.val = reply;
    }
}
