use regex::Regex;

pub fn solve() {
    let input = include_str!("../input/day18");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> i64 {
    fn get_prec(op: BinOp) -> u32 {
        match op {
            BinOp::Add => 1,
            BinOp::Mul => 1,
        }
    }

    sum_lines(input, get_prec)
}

fn part2(input: &str) -> i64 {
    fn get_prec(op: BinOp) -> u32 {
        match op {
            BinOp::Add => 2,
            BinOp::Mul => 1,
        }
    }

    sum_lines(input, get_prec)
}

fn sum_lines(input: &str, get_prec: fn(BinOp) -> u32) -> i64 {
    let mut sum = 0;

    for line in input.lines() {
        let expr = parse(line, get_prec);
        sum += evaluate(&expr);
    }

    sum
}

fn evaluate(expr: &Expr) -> i64 {
    match expr {
        Expr::Literal { value } => *value,
        Expr::Binop { op, lhs, rhs } => {
            let lhs = evaluate(lhs);
            let rhs = evaluate(rhs);
            match op {
                BinOp::Add => lhs + rhs,
                BinOp::Mul => lhs * rhs,
            }
        }
    }
}

fn parse(s: &str, get_prec: fn(BinOp) -> u32) -> Expr {
    let tokens = lex(s);
    let mut pos = 0;
    parse_expr(&tokens, &mut pos, 0, get_prec)
}

fn lex(s: &str) -> Vec<Token> {
    let re = Regex::new(r"(\d+|[+*()])").unwrap();

    let mut tokens = Vec::new();

    for m in re.find_iter(s) {
        let tok = match m.as_str() {
            "+" => Token::BinOp(BinOp::Add),
            "*" => Token::BinOp(BinOp::Mul),
            "(" => Token::LParen,
            ")" => Token::RParen,
            s => Token::Literal(s.parse().unwrap()),
        };

        tokens.push(tok);
    }

    tokens
}

fn parse_expr(tokens: &[Token], pos: &mut usize, prec: u32, get_prec: fn(BinOp) -> u32) -> Expr {
    let mut e = parse_atom(tokens, pos, get_prec);

    loop {
        match tokens.get(*pos) {
            Some(Token::BinOp(op)) => {
                let op_prec = get_prec(*op);

                if op_prec <= prec {
                    break;
                }

                *pos += 1;

                let rhs = parse_expr(tokens, pos, op_prec, get_prec);

                e = Expr::Binop {
                    op: *op,
                    lhs: Box::new(e),
                    rhs: Box::new(rhs),
                };
            }

            Some(Token::RParen) => {
                break;
            }

            None => break,

            _ => {
                panic!("Invalid expression")
            }
        }
    }

    e
}

fn parse_atom(tokens: &[Token], pos: &mut usize, get_prec: fn(BinOp) -> u32) -> Expr {
    match tokens.get(*pos) {
        Some(Token::LParen) => {
            *pos += 1;
            let e = parse_expr(tokens, pos, 0, get_prec);
            *pos += 1;
            e
        }

        Some(Token::Literal(value)) => {
            *pos += 1;
            Expr::Literal { value: *value }
        }

        _ => {
            panic!("Invalid atom")
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Token {
    Literal(i64),
    BinOp(BinOp),
    LParen,
    RParen,
}

#[derive(Debug)]
enum Expr {
    Literal {
        value: i64,
    },

    Binop {
        op: BinOp,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum BinOp {
    Add,
    Mul,
}
