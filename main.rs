use std::env;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::io::{self, Write};

fn main() {
    loop {
        let current_env_path = env::current_dir().unwrap();
        print!("\x1B[40;36;1m{}▷ \x1B[0m ", current_env_path.display());
        if let Err(e) = io::stdout().flush() {
            eprintln!("{}", e);
        };

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let main_command = parts.next().unwrap();
            let args = parts;

            match main_command {
                "cd" => {
                    let navigate_dir = args.peekable().peek().map_or("/", |x| *x);
                    let root = Path::new(navigate_dir);
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }
                    previous_command = None;
                },
                "exit" => return,
                main_command => {
                    let stdin = previous_command.map_or(
                        Stdio::inherit(),
                        |output: Child| Stdio::from(output.stdout.unwrap()));

                    let stdout = if commands.peek().is_some() {
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let child_process_output = Command::new(main_command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();
        
                    match child_process_output {
                        Ok(child_process_output) => { previous_command = Some(child_process_output) },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            if let Err(e) = final_command.wait() {
                eprintln!("{}", e)
            };
        }
    }
}

