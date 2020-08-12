mod errors;
use errors::{ShellError, ShellResult};

mod builtins;

use std::{
    env::{self, current_dir},
    io::{self, stdin, stdout, Write},
    path::Path,
};

fn main() -> Result<(), io::Error> {
    loop {
        prompt()?;

        let line = read_line()?;

        if line.is_empty() {
            // EOF
            break;
        }

        match eval(&line) {
            Ok(Some(Action::Exit)) => break,
            Ok(None) => continue,
            Err(e) => eprintln!("{}", e),
        }
    }

    Ok(())
}

fn prompt() -> Result<(), io::Error> {
    let cwd = current_dir()?;
    print!("{} $ ", cwd.display());
    stdout().flush()?;
    Ok(())
}

fn read_line() -> Result<String, io::Error> {
    let mut line = String::new();
    stdin().read_line(&mut line)?;
    Ok(line)
}

enum Action {
    Exit,
}

fn eval(line: &str) -> ShellResult<Option<Action>> {
    let mut parts = line.split_whitespace();

    let command = match parts.next() {
        Some(c) => c,
        None => return Ok(None),
    };

    let args = parts.collect::<Vec<_>>();

    match command {
        // "ls" => builtins::ls(&args)?,
        // "pwd" => builtins::pwd()?,
        "cd" => builtins::cd(&args)?,
        "exit" => return Ok(Some(Action::Exit)),
        c => find_and_run(c, &args)?,
    }

    Ok(None)
}

fn find_and_run(command: &str, args: &[&str]) -> ShellResult<()> {
    let path_env_var = env::var("PATH").unwrap_or(String::new());
    let directories = path_env_var.split(":").map(Path::new);

    let executable = directories
        .map(|dir| dir.join(command))
        .filter(|file| file.exists())
        .next();

    if let Some(exe) = executable {
        std::process::Command::new(exe).args(args).spawn()?.wait()?;
    } else {
        return Err(ShellError::command_not_found(command));
    }

    Ok(())
}
