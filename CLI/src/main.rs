use colored::*;
use std::io::BufRead;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

fn main() {
    println!(
        "{}",
        "Welcome to Uri's Command Line Project in Rust - Enjoy!"
            .bold()
            .blue()
    );
    let mut current_dir = env::current_dir().unwrap();
    loop {
        // Print the current directory and prompt with blue text and a black background
        print!("{}> ", current_dir.display().to_string().blue().on_black());
        io::stdout().flush().unwrap();

        let input: String = get_input().to_lowercase();
        let mut args = input.split_whitespace();
        match args.next() {
            Some("exit") => break,
            Some("help") => print_help(),
            Some("grep") => {
                if let (Some(filename), Some(pattern)) = (args.next(), args.next()) {
                    if let Err(err) = grep(filename, pattern) {
                        println!("{}: {}", "Error during grep".red(), err);
                    }
                } else {
                    println!("{}", "Usage: grep <filename> <word_to_emphesize>".yellow());
                }
            }
            Some("find") => {
                if let Some(target) = args.next() {
                    if let Err(err) = find(&current_dir, target) {
                        println!("{}: {}", "Error during find".red(), err);
                    }
                } else {
                    println!("{}", "Usage: find <name>".yellow());
                }
            }
            Some("ls") | Some("dir") => {
                if let Err(err) = list_directory(&current_dir) {
                    println!("{}: {}", "Error listing directory".red(), err);
                }
            }
            Some("echo") => {
                let echo_text: String = args.collect::<Vec<&str>>().join(" ");
                println!("{}", echo_text.green());
            }
            Some("cd") => {
                if let Some(target) = args.next() {
                    if let Err(err) = change_directory(&mut current_dir, target) {
                        println!("{}: {}", "Error changing directory".red(), err);
                    }
                } else {
                    println!("{}", "Usage: cd <directory>".yellow());
                }
            }
            Some("pwd") => {
                println!("{}", current_dir.display().to_string().cyan());
            }
            Some("cat") => {
                if let Some(filename) = args.next() {
                    if let Err(err) = cat(&current_dir, filename) {
                        println!("{}: {}", "Error reading file".red(), err);
                    }
                } else {
                    println!("{}", "Usage: cat <filename>".yellow());
                }
            }
            Some("del") | Some("rm") => {
                if let Some(filename) = args.next() {
                    if let Err(err) = delete_file(&current_dir, filename) {
                        println!("{}: {}", "Error deleting file".red(), err);
                    }
                } else {
                    println!("{}", "Usage: del/rm <filename>".yellow());
                }
            }
            Some("rmdir") => {
                if let Some(dirname) = args.next() {
                    if let Err(err) = delete_directory(&current_dir, dirname) {
                        println!("{}: {}", "Error deleting directory".red(), err);
                    }
                } else {
                    println!("{}", "Usage: rmdir <directory>".yellow());
                }
            }
            Some("mv") => {
                if let (Some(src), Some(dest)) = (args.next(), args.next()) {
                    if let Err(err) = move_file(&current_dir, src, dest) {
                        println!("{}: {}", "Error moving file".red(), err);
                    }
                } else {
                    println!("{}", "Usage: mv <source> <destination>".yellow());
                }
            }
            Some("makefile") => {
                if let Some(filename) = args.next() {
                    let ext = args.next().unwrap_or("txt");
                    if let Err(err) = makefile(&mut current_dir, filename, ext) {
                        println!("{}: {}", "Error creating file".red(), err);
                    }
                } else {
                    println!("{}", "Usage: makefile <filename> [extension]".yellow());
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
    println!("{}", "--- HELP ---".bold().blue());
    println!("{}", "Available commands:".bold());
    println!(
        "{}",
        "1) grep - matches text in files - grep <filename> <word_to_emphesize>".yellow()
    );
    println!("{}", "2) echo - repeats input - echo <input>".yellow());
    println!(
        "{}",
        "3) find - locates files or directories - find <file/directory path>".yellow()
    );
    println!("{}", "4) ls/dir - lists directories - ls/dir".yellow());
    println!("{}", "5) exit - exits program - exit".yellow());
    println!(
        "{}",
        "6) help - prints explanation of all the commands and their format - help".yellow()
    );
    println!(
        "{}",
        "7) cd - change directory - cd <directory>, cd '..' to go up one directory".yellow()
    );
    println!("{}", "8) cat - prints a file - cat <filename>".yellow());
    println!(
        "{}",
        "9) del/rm - deletes a file - del/rm <filename>".yellow()
    );
    println!(
        "{}",
        "10) rmdir - deletes a directory - rmdir <directory>".yellow()
    );
    println!(
        "{}",
        "11) mv - moves a file - mv <source> <destination>".yellow()
    );
    println!("{}","12) makefile - creates a file with an optional extension - makefile <filename> [extension]".yellow());
}

fn list_directory(path: &Path) -> Result<(), io::Error> {
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
    println!("{}", contents.red());

    Ok(())
}

fn grep(filename: &str, pattern: &str) -> Result<(), io::Error> {
    let file = fs::File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut count = 0;

    for (index, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(pattern) {
            count += 1;
            println!("{}: {}", (index + 1).to_string().bold().magenta(), line);
        }
    }

    println!(
        "{}: {}",
        "Occurrences of".bold().green(),
        count.to_string().bold().green()
    );
    Ok(())
}

fn delete_file(current_dir: &Path, filename: &str) -> Result<(), io::Error> {
    let mut file_path = current_dir.to_path_buf();
    file_path.push(filename);

    if file_path.is_file() {
        fs::remove_file(file_path)?;
        println!("{}", "File deleted successfully".green());
        Ok(())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "File not found"))
    }
}

fn delete_directory(current_dir: &Path, dirname: &str) -> Result<(), io::Error> {
    let mut dir_path = current_dir.to_path_buf();
    dir_path.push(dirname);

    if dir_path.is_dir() {
        fs::remove_dir_all(dir_path)?;
        println!("{}", "Directory deleted successfully".green());
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Directory not found",
        ))
    }
}

fn move_file(current_dir: &Path, src: &str, dest: &str) -> Result<(), io::Error> {
    let mut src_path = current_dir.to_path_buf();
    src_path.push(src);

    let dest_path = PathBuf::from(dest);

    if src_path.is_file() {
        fs::rename(src_path, dest_path)?;
        println!("{}", "File moved successfully".green());
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Source file not found",
        ))
    }
}

fn makefile(current_dir: &mut PathBuf, filename: &str, ext: &str) -> Result<(), io::Error> {
    let mut file_path = current_dir.clone();
    file_path.push(format!("{}.{}", filename, ext));

    // Create an empty file
    let file = fs::File::create(&file_path)?;

    println!(
        "{} {}",
        "File created successfully:".green(),
        file_path.display()
    );

    Ok(())
}
