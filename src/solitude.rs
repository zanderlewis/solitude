use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::thread;

struct Interpreter {
    vars: Arc<Mutex<HashMap<String, String>>>,
    funcs: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            vars: Arc::new(Mutex::new(HashMap::new())),
            funcs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn process_lines(&self, lines: Vec<String>) {
        let mut i = 0;
        let mut handles = vec![];

        while i < lines.len() {
            let line = lines[i].trim();
            // Multi-line comments are three dots on a line by themselves
            if line == "..." {
                i += 1;
                while i < lines.len() {
                    if lines[i].trim() == "..." {
                        i += 1;
                        break;
                    }
                    i += 1;
                }
                continue;
            }
            if line.is_empty() || line.starts_with('.') {
                i += 1;
                continue; // Comment or blank line
            }

            if line.starts_with("!!") {
                let interpreter = self.clone();
                let mut thread_lines = vec![];
                i += 1; // Skip the `!!` line

                while i < lines.len() {
                    let thread_line = lines[i].trim();
                    if thread_line.starts_with("??") {
                        i += 1; // Skip the `??` line
                        break;
                    }
                    if thread_line.starts_with('{') {
                        i += 1; // Skip the `{` line
                        while i < lines.len() {
                            let block_line = lines[i].trim();
                            if block_line.starts_with('}') {
                                i += 1; // Skip the `}` line
                                break;
                            }
                            thread_lines.push(lines[i].clone());
                            i += 1;
                        }
                    } else {
                        thread_lines.push(lines[i].clone());
                        i += 1;
                    }
                }

                let handle = thread::spawn(move || {
                    interpreter.process_lines(thread_lines);
                });
                handles.push(handle);
            } else {
                self.process_line(&lines, &mut i);
                i += 1;
            }
        }

        // Join all threads to ensure they complete execution
        for handle in handles {
            if let Err(e) = handle.join() {
                eprintln!("Error joining thread: {:?}", e);
            }
        }
    }

    fn process_line(&self, lines: &Vec<String>, i: &mut usize) {
        let line = lines[*i].trim();
        if line.is_empty() || line.starts_with('.') {
            return; // Comment or blank line
        }

        if line.starts_with("var ") {
            self.process_var_declaration(&line[4..]);
        } else if line.starts_with('-') {
            self.delete_var(&line[1..]);
        } else if line.starts_with("if ") {
            *i += 1;
            if self.process_if_statement(&line[3..]) {
                let mut if_lines = vec![];
                while *i < lines.len() {
                    let if_line = lines[*i].trim();
                    if if_line.starts_with("fi") {
                        *i += 1;
                        break;
                    }
                    if_lines.push(lines[*i].clone());
                    *i += 1;
                }
                self.process_lines(if_lines);
            } else {
                while *i < lines.len() {
                    let if_line = lines[*i].trim();
                    if if_line.starts_with("fi") {
                        *i += 1;
                        break;
                    }
                    *i += 1;
                }
            }
        } else if line.starts_with("func ") {
            self.define_function(&lines, i);
        } else if line.starts_with("call ") {
            self.execute_function(&line[5..]);
        } else if line.starts_with("input ") {
            self.input_variable(&line[6..]);
        } else {
            let mut output = line.to_string();
            self.replace_variables(&mut output);
            self.process_escape_sequences(&mut output);
            print!("{}", output);
        }
    }

    fn process_var_declaration(&self, declaration: &str) {
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

    fn set_var(&self, name: &str, value: &str) {
        let mut vars = self.vars.lock().unwrap();
        vars.insert(name.to_string(), value.to_string());
    }

    fn get_var(&self, name: &str) -> Option<String> {
        let vars = self.vars.lock().unwrap();
        vars.get(name).cloned()
    }

    fn delete_var(&self, name: &str) {
        let mut vars = self.vars.lock().unwrap();
        if vars.remove(name).is_none() {
            eprintln!("Error: Undefined variable {}", name);
        }
    }

    fn define_function(&self, lines: &Vec<String>, i: &mut usize) {
        let line = lines[*i].trim();
        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() == 2 {
            let name = parts[1];
            let mut body = vec![];
            *i += 1;
            while *i < lines.len() {
                let func_line = lines[*i].trim();
                if func_line.starts_with("cnuf") {
                    *i += 1;
                    break;
                }
                body.push(lines[*i].clone());
                *i += 1;
            }
            let mut funcs = self.funcs.lock().unwrap();
            funcs.insert(name.to_string(), body);
        } else {
            eprintln!("Error: Invalid function definition format.");
        }
    }

    fn execute_function(&self, name: &str) {
        let funcs = self.funcs.lock().unwrap();
        if let Some(body) = funcs.get(name) {
            let body = body.clone(); // Clone the body before dropping the lock
            drop(funcs); // Release the lock before processing the body
            for line in body {
                let mut line = line.clone();
                self.replace_variables(&mut line);
                self.process_escape_sequences(&mut line);
                print!("{}", line);
            }
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
                        output.push_str(&value);
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
                output.push_str(&value);
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
                    } else if *next == 'n' {
                        chars.next();
                        output.push('\n');
                    } else if *next == 'r' {
                        chars.next();
                        output.push('\r');
                    } else if *next == 't' {
                        chars.next();
                        output.push('\t');
                    } else {
                        output.push(c);
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
            } else if *c == '>' || *c == '<' || *c == '=' {
                let op = *c;
                chars.next();
                if let Some('=') = chars.peek() {
                    chars.next();
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
                    result = match op {
                        '>' => if result >= value { 1.0 } else { 0.0 },
                        '<' => if result <= value { 1.0 } else { 0.0 },
                        '=' => if result == value { 1.0 } else { 0.0 },
                        _ => result,
                    };
                }
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

    fn input_variable(&self, prompt: &str) {
        let parts: Vec<&str> = prompt.split("->").collect();
        let name = parts[0].trim();
        let mut message = if parts.len() == 2 { parts[1].trim().to_string() } else { "Enter value: ".to_string() };
    
        // Process the message to replace variables and escape sequences
        self.replace_variables(&mut message);
        self.process_escape_sequences(&mut message);
    
        print!("{}", message);
        io::stdout().flush().expect("Error flushing stdout");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading input");
        input = input.trim().to_string();
        self.set_var(name, &input);
    }

    fn process_file(&self, filename: &str) {
        if let Ok(file) = File::open(filename) {
            let lines: Vec<String> = io::BufReader::new(file).lines().filter_map(Result::ok).collect();
            self.process_lines(lines);
        } else {
            eprintln!("Error: Could not open file {}", filename);
        }
    }
}

impl Clone for Interpreter {
    fn clone(&self) -> Self {
        Interpreter {
            vars: Arc::clone(&self.vars),
            funcs: Arc::clone(&self.funcs),
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <file>", args[0]);
        return;
    }

    let interpreter = Interpreter::new();
    interpreter.process_file(&args[1]);
}
