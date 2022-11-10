#[derive(PartialEq)]
enum Token {
    Operand(i32),
    Operator(Operator),
    LeftParenthesis,
    RightParenthesis
}

#[derive(PartialEq)]
enum Operator {
    Multiply,
    Divide,
    Add,
    Subtract, //binary -
    Negate    //unary -
}

fn main() {
    let args = std::env::args()
        .skip(1) //skip the name of the executable
        .collect::<Vec<String>>();
    
    if args.is_empty() {
        let input_text = read_expression();
        if input_text.is_empty() {
            return;
        }
        process_expression(input_text)
    } else {
        process_expression(args)
    }
}

//returns 1 line of stdin split by whitespace
fn read_expression() -> Vec<String> {
    print!("Enter your expression: ");
    std::io::Write::flush(&mut std::io::stdout())
        .expect("Error: could not flush text to stdout");
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Error: could not read stdin");
    line.split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

//prints out the result of evaluating the input text as an expression
fn process_expression(input_text: Vec<String>) {    
    let valid = validate_chars(&input_text);
    if !valid {
        println!("Not valid!");
        return;
    }

    let result = tokenize(&split_strings(&input_text))
        .and_then(infix_to_postfix)
        .and_then(evaluate_postfix);

    match result {
        Ok(value) => println!("{:b}", value),
        Err(_) => println!("Not valid!")
    }
}
    
fn is_binary(num: &str) -> bool {
    num.chars().all(|ch| ch == '0' || ch == '1')
}

fn tokenize(strings: &[String]) -> Result<Vec<Token>, String> {
    let mut unary = true;
    strings.iter().map(|string| {
        match string.as_str() {
            "(" => { unary = true; Ok(Token::LeftParenthesis)},
            ")" => { unary = false; Ok(Token::RightParenthesis)},
            "*" => { unary = true; Ok(Token::Operator(Operator::Multiply))},
            "/" => { unary = true; Ok(Token::Operator(Operator::Divide))},
            "+" => { unary = true; Ok(Token::Operator(Operator::Add))},
            "-" if unary => {unary = false; Ok(Token::Operator(Operator::Negate))},
            "-" => { unary = true; Ok(Token::Operator(Operator::Subtract))},
            num if is_binary(num) => {
                unary = false;
                if let Ok(num) = i32::from_str_radix(num, 2) {
                    Ok(Token::Operand(num))
                } else {
                    Err(String::from("Unable to parse number"))
                }
            },
            _ => Err(String::from("Invalid token"))
        }
    }).collect()
}

fn split_strings(tokens: &[String]) -> Vec<String> {
    tokens.iter()
        .flat_map(|s| split_string(s))
        .collect()
}

//splits a string into a Vec of strings
// ex. "(10010+" -> ["(","10010","+"]
fn split_string(token: &str) -> Vec<String> {
    let mut ret = Vec::new();
    let mut tmp = String::new();
    for ch in token.chars() {
        if ch == '0' || ch == '1' {
            tmp.push(ch);
        } else {
            if !tmp.is_empty() {
                ret.push(tmp);
            }
            ret.push(String::from(ch));
            tmp = String::new();
        }
    }
    if !tmp.is_empty() {
        ret.push(tmp); 
    }
    ret
}

//returns false if there are any invalid characters
fn validate_chars(tokens: &[String]) -> bool {
    tokens.iter()
        .flat_map(|s| s.chars())
        .all(|ch| match ch {
            '('|')'|'1'|'0'|'+'|'-'|'/'|'*' => true,
            _ => false
        })
}

fn precedence(operator: &Operator) -> u8 {
    match operator { 
        Operator::Negate => 2,
        Operator::Multiply => 1,
        Operator::Divide => 1,
        Operator::Add => 0,
        Operator::Subtract => 0,
    }
}

//Uses shunting yard algorithm 
fn infix_to_postfix(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut output: Vec<Token> = Vec::new();
    let mut operators: Vec<Token> = Vec::new();
    for token in tokens {
        match token {
            Token::Operator(ref operator) => {
                while let Some(top) = operators.last() {
                    match top {
                        Token::LeftParenthesis => break,
                        Token::Operator(top_op) => {
                            if precedence(top_op) >= precedence(&operator) {
                                output.push(operators.pop().unwrap());
                            } else {
                                break;
                            }
                        },
                        _ => return Err("Impossible token in operator stack".to_string())
                    };
                }
                operators.push(token);
            },
            Token::LeftParenthesis => operators.push(token),
            Token::RightParenthesis => {
                match operators.iter().rposition(|op| op == &Token::LeftParenthesis) {
                    Some(pos) => output.extend(operators.drain(pos+1..).rev()),
                    None => return Err("Parenthesis mismatch".to_string())
                };
                operators.pop();
            },
            Token::Operand(_) => output.push(token),
        }
    }

    if operators.contains(&Token::LeftParenthesis) {
        return Err("Parenthesis mismatch".to_string());
    }
    output.extend(operators.into_iter().rev());
    Ok(output)
}

fn evaluate_postfix(expression: Vec<Token>) -> Result<i32, String> {
    let mut stack: Vec<i32> = Vec::new();
    for token in expression {
        match token {
            Token::Operator(operator) => {
                match operator {
                    Operator::Negate => {
                        match stack.pop() {
                            Some(operand) => stack.push(-operand),
                            None => return Err("Invalid".to_string())
                        }
                    },
                    _ => {
                        match stack.pop().zip(stack.pop()) {
                            Some((operand_1, operand_2)) => {
                                let result = match operator {
                                    Operator::Add => operand_2 + operand_1,
                                    Operator::Subtract => operand_2 - operand_1,
                                    Operator::Multiply => operand_2 * operand_1,
                                    Operator::Divide => operand_2 / operand_1,
                                    Operator::Negate => unreachable!()
                                };
                                stack.push(result);
                            },
                            None => return Err("Invalid".to_string())
                         }
                    }
                }
            },
            Token::Operand(num) => stack.push(num),
            _ => return Err("Invalid".to_string())
        }
    }

    if stack.len() != 1 {
        Err("Invalid".to_string())
    } else {
        Ok(stack.pop().unwrap())
    }
}