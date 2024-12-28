use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    Operator(char),
}
pub fn eval(expression: &str) -> f64 {
    // 步骤 1: 去除等号前缀
    let expr = if expression.starts_with('=') {
        &expression[1..]
    } else {
        expression
    };
    // 步骤 2: 标记化
    let tokens = match tokenize(expr) {
        Ok(toks) => toks,
        Err(e) => {
            eprintln!("标记化错误: {}", e);
            return 0.0;
        }
    };
    // 步骤 3: 计算表达式
    let res = evaluate_infix(&tokens).unwrap_or_else(|e| {
        eprintln!("计算时出现错误: {}", e);
        0.0
    });
    res
}

fn tokenize(expr: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut current_num = String::new();
    let mut chars = expr.chars().peekable();

    while let Some(&ch) = chars.peek() {
        if ch.is_whitespace() {
            chars.next();
        } else if ch.is_digit(10) || ch == '.' {
            current_num.push(ch);
            chars.next();
        } else if is_operator(ch) {
            //将临时存放的数字加入tokens
            if !current_num.is_empty() {
                let num: f64 = current_num
                    .parse::<f64>()
                    .map_err(|_| format!("无效的数字: {}", current_num))?;
                tokens.push(Token::Number(num));
                current_num.clear();
            }
            tokens.push(Token::Operator(ch));
            chars.next();
        } else {
            return Err(format!("无效的字符: {}", ch));
        }
    }
    // 如果字符串结束时还有未处理的数字
    if !current_num.is_empty() {
        let num: f64 = current_num
            .parse()
            .map_err(|_| format!("无效的数字: {}", current_num))?;
        tokens.push(Token::Number(num));
    }
    Ok(tokens)
}

// 判断字符是否为有效运算符
fn is_operator(ch: char) -> bool {
    matches!(ch, '+' | '-' | '*' | '/')
}
fn evaluate_infix(tokens: &Vec<Token>) -> Result<f64, String> {
    let mut numbers: VecDeque<f64> = VecDeque::new();
    let mut operators: VecDeque<char> = VecDeque::new();
    for token in tokens {
        match *token {
            Token::Number(n) => {
                numbers.push_back(n);
            }
            Token::Operator(op) => {
                while let Some(&top_op) = operators.back() {
                    if precedence(top_op) >= precedence(op) {
                        let o2 = numbers.pop_back().ok_or("缺少操作数")?;
                        let o1 = numbers.pop_back().ok_or("缺少操作数")?;
                        let result = apply_operator(top_op, o1, o2);
                        numbers.push_back(result);
                        operators.pop_back();
                    } else {
                        break; // 如果当前运算符优先级较高，则跳出循环，压入栈
                    }
                }
                operators.push_back(op);
            }
        }
    }
    // 计算剩余的操作符
    while let Some(op) = operators.pop_back() {
        let o2 = numbers.pop_back().ok_or("缺少操作数")?;
        let o1 = numbers.pop_back().ok_or("缺少操作数")?;
        let result = apply_operator(op, o1, o2);
        numbers.push_back(result);
    }
    Ok(numbers.pop_back().unwrap_or(0.0))
}
/// 运算符优先级
fn precedence(op: char) -> usize {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0,
    }
}
fn apply_operator(op: char, a: f64, b: f64) -> f64 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        _ => panic!("无效运算符"),
    }
}
