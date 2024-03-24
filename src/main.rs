use std::env;

const VERSION: &str = "0.1.0";
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
}
