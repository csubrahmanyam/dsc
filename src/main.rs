use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = if args.len() > 1 { &args[1] } else { "app-info" };
    user_prompt(command)
}

fn user_prompt(command: &str) {
    if command == "app-info" {
        println!(
            r#"
dsc: Desktop short-cut creator ( for Linux Mint )
    
  dsc makes Desktop short-cut creation easy for users.
    
    Supported app formats:
     
     AppImages,
     Single binary apps
    
  For more info use:
    dsc help   "#
        );
    }
}
