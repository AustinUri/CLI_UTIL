use colored::*;
use std::io::BufRead;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

fn main() {
    println!("This is Uri's command line project in Rust - enjoy");
    let mut current_dir = env::current_dir().unwrap();
    loop {
        print!("{}> ", current_dir.display().to_string().blue());
        io::stdout().flush().unwrap();
        let input: String = get_input().to_lowercase();
        let mut args = input.split(" ");
        match args.next() {
            Some("exit") => break,
            Some("help") => print_help(),
            Some("grep") => {
                if let (Some(filename), Some(pattern)) = (args.next(), args.next()) {
                    if let Err(err) = grep(filename, pattern) {
                        println!("Error during grep: {}", err);
                    }
                } else {
                    println!("Usage: grep <filename> <word_to_emphesize>");
                }
            }
            Some("find") => {
                if let Some(target) = args.next() {
                    if let Err(err) = find(&current_dir, target) {
                        println!("Error during find: {}", err);
                    }
                } else {
                    println!("Usage: find <name>");
                }
            }
            Some("ls") | Some("dir") => {
                if let Err(err) = list_directory(".") {
                    println!("Error listing directory - {}", err);
                }
            }
            Some("echo") => {
                let echo_text: String = args.collect::<Vec<&str>>().join(" ");
                println!("{}", echo_text);
            }
            Some("cd") => {
                if let Some(target) = args.next() {
                    //println!("{}", target);
                    if let Err(err) = change_directory(&mut current_dir, target) {
                        println!("Error during find: {}", err);
                    }
                } else {
                    println!("Usage: cd <path/..>");
                }
            }
            Some("pwd") => {
                println!("{}", current_dir.display());
            }
            Some("cat") => {
                if let Some(target) = args.next() {
                    if let Err(err) = cat(&current_dir, target) {
                        println!("Error during find: {}", err);
                    }
                } else {
                    println!("Usage: find <name>");
                }
            }
            _ => print_help(),
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn print_help() {
    println!(
        "---HELP--- \n
    the command - explanation - format
    ___________________________________
    1)grep - matches text in files - grep <filename> <word_to_emphesize>\n
    2)echo - reapets input - echo <input>\n
    3)find - locates files or directories - find <file/directory path>\n
    4)ls/dir - lists directories - ls/dir \n
    5)exit/EXIT - exits program - exit/EXIT \n
    6)help/HELP - prints explanation of all the commands and their format - help/HELP\n
    7)cd - change directory - cd  <directory> , cd '..' to get out of the current directory\n
    8)cat - prints a file - cat <filename> \n
    9)mv - moves a file location(from current dir to another - mv <filename> <target_repo>\n
    10)rm - deletes a file - rm <filename> \n
    11)rmdir - deletes a repository(needs to have access) - rmdir <path> \n
    12)cratefile - creates a file - \n
    13)rename - renames a file or a repo - rename <file/repo> <new_name>\n
    "
    )
}

fn grep(filename: &str, pattern: &str) -> Result<(), io::Error> {
    let file = fs::File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut count = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(pattern) {
            count += 1;
            println!("{}: {}", index + 1, line);
        }
    }

    println!("Occurrences of '{}': {}", pattern, count);
    Ok(())
}
fn list_directory(path: &str) -> Result<(), io::Error> {
    let entries = fs::read_dir(path)?;

    for entry in entries {
        let entry = entry?;
        println!("{}", entry.path().display());
    }

    Ok(())
}

fn find(path: &Path, target: &str) -> Result<(), io::Error> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                find(&path, target)?;
            } else if path.file_name().unwrap().to_string_lossy().contains(target) {
                println!("{}", path.display());
            }
        }
    }
    Ok(())
}

fn change_directory(current_dir: &mut PathBuf, target: &str) -> Result<(), io::Error> {
    let new_dir = if target == ".." {
        current_dir.pop();
        current_dir.clone()
    } else {
        let mut new_path = current_dir.clone();
        new_path.push(target);
        new_path
    };

    if new_dir.is_dir() {
        env::set_current_dir(&new_dir)?;
        *current_dir = env::current_dir()?;
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Directory not found",
        ))
    }
}

fn cat(current_dir: &Path, filename: &str) -> Result<(), io::Error> {
    let mut file_path = current_dir.to_path_buf();
    file_path.push(filename);

    let contents = fs::read_to_string(file_path)?;
    println!("{}", contents.red().on_black());

    Ok(())
}

//fn rm

//fn createFile

//fn rename

//fn move
