use dirs::home_dir;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

fn main() {
    println!("This is Uri's command line project in Rust - enjoy");
    let current_dir = env::current_dir().unwrap();
    loop {
        let input: String = get_input().to_lowercase();
        let mut args = input.split(" ");
        match args.next() {
            Some("exit") => break,
            Some("help") => print_help(),
            Some("grep") => {}
            Some("find") => {
                if let Some(target) = args.next() {
                    if let Err(err) = find(target) {
                        println!("Error during find: {}", err);
                    }
                } else {
                    println!("Usage: find <name>");
                }
            }
            Some("ls") | Some("dir") => {
                if let Err(err) = list_directory(&current_dir) {
                    println!("Error listing directory - {}", err);
                }
            }
            Some("echo") => {}
            Some("cd") => {}
            Some("pwd") => {
                println!("{}", current_dir.display());
            }
            Some("cat") => {}
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
    _______________________________
    1)grep - matches text in files - grep <filename> <word_to_emphesize>\n
    2)echo - reapets input - echo <input>\n
    3)find - locates files or directories - find <file/directory path>\n
    4)ls/dir - lists directories - ls/dir \n
    5)exit/EXIT - exits program - exit/EXIT \n
    6)help/HELP - prints explanation of all the commands and their format - help/HELP\n
    7)cd - change directory - cd  <directory> , cd '..' to get out of the current directory\n
    8)cat - prints a file - cat <filename> \n"
    )
}

//fn grep(File: String , target:) {} still don't know the type of this..

fn list_directory(path: &PathBuf) -> Result<(), io::Error> {
    let entries = fs::read_dir(path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    for entry in entries {
        println!("{}", entry.display());
    }

    Ok(())
}

fn find(target: &str) -> Result<(), io::Error> {
    if let Some(home_path) = home_dir() {
        search_directory(&home_path, target)
    } else {
        println!("Could not find the home directory.");
        Ok(())
    }
}

fn search_directory(root: &Path, target: &str) -> Result<(), io::Error> {
    if root.is_dir() {
        for entry in fs::read_dir(root)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                // If it's a directory, search recursively
                if let Err(err) = search_directory(&path, target) {
                    println!("Error searching directory {}: {}", path.display(), err);
                }
            }
            if let Some(file_name) = path.file_name() {
                if file_name.to_string_lossy().contains(target) {
                    println!("{}", path.display());
                }
            }
        }
    }
    Ok(())
}
