use std::io;

// Convert fh temperature to celsius
fn fh_to_celsius (fh: f64) -> f64 {
    return (fh - 32.0) * 5.0/9.0;
}

fn fibonacci (target: i32, num1: i32, num2: i32, count: i32 ) {
    println!("{}", num1);
    let aux = num2;
    let num2 = num1 + num2;
    let num1 = aux;
    if count >= target { return; }
    let count = count + 1;
    fibonacci(target, num1, num2, count);
}

fn main() {
    // Converting
    println!("Please input the temperature in ºF");
    let mut fh = String::new();
    io::stdin().read_line(&mut fh)
        .expect("Failed to read line");
    let fh: f64 = match fh.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };
    println!("{}ºF is {:.2}ºC", fh, fh_to_celsius(fh));

    // Fibonacci
    println!("Please input the fibonacci number");
    let mut fib = String::new();
    io::stdin().read_line(&mut fib)
        .expect("Failed to read line");
    let fib: i32 = match fib.trim().parse() {
        Ok(num) => num,
        Err(_) => return,
    };
    fibonacci(fib, 0, 1, 1);
}
