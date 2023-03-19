// use env module
use std::env::{args, Args};

fn main() {
    // marcro println
    let mut args: Args = args(); // arguments are like generator in python
    println!("{:?}", args);
    // println!("{:?}", args.inner);
    // get first second and third argument
    let first = args.nth(1).unwrap(); // first argument
    let first_number = first.parse::<f32>().unwrap(); // parse to i32
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    // let operator = operator.parse::<char>().unwrap();
    let second = args.nth(0).unwrap(); // .unwrap();
    let second_number = second.parse::<f32>().unwrap(); // parse to i32 turbofish
    let value: f32 = operate(operator, first_number, second_number);
    // println!("{first} {operator} {second} = {value}")
    println!("{}", output(first_number, operator, second_number, value))
    // println!("{} {} {}", first, operator, second);
    // println!("{:?}", args);
    // for arg in args {
    //     println!("{}", arg);
    // }
}

fn operate(operator: char, first: f32, second: f32) -> f32 {
    match operator {
        '+' => first + second,
        '-' => first - second,
        '*' | 'x' | 'X' => first * second,
        '/' => first / second,
        _ => panic!("Unknown operator"),
    }
    // or
    // if operator == '+' {
    //     println!("addition");
    //     first + second
    // } else if operator == '*' {
    //     println!("multiplication");
    //     first * second
    // } else if operator == '/' {
    //     println!("devide");
    //     first / second
    // } else if operator == '-' {
    //     println!("minus");
    //     first - second
    // } else {
    //     0
    // }
}

fn output(first_number: f32, operator: char, second: f32, result: f32) -> String {
    format!("{first_number} {operator} {second} = {result}")
}
// 1. what does unwrap do?
// anwer: unwrap() returns the value of an Option<T> or panics if the value is None.
// 2. what is Option<T>?
// answer: Option<T> is an enum that can be either Some or None.
// 3> what Some and None?
// answer: Some is a variant of Option<T> that contains a value of type T.
// None is a variant of Option<T> that does not contain a value.
// 4. what is enum?
// answer: enum is a type that can be one of a few different variants.
// 5. what is T in Option<T>?
// answer: T is a generic type parameter.

// nth method
// fn nth(&mut self, n: usize) -> Option<String> {
//     let mut i = 0;
//     while let Some(arg) = self.next() {
//         if i == n {
//             return Some(arg);
//         }
//         i += 1;
//     }
//     None
// }
