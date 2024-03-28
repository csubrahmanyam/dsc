use std::env;
use std::fs;
use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use whoami;

const VERSION: &str = "1.0.0";

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
        println!("  dsc author - about author");
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
            let mut icon_path = input();
            if icon_path == "\n".to_owned() {
                icon_path = "utilities-terminal".to_owned();
            }

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
            create_shortcut(desktop_entry);
            break;
        }
    } else if command == "author" {
        println!("dsc\n  author: C Subrahmanyam");
        println!("  credentials: Wrote 'hello world' program in 10 different languages!");
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

fn create_shortcut(desktop_entry: DesktopEntry) {
    // short-cut template
    let short_cut_template = format!(
        r#"[Desktop Entry]
Type=Application
Terminal={}
Exec={}
Name={}
Icon={}
Comment=app"#,
        desktop_entry.terminal,
        desktop_entry.app_path,
        desktop_entry.app_name,
        desktop_entry.icon_path
    );

    // short-cut file path
    let short_cut_path: &str = &format!(
        "/home/{}/.local/share/applications/{}.desktop",
        whoami::username(),
        desktop_entry.app_name
    );
    let short_cut_path = Path::new(&short_cut_path);

    //Open the file
    let mut short_cut_file: fs::File = match fs::File::open(&short_cut_path) {
        Ok(file) => {
            println!(
                "A short-cut with name '{}' already exists!!",
                desktop_entry.app_name
            );
            println!("Try any other name for your short-cut!");
            file
        }
        Err(error) => match fs::File::create(&short_cut_path) {
            Ok(file) => file,

            Err(why) => panic!("Error!\n\nUnable to create the short-cut!"),
        },
    };

    //write to file
    match short_cut_file.write_all(short_cut_template.as_bytes()) {
        Err(why) => panic!("Error:\n  Unable to create the short-cut!"),
        Ok(_) => println!("Successfully created short-cut!\n\nWait for a moment..."),
    };

    //add permissions to file as executable
    let mut permissions = short_cut_file.metadata().unwrap().permissions();

    permissions.set_mode(0o744);
    println!("Yep!,\n");
    println!("-> Go to Linux Mint Menu");
    println!("-> Search for name '{}'", desktop_entry.app_name);
    println!("-> Right click on the name and\n select add to desktop or panel!");
}
