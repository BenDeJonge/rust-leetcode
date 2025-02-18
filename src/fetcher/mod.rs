mod difficulty;
mod fetch;
mod file_writer;
mod question;

use std::{env, error::Error, fmt::Display};

#[derive(Debug)]
struct ArgCountError {}

impl Error for ArgCountError {}

impl Display for ArgCountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Incorrect argument count")
    }
}

fn unpack_args(args: &[String]) -> Result<&str, ArgCountError> {
    match args {
        [_main, _fname, arg1] => Ok(arg1.as_str()),
        _ => Err(ArgCountError {}),
    }
}
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();
    let name = unpack_args(&args).expect("expected 1 argument: <problem-name>");
    println!("Querrying {name}");
    match fetch::fetch_question_async(name).await {
        Ok(q) => {
            println!("Retrieved question");
            if q.code_definition.is_none() {
                println!("No code definition present");
            }
            println!("Creating Rust code example");
            file_writer::create_code_file(&q);
        }
        Err(e) => {
            dbg!(e);
        }
    };
    Ok(())
}
