use std::env;
use std::fs;

fn vec_to_int(vec: &[char]) -> u64 {
    // let mut base = 1;
    let mut r = 0;


    for c in vec {
        if c.is_digit(10) {
            r *= 10;
            r += u64::from(c.to_digit(10).unwrap());
            
        }
    }

    return r;
}

fn main() {

    let mut args = env::args();
    let file_string = args.nth(1).unwrap();

    let file_contents = fs::read_to_string(file_string).unwrap();

    let lines: Vec<&str>= file_contents.lines().collect();

    let n_cols = lines[0].len();


    let mut stack: Vec<u64> = Vec::new();
    let mut total: u64 = 0;


    let mut reduce_op: fn(u64, u64) -> u64 = |x, y| 0;

    for col_idx in 0..n_cols {
        let mut col: Vec<char> = lines.iter().map(|line| line.chars().nth(col_idx).unwrap()).collect();

        let op_char = col.pop().unwrap();
        let n = vec_to_int(&col);

        if op_char == '*' {
            reduce_op = std::ops::Mul::mul;
        }
        else if op_char == '+' {
            reduce_op = std::ops::Add::add; 
        }

        if op_char == ' ' && n == 0 {
            total += stack.iter().copied().reduce(reduce_op).unwrap();
            stack.clear();
        }
        else {
            stack.push(n);
        }
    }

    total += stack.iter().copied().reduce(reduce_op).unwrap();

    println!("{}", total);
}