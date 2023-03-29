pub mod interpreter {
    use std::collections::HashMap;
    use std::fmt;

    #[derive(Debug, PartialEq, Clone)]
    pub enum Expr {
        Symbol(String),
        Number(f64),
        List(Vec<Expr>),
    }

    type Function = fn(&[Expr], &mut Environment) -> Result<Expr, String>;

    #[derive(Default)]
    pub struct Environment {
        symbols: HashMap<String, Expr>,
        functions: HashMap<String, Function>,
    }

    impl fmt::Debug for Environment {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Environment")
                .field("symbols", &self.symbols)
                .field("functions", &"Function HashMap") // Use a string description for the functions field
                .finish()
        }
    }

    impl fmt::Display for Expr {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Expr::Symbol(s) => write!(f, "{}", s),
                Expr::Number(n) => write!(f, "{}", n),
                Expr::List(l) => {
                    let inner: Vec<String> = l.iter().map(|e| e.to_string()).collect();
                    write!(f, "({})", inner.join(" "))
                }
            }
        }
    }

    pub fn tokenize(input: &str) -> Vec<String> {
        // Replace parentheses with spaces and add split tokens
        input.replace("(", " ( ")
            .replace(")", " ) ")
            .split_whitespace()
            .map(|token| token.to_string())
            .collect()
    }

    pub fn parse(tokens: &[String]) -> Result<(Expr, &[String]), String> {
        if tokens.is_empty() {
            return Err("Unexpected EOF".to_string());
        }

        let (token, rest) = tokens.split_first().unwrap();

        let expr = match &token[..] {
            "(" => {
                let mut list = Vec::new();
                let mut remaining_tokens = rest;

                while let Some(token) = remaining_tokens.first() {
                    if token == ")" {
                        break;
                    }

                    let (parsed_expr, new_remaining_tokens) = parse(remaining_tokens)?;
                    list.push(parsed_expr);
                    remaining_tokens = new_remaining_tokens;
                }

                if remaining_tokens.is_empty() {
                    return Err("Unexpected EOF".to_string());
                }

                let (_, new_remaining_tokens) = remaining_tokens.split_first().unwrap();
                (Expr::List(list), new_remaining_tokens)
            }
            ")" => {
                return Err("Unexpected )".to_string());
            }
            _ => {
                let atom = if let Ok(number) = token.parse::<f64>() {
                    Expr::Number(number)
                } else {
                    Expr::Symbol(token.clone())
                };

                (atom, rest)
            }
        };

        Ok(expr)
    }


    fn add(args: &[Expr], _env: &mut Environment) -> Result<Expr, String> {
        let mut sum = 0.0;

        for arg in args {
            match arg {
                Expr::Number(n) => sum += n,
                _ => return Err("Invalid argument type for addition".to_string()),
            }
        }

        Ok(Expr::Number(sum))
    }

    fn subtract(args: &[Expr], _env: &mut Environment) -> Result<Expr, String> {
        if args.is_empty() {
            return Err("At least one argument is required for subtraction".to_string());
        }

        let mut args_iter = args.iter();
        let first_arg = args_iter.next().unwrap();

        let mut difference = match first_arg {
            Expr::Number(n) => *n,
            _ => return Err("Invalid argument type for subtraction".to_string()),
        };

        for arg in args_iter {
            match arg {
                Expr::Number(n) => difference -= n,
                _ => return Err("Invalid argument type for subtraction".to_string()),
            }
        }

        Ok(Expr::Number(difference))
    }

    fn car(args: &[Expr], _env: &mut Environment) -> Result<Expr, String> {
        if args.len() != 1 {
            return Err("Expected exactly one argument for car".to_string());
        }

        match &args[0] {
            Expr::List(list) if !list.is_empty() => Ok(list[0].clone()),
            _ => Err("Invalid argument type for car".to_string()),
        }
    }

    fn cdr(args: &[Expr], _env: &mut Environment) -> Result<Expr, String> {
        if args.len() != 1 {
            return Err("Expected exactly one argument for cdr".to_string());
        }

        match &args[0] {
            Expr::List(list) => Ok(Expr::List(list[1..].to_vec())),
            _ => Err("Invalid argument type for cdr".to_string()),
        }
    }

    fn define(args: &[Expr], env: &mut Environment) -> Result<Expr, String> {
        if args.len() != 2 {
            return Err("Exactly 2 arguments are required for 'define'".to_string());
        }
    
        let symbol = match &args[0] {
            Expr::Symbol(s) => s,
            _ => return Err("First argument of 'define' must be a symbol".to_string()),
        };
    
        let value = eval(&args[1], env)?;
    
        env.symbols.insert(symbol.clone(), value.clone());
    
        Ok(value)
    }

    fn print(args: &[Expr], env: &mut Environment) -> Result<Expr, String> {
        if args.len() != 1 {
            return Err("Exactly 1 argument is required for 'print'".to_string());
        }
    
        let value = eval(&args[0], env)?;
        println!("{}", value);
    
        Ok(value)
    }

    impl Environment {
        pub fn new() -> Self {
            let mut env = Environment::default();
            env.functions.insert("+".to_string(), add);
            env.functions.insert("-".to_string(), subtract);
            env.functions.insert("car".to_string(), car);
            env.functions.insert("cdr".to_string(), cdr);
            env.functions.insert("define".to_string(), define);
            env.functions.insert("print".to_string(), print);
            env
        }
    }


    pub fn eval(expr: &Expr, env: &mut Environment) -> Result<Expr, String> {
        match expr {
            Expr::Symbol(symbol) => {
                env.symbols
                    .get(symbol)
                    .cloned()
                    .ok_or_else(|| format!("Undefined symbol: {}", symbol))
            }
            Expr::Number(_) => Ok(expr.clone()),
            Expr::List(list) => {
                if list.is_empty() {
                    return Err("Cannot evaluate an empty list".to_string());
                }
    
                let first_expr = &list[0];
                match first_expr {
                    Expr::Symbol(symbol) => match &symbol[..] {
                        "define" => {
                            if list.len() != 3 {
                                return Err("Invalid number of arguments for 'define'".to_string());
                            }
                            let var_name = match &list[1] {
                                Expr::Symbol(name) => name,
                                _ => return Err("Expected a symbol for the variable name".to_string()),
                            };
                            let value = eval(&list[2], env)?;
                            env.symbols.insert(var_name.clone(), value);
                            Ok(Expr::Symbol(var_name.clone()))
                        }
                        "print" => {
                            if list.len() != 2 {
                                return Err("Invalid number of arguments for 'print'".to_string());
                            }
                            let value = eval(&list[1], env)?;
                            println!("{}", value);
                            Ok(value)
                        }
                        _ => {
                            if env.functions.contains_key(symbol) {
                                let func = env.functions[symbol];
                                let args: Result<Vec<Expr>, String> =
                                    list[1..].iter().map(|expr| eval(expr, env)).collect();
                                match args {
                                    Ok(evaluated_args) => func(&evaluated_args, env),
                                    Err(e) => Err(e),
                                }
                            } else {
                                Err(format!("Undefined function: {}", symbol))
                            }
                        }
                    },
                    _ => {
                        let evaluated_list: Result<Vec<Expr>, String> =
                            list.iter().map(|expr| eval(expr, env)).collect();
                        evaluated_list.map(|elems| Expr::List(elems))
                    }
                }
            }
        }
    }
    
    
}
