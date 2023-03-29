use std::io::{self, Write};
use lisp_interpreter::interpreter::{Environment, tokenize, parse, eval};

fn interpret(input: &str, env: &mut Environment) -> Result<String, String> {
    let tokens = tokenize(input);
    let (parsed_expr, _) = parse(&tokens)?;

    let result = eval(&parsed_expr, env)?;
    Ok(format!("{}", result))
}

fn main() {
    let mut env = Environment::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match interpret(&input, &mut env) {
            Ok(result) => println!("{}", result),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
