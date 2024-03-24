use std::env;
use std::fs::File;
use std::io;
use whoami;

const VERSION: &str = "0.1.0";

#[derive(Debug)]
struct DesktopEntry {
    app_name: String,
    app_path: String,
    icon_path: String,
    terminal: bool,
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let command = if args.len() > 1 { &args[1] } else { "app-info" };
    user_prompt(command)
}

/// Prompts the user based on the command entered!
fn user_prompt(command: &str) {
    // app-info: prints about dsc
    if command == "app-info" {
        println!(
            r#"
dsc: Desktop short-cut creator ( for Linux Mint )
version: {VERSION}
    
 dsc makes Desktop short-cut creation easy for users.
    
 Supported app formats:
     
    AppImages,
    Single binary apps
    
 For more info use:
    dsc help   "#
        );
    }
    // help: prints list of all commands
    else if command == "help" {
        println!("dsc commands:");
        println!("  dsc      - about dsc");
        println!("  dsc help - all commands list");
        println!("  dsc version - gives the version of dsc");
        println!("  dsc new  - create new app short-cut");
    }
    // version: prints dsc version
    else if command == "version" {
        println!("dsc: Desktop short-cut creator");
        println!("version: {VERSION}");
    }
    // new: creates new desktop short-cut
    else if command == "new" {
        loop {
            println!("Enter app name: ");
            let app_name = input();
            if app_name == "\n" {
                println!("Enter all fields!");
                continue;
            }

            println!("Enter full path to app:");
            let app_path = input();
            if app_path == "\n" {
                println!("Enter all fields!");
                continue;
            }

            println!("Enter full path to app icon (optional):");
            let icon_path = input();

            println!("Always open app in a terminal(y/n): ");
            let terminal: bool = {
                if input() == "y".to_owned() {
                    true
                } else {
                    false
                }
            };

            let desktop_entry = DesktopEntry {
                app_name,
                app_path,
                icon_path,
                terminal,
            };
            // create_shortcut(desktop_entry);
            break;
        }
    }
}

//takes input from user and handles errors!
fn input() -> String {
    let mut user_input = String::new();
    match io::stdin().read_line(&mut user_input) {
        Ok(_n) => {}
        Err(_error) => {
            {
                println!("error!!! \nTry again!");
            };
        }
    }
    user_input
}

// fn create_shortcut(desktop_entry: DesktopEntry) {
//     let short_cut = File::open("main.rs").unwrap();
//     // println!("{}", whoami::username().len());
// }
