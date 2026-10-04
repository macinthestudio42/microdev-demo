use microdev_demo::Checklist;
use std::{fs, process::ExitCode};

const USAGE: &str = "usage: microdev-demo [list | add TITLE | done NUMBER] [--file PATH]";

fn main() -> ExitCode {
    match run(std::env::args().skip(1).collect()) {
        Ok(output) => {
            print!("{output}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(mut args: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
    let path = match args.iter().position(|arg| arg == "--file") {
        Some(at) if at + 1 < args.len() => {
            let path = args.remove(at + 1);
            args.remove(at);
            path
        }
        Some(_) => return Err(USAGE.into()),
        None => "TODO.md".to_owned(),
    };
    let mut list = Checklist::parse(&fs::read_to_string(&path).unwrap_or_default());
    let words: Vec<&str> = args.iter().map(String::as_str).collect();
    match words.as_slice() {
        [] | ["list"] => {}
        ["add", title] => {
            list.add(title)?;
            fs::write(&path, list.render())?;
        }
        ["done", number] => {
            list.complete(number.parse().map_err(|_| USAGE)?)?;
            fs::write(&path, list.render())?;
        }
        _ => return Err(USAGE.into()),
    }
    let (done, all) = list.progress();
    let mut output: String = list
        .items()
        .map(|(number, title, done)| {
            format!("{number:>2}. [{}] {title}\n", if done { 'x' } else { ' ' })
        })
        .collect();
    output.push_str(&format!("{done}/{all} done\n"));
    Ok(output)
}
