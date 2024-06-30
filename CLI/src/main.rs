use std::io;

fn main() {
    println!("This is Uri's command line project in Rust - enjoy");
    while true {
        let input: String = get_input();
        println!("{}", input);
        if input == "exit" || input == "EXIT" {
            break;
        } else if input == "help" || input == "HELP" {
            print_help();
        }
    }
}

fn get_input() -> String {
    let mut input: String = String::new();
    match io::stdin().read_line(&mut input) {
        Ok() => {}
        Err() => {}
    }

    input.trim().to_string()
}

fn print_help() {
    println!(
        "---HELP--- \n
    1)grep - matches text in files\n
    2)echo - reapets input\n
    3)find - locates files or directories\n
    4)ls/dir - lists directories\n
    5)pwd - shows current full directory\n
    6)exit/EXIT - exits program\n
    7)help/HELP - prints explanation of all the commands\n "
    )
}
