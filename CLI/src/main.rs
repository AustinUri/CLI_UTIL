use std::path::{Path, PathBuf};
use std::{env, fs, io};
use walkdir::WalkDir;
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
                if let Some(target) = args.next() {
                    if let Err(err) = find(target) {
                        println!("Error during find: {}", err);
                    }
                } else {
                    println!("Usage: find <name>");
                }
            }
            Some("echo") => {
                let echo_text: String = args.collect::<Vec<&str>>().join(" ");
                println!("{}", echo_text);
            }
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

fn list_directory(path: &str) -> Result<(), io::Error> {
    let entries = WalkDir::new(path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok());

    for entry in entries {
        println!("{}", entry.path().display());
    }

    Ok(())
}

fn find(target: &str) -> Result<(), io::Error> {
    let entries = WalkDir::new("/")
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok());

    for entry in entries {
        let path = entry.path();
        if let Some(file_name) = path.file_name() {
            if file_name.to_string_lossy().contains(target) {
                println!("{}", path.display());
            }
        }
    }

    Ok(())
}
