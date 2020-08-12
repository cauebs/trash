use crate::errors::{ShellError, ShellResult};

use std::{
    env::{current_dir, set_current_dir},
    path::PathBuf,
};


pub fn ls(args: &[&str]) -> ShellResult<()> {
    let dir = args.first().map(PathBuf::from).unwrap_or(current_dir()?);

    let mut entries: Vec<_> = dir.read_dir()?.filter_map(Result::ok).collect();

    entries.sort_by_key(|entry| {
        (
            entry
                .file_type()
                .map(|e| if e.is_dir() { 0 } else { 1 })
                .unwrap_or(2),
            entry.file_name(),
        )
    });

    for entry in entries {
        println!("{}", entry.file_name().to_string_lossy());
    }

    Ok(())
}

pub fn pwd() -> ShellResult<()> {
    let cwd = current_dir()?;
    println!("{}", cwd.display());
    Ok(())
}

pub fn cd(args: &[&str]) -> ShellResult<()> {
    if args.len() > 2 {
        let error_message = format!("expected zero or one argument, got {}", args.len());
        return Err(ShellError::invalid_args("cd", &error_message));
    }

    let cwd = current_dir()?;
    let arg = args.first().unwrap_or(&"..");
    set_current_dir(cwd.join(arg))?;

    Ok(())
}
