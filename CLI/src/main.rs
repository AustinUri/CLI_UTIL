use colored::*;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let mut current_dir = std::env::current_dir()?;

    println!(
        "{}",
        "Welcome to Uri's Command Line Project in Rust - Enjoy!"
            .bold()
            .blue()
    );

    loop {
        print!("{}> ", current_dir.display().to_string().blue().on_black());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let mut args = input.split_whitespace();
        let command = args.next();

        match command {
            Some("exit") => break,
            Some("help") => print_help(),
            Some("grep") => handle_command(&mut args, |args| {
                let filename = args.next().ok_or("Usage: grep <filename> <pattern>")?;
                let pattern = args.next().ok_or("Usage: grep <filename> <pattern>")?;
                grep(filename, pattern)?;
                Ok(())
            })?,
            Some("find") => handle_command(&mut args, |args| {
                let target = args.next().ok_or("Usage: find <target>")?;
                find(&current_dir, target)?;
                Ok(())
            })?,
            Some("ls") | Some("dir") => handle_command(&mut args, |_| {
                list_directory(&current_dir)?;
                Ok(())
            })?,
            Some("echo") => handle_command(&mut args, |args| {
                let echo_text: String = args.collect::<Vec<&str>>().join(" ");
                println!("{}", echo_text.green());
                Ok(())
            })?,
            Some("cd") => handle_command(&mut args, |args| {
                let target = args.next().ok_or("Usage: cd <directory>")?;
                change_directory(&mut current_dir, target)?;
                Ok(())
            })?,
            Some("pwd") => handle_command(&mut args, |_| {
                println!("{}", current_dir.display().to_string().cyan());
                Ok(())
            })?,
            Some("cat") => handle_command(&mut args, |args| {
                let filename = args.next().ok_or("Usage: cat <filename>")?;
                cat(&current_dir, filename)?;
                Ok(())
            })?,
            Some("del") | Some("rm") => handle_command(&mut args, |args| {
                let filename = args.next().ok_or("Usage: del/rm <filename>")?;
                delete_file(&current_dir, filename)?;
                Ok(())
            })?,
            Some("rmdir") => handle_command(&mut args, |args| {
                let dirname = args.next().ok_or("Usage: rmdir <dirname>")?;
                delete_directory(&current_dir, dirname)?;
                Ok(())
            })?,
            Some("mv") => handle_command(&mut args, |args| {
                let src = args.next().ok_or("Usage: mv <src> <dest>")?;
                let dest = args.next().ok_or("Usage: mv <src> <dest>")?;
                move_file(&current_dir, src, dest)?;
                Ok(())
            })?,
            Some("makefile") => handle_command(&mut args, |args| {
                let filename = args
                    .next()
                    .ok_or("Usage: makefile <filename> [extension]")?;
                let ext = args.next().unwrap_or("txt");
                makefile(&mut current_dir, filename, ext)?;
                Ok(())
            })?,
            _ => println!(
                "{}",
                "Unknown command. Type 'help' for a list of commands.".red()
            ),
        }
    }

    Ok(())
}

fn handle_command<F>(args: &mut std::str::SplitWhitespace, f: F) -> Result<(), Box<dyn Error>>
where
    F: FnOnce(&mut std::str::SplitWhitespace) -> Result<(), Box<dyn Error>>,
{
    match f(args) {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("{}", err.to_string().red());
            Err(err)
        }
    }
}

fn print_help() {
    println!("{}", "--- HELP ---".bold().blue());
    println!("{}", "Available commands:".bold());
    println!(
        "{}",
        "1) grep - matches text in files - grep <filename> <word_to_emphasize>".yellow()
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
    println!(
        "{}",
        "12) makefile - creates a file with an optional extension - makefile <filename> [extension]"
            .yellow()
    );
}

fn grep(filename: &str, pattern: &str) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let mut count = 0;

    for (index, line) in contents.lines().enumerate() {
        if line.contains(pattern) {
            println!("{}: {}", (index + 1).to_string().bold().magenta(), line);
            count += 1;
        }
    }

    println!(
        "{}: {}",
        "Occurrences of".bold().green(),
        count.to_string().bold().green()
    );

    Ok(())
}

fn find(current_dir: &PathBuf, target: &str) -> Result<(), Box<dyn Error>> {
    let target_path = Path::new(target);

    // Check if the target is a full path
    if target_path.is_absolute() {
        if target_path.exists() {
            println!("{}", target_path.display().to_string().green());
        } else {
            println!("Path does not exist: {}", target);
        }
        return Ok(());
    }

    // Otherwise, perform a search in the current directory and subdirectories
    let entries = match fs::read_dir(current_dir) {
        Ok(entries) => entries,
        Err(e) => {
            if e.kind() == io::ErrorKind::PermissionDenied {
                return Ok(());
            } else {
                return Err(e.into());
            }
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                if e.kind() == io::ErrorKind::PermissionDenied {
                    continue;
                } else {
                    return Err(e.into());
                }
            }
        };

        let path = entry.path();
        if path.is_dir() {
            find(&path, target)?;
        } else if path
            .file_name()
            .and_then(|name| name.to_str())
            .map_or(false, |name| name.contains(target))
        {
            println!("{}", path.display().to_string().green());
        }
    }

    Ok(())
}

fn list_directory(current_dir: &PathBuf) -> Result<(), Box<dyn Error>> {
    let entries = fs::read_dir(current_dir)?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        println!("{}", path.display());
    }

    Ok(())
}

fn change_directory(current_dir: &mut PathBuf, target: &str) -> Result<(), Box<dyn Error>> {
    if target == ".." {
        current_dir.pop();
    } else {
        current_dir.push(target);
        if !current_dir.is_dir() {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::NotFound,
                "Directory not found",
            )));
        }
    }

    Ok(())
}

fn cat(current_dir: &PathBuf, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut file_path = current_dir.clone();
    file_path.push(filename);

    let contents = fs::read_to_string(file_path)?;
    println!("{}", contents.red());

    Ok(())
}

fn delete_file(current_dir: &PathBuf, filename: &str) -> Result<(), Box<dyn Error>> {
    let mut file_path = current_dir.clone();
    file_path.push(filename);

    fs::remove_file(file_path)?;
    println!("{}", "File deleted successfully".green());

    Ok(())
}

fn delete_directory(current_dir: &PathBuf, dirname: &str) -> Result<(), Box<dyn Error>> {
    let mut dir_path = current_dir.clone();
    dir_path.push(dirname);

    // Ensure we are not deleting the current directory
    if dir_path == *current_dir {
        return Err("Cannot delete the current working directory".into());
    }

    fs::remove_dir_all(dir_path)?;
    println!("{}", "Directory deleted successfully".green());

    Ok(())
}

fn move_file(current_dir: &PathBuf, src: &str, dest: &str) -> Result<(), Box<dyn Error>> {
    let mut src_path = current_dir.clone();
    src_path.push(src);

    let mut dest_path = current_dir.clone();
    dest_path.push(dest);

    // Check if destination is a directory
    if dest_path.is_dir() {
        dest_path.push(src); // Move the file into the directory
    } else {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Destination is not a directory",
        )));
    }

    fs::rename(src_path, dest_path)?;
    println!("{}", "File moved successfully".green());

    Ok(())
}

fn makefile(current_dir: &mut PathBuf, filename: &str, ext: &str) -> Result<(), Box<dyn Error>> {
    let mut file_path = current_dir.clone();
    file_path.push(format!("{}.{}", filename, ext));
    fs::File::create(file_path)?;
    println!(
        "{}",
        format!("File {}.{} created successfully", filename, ext)
            .green()
            .bold()
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_grep() {
        let temp_file = "test_grep.txt";
        fs::write(temp_file, "Hello\nRust\nWorld\n").unwrap();
        let result = grep(temp_file, "Rust");
        fs::remove_file(temp_file).unwrap();
        assert!(result.is_ok());
    }

    #[test]
    fn test_find() {
        let current_dir = env::current_dir().unwrap();
        let result = find(&current_dir, "Cargo.toml");
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_directory() {
        let current_dir = env::current_dir().unwrap();
        let result = list_directory(&current_dir);
        assert!(result.is_ok());
    }

    #[test]
    fn test_change_directory() {
        let mut current_dir = env::current_dir().unwrap();
        let result = change_directory(&mut current_dir, "..");
        assert!(result.is_ok());
    }

    #[test]
    fn test_cat() {
        let temp_file = "test_cat.txt";
        fs::write(temp_file, "Hello, world!").unwrap();
        let current_dir = env::current_dir().unwrap();
        let result = cat(&current_dir, temp_file);
        fs::remove_file(temp_file).unwrap();
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_file() {
        let temp_file = "test_delete_file.txt";
        fs::write(temp_file, "Hello, world!").unwrap();
        let current_dir = env::current_dir().unwrap();
        let result = delete_file(&current_dir, temp_file);
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_directory() {
        let temp_dir = "test_delete_directory";
        fs::create_dir(temp_dir).unwrap();
        let current_dir = env::current_dir().unwrap();
        let result = delete_directory(&current_dir, temp_dir);
        assert!(result.is_ok());
    }

    #[test]
    fn test_move_file() {
        let src_file = "test_src_file.txt";
        let dest_dir = "test_dest_directory";
        fs::write(src_file, "Hello, world!").unwrap();
        fs::create_dir(dest_dir).unwrap();
        let current_dir = env::current_dir().unwrap();
        let result = move_file(&current_dir, src_file, dest_dir);
        assert!(result.is_ok());
        fs::remove_file(format!("{}/{}", dest_dir, src_file)).unwrap();
        fs::remove_dir(dest_dir).unwrap();
    }

    #[test]
    fn test_makefile() {
        let filename = "test_makefile";
        let ext = "txt";
        let mut current_dir = env::current_dir().unwrap();
        let result = makefile(&mut current_dir, filename, ext);
        assert!(result.is_ok());
        fs::remove_file(format!("{}.{}", filename, ext)).unwrap();
    }
}
