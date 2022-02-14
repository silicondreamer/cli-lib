use std::collections::HashMap;
use std::io;
use std::io::Write; // <--- bring flush() into scope

#[derive(Debug)]
struct Command<'a> {
    name: &'a str,
    id: u32,
    callback: fn() -> Result<String, String>,
}

pub type CommandResult = Result<String, String>;

fn test() -> CommandResult {
    Ok("test!".to_string())
}

fn exit() -> CommandResult {
    std::process::exit(0);
}

//declare a trait
trait Printable {
    fn print(&self);
}

trait Executable {
    fn execute(&self) -> CommandResult;
}

//implement the trait
impl<'a> Printable for Command<'a> {
    fn print(&self) {
        println!(
            "Printing command usage with id:{} and name {}",
            self.id, self.name
        )
    }
}

impl<'a> Executable for Command<'a> {
    fn execute(&self) -> CommandResult {
        (self.callback)()
    }
}

pub fn create_cli() {
    init_cli();
}

pub fn destroy_cli() {}

fn init_cli() {
    let mut commands: HashMap<&str, Command> = HashMap::new();
    register_commands(&mut commands);
    command_loop(&mut commands);
}

fn register_commands(commands: &mut HashMap<&str, Command>) {
    let c1 = Command {
        name: "test",
        id: 0,
        callback: test,
    };
    let c2 = Command {
        name: "exit",
        id: 1,
        callback: exit,
    };
    commands.insert(c1.name, c1);
    commands.insert(c2.name, c2);
}

fn command_loop(commands: &HashMap<&str, Command>) {
    loop {
        println!("");
        display_prompt();

        let data = get_user_input();
        let mut token_vec = Vec::new();
        tokenise_line(data, &mut token_vec);
        if token_vec.len() > 0 {
            let res = process_command(&commands, &token_vec);
            match res {
                Ok(o) => println!("OK: {:?}", o),
                Err(e) => println!("ERROR: {:?}", e),
            }
        }
    }
}

fn display_prompt() {
    print!("$>");
    io::stdout().flush().unwrap();
}

fn get_user_input() -> String {
    let mut line = String::new();
    let _num_bytes_read = std::io::stdin().read_line(&mut line).unwrap();
    return line;
}

fn tokenise_line(line: String, token_vec: &mut Vec<String>) {
    if line.chars().count() > 0 {
        for token in line.trim().split_whitespace() {
            token_vec.push(token.to_string());
        }
    }
}

fn process_command(commands: &HashMap<&str, Command>, token_vec: &Vec<String>) -> CommandResult {
    match commands.get(&token_vec[0] as &str) {
        Some(command) => Ok((command.callback)().unwrap()),
        None => Err("command not found".to_string()),
    }
}
