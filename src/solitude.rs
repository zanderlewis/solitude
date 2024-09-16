use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

struct Interpreter {
    vars: HashMap<String, String>,
    funcs: HashMap<String, String>,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            vars: HashMap::new(),
            funcs: HashMap::new(),
        }
    }

    fn process_lines(&mut self, lines: Vec<String>) {
        let mut i = 0;
        while i < lines.len() {
            let line = lines[i].trim();
            if line.is_empty() || line.starts_with('.') {
                i += 1;
                continue; // Comment or blank line
            }
    
            if line.starts_with("var ") {
                self.process_var_declaration(&line[4..]);
            } else if line.starts_with('-') {
                self.delete_var(&line[1..]);
            } else if line.starts_with("if ") {
                if self.process_if_statement(&line[3..]) {
                    i += 1; // Execute the next line if the condition is met
                    if i < lines.len() {
                        let next_line = lines[i].trim();
                        self.process_line(next_line);
                    }
                }
            } else if line.starts_with("func ") {
                self.define_function(&line[5..]);
            } else if line.starts_with("call ") {
                self.execute_function(&line[5..]);
            } else if line.starts_with("input ") {
                self.input_variable(&line[6..]);
            } else {
                self.process_line(line);
            }
            i += 1;
        }
    }
    
    fn process_line(&mut self, line: &str) {
        let line = line.trim();
        if line.is_empty() || line.starts_with('.') {
            return; // Comment or blank line
        }
    
        if line.starts_with("var ") {
            self.process_var_declaration(&line[4..]);
        } else if line.starts_with('-') {
            self.delete_var(&line[1..]);
        } else if line.starts_with("if ") {
            if self.process_if_statement(&line[3..]) {
                // Execute the next line if the condition is met
                // This function does not handle the next line, so this part is omitted
            }
        } else if line.starts_with("func ") {
            self.define_function(&line[5..]);
        } else if line.starts_with("call ") {
            self.execute_function(&line[5..]);
        } else if line.starts_with("input ") {
            self.input_variable(&line[6..]);
        } else {
            let mut output = line.to_string();
            self.replace_variables(&mut output);
            self.process_escape_sequences(&mut output);
            println!("{}", output);
        }
    }

    fn process_var_declaration(&mut self, declaration: &str) {
        let parts: Vec<&str> = declaration.split('=').collect();
        if parts.len() == 2 {
            let name = parts[0].trim();
            let mut value = parts[1].trim().to_string();
            self.replace_variables(&mut value);
            if value.contains(['+', '-', '*', '/'].as_slice()) {
                value = self.evaluate_expression(&value).to_string();
            }
            self.set_var(name, &value);
        } else {
            eprintln!("Error: Invalid variable declaration format.");
        }
    }

    fn set_var(&mut self, name: &str, value: &str) {
        self.vars.insert(name.to_string(), value.to_string());
    }

    fn get_var(&self, name: &str) -> Option<&String> {
        self.vars.get(name)
    }

    fn delete_var(&mut self, name: &str) {
        if self.vars.remove(name).is_none() {
            eprintln!("Error: Undefined variable {}", name);
        }
    }

    fn define_function(&mut self, definition: &str) {
        let parts: Vec<&str> = definition.splitn(2, ' ').collect();
        if parts.len() == 2 {
            let name = parts[0];
            let body = parts[1];
            self.funcs.insert(name.to_string(), body.to_string());
        } else {
            eprintln!("Error: Invalid function definition format.");
        }
    }

    fn execute_function(&self, name: &str) {
        if let Some(body) = self.funcs.get(name) {
            let mut body = body.clone();
            self.replace_variables(&mut body);
            self.process_escape_sequences(&mut body);
            println!("{}", body);
        } else {
            eprintln!("Error: Undefined function {}", name);
        }
    }

    fn replace_variables(&self, s: &mut String) {
        let mut output = String::new();
        let mut var_name = String::new();
        let mut var_mode = false;

        for c in s.chars() {
            if c == '$' {
                var_mode = true;
            } else if var_mode && (c.is_alphanumeric() || c == '_') {
                var_name.push(c);
            } else {
                if var_mode {
                    if let Some(value) = self.get_var(&var_name) {
                        output.push_str(value);
                    } else {
                        eprintln!("Error: Undefined variable {}", var_name);
                    }
                    var_mode = false;
                    var_name.clear();
                }
                output.push(c);
            }
        }

        if var_mode {
            if let Some(value) = self.get_var(&var_name) {
                output.push_str(value);
            } else {
                eprintln!("Error: Undefined variable {}", var_name);
            }
        }

        *s = output;
    }

    fn process_escape_sequences(&self, s: &mut String) {
        let mut output = String::new();
        let mut chars = s.chars().peekable();

        while let Some(c) = chars.next() {
            if c == '\\' {
                if let Some(next) = chars.peek() {
                    if *next == '0' {
                        chars.next();
                        if let Some(next_next) = chars.peek() {
                            if *next_next == '3' {
                                chars.next();
                                if let Some(next_next_next) = chars.peek() {
                                    if *next_next_next == '3' {
                                        chars.next();
                                        output.push('\x1b');
                                    }
                                }
                            }
                        }
                    } else if *next == 'x' {
                        chars.next();
                        let mut hex = String::new();
                        if let Some(h1) = chars.next() {
                            hex.push(h1);
                        }
                        if let Some(h2) = chars.next() {
                            hex.push(h2);
                        }
                        if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                            output.push(byte as char);
                        }
                    }
                }
            } else {
                output.push(c);
            }
        }

        *s = output;
    }

    fn evaluate_expression(&self, expr: &str) -> f64 {
        let mut result = 0.0;
        let mut operator = '+';
        let mut chars = expr.chars().peekable();

        while let Some(c) = chars.peek() {
            if c.is_whitespace() {
                chars.next();
                continue;
            }

            if c.is_digit(10) || *c == '-' {
                let mut number_str = String::new();
                while let Some(d) = chars.peek() {
                    if d.is_digit(10) || *d == '.' || *d == '-' {
                        number_str.push(*d);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let value: f64 = f64::from_str(&number_str).unwrap_or(0.0);
                match operator {
                    '+' => result += value,
                    '-' => result -= value,
                    '*' => result *= value,
                    '/' => result /= value,
                    _ => (),
                }
            } else if ['+', '-', '*', '/'].contains(c) {
                operator = *c;
                chars.next();
            } else {
                chars.next();
            }
        }

        result
    }

    fn process_if_statement(&self, condition: &str) -> bool {
        let mut condition = condition.to_string();
        self.replace_variables(&mut condition);
        let result = self.evaluate_expression(&condition);
        result != 0.0
    }

    fn input_variable(&mut self, prompt: &str) {
        let parts: Vec<&str> = prompt.split("->").collect();
        let name = parts[0].trim();
        let message = if parts.len() == 2 { parts[1].trim() } else { "Enter value: " };

        println!("{}", message);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading input");
        input = input.trim().to_string();
        self.set_var(name, &input);
    }

    fn process_file(&mut self, filename: &str) {
        if let Ok(file) = File::open(filename) {
            let lines: Vec<String> = io::BufReader::new(file).lines().filter_map(Result::ok).collect();
            self.process_lines(lines);
        } else {
            eprintln!("Error: Could not open file {}", filename);
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let mut interpreter = Interpreter::new();
    interpreter.process_file(&args[1]);
}
