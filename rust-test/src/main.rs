use std::io;

fn main() {
    loop {
        println!("Enter a number to choose the program.
                  Options:
                  1: Convert temperatures
                  2: Nth Fibonacci number
                  3: Is the number prime
                  4: Quit program manager");
        let mut selection = String::new();
        io::stdin().read_line(&mut selection).expect("Failed to read");
        match selection.trim() {
            "1" => convert_temps(),
            "2" => nth_fibonacci(),
            "3" => prime_checker(),
            "4" => break,
            _ => {
                println!("Invalid selection entered. Try again.");
                continue
            }
        }
    }
}

fn convert_temps() {
    println!("Enter F to enter Fahrenheit, or C to enter Celsius");
    let mut degree = String::new();
    io::stdin().read_line(&mut degree).expect("Failed to read"); 
    let degree = degree.trim();

    let mut temperature = String::new();
    print!("Enter the temperature:");
    io::stdin().read_line(&mut temperature).expect("Failed to read");
    let temperature : f64 = temperature.trim().parse().unwrap();

    if degree.eq("F") {
        let celsius = (temperature - 32.0) * (5.0/9.0);
        println!("The value in celsius is: {}", celsius);
    } else {
        let fahr = temperature * (9.0/5.0) + 32.0;
        println!("The value in fahrenheit is: {}", fahr);
    }
}

fn nth_fibonacci() {
    println!("Enter the index of the number you wish to find in the Fibonacci sequence.");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read"); 
    let n : u32 = n.trim().parse().unwrap();

    fn fibo_helper(num : u32) -> i64 {
        if num < 2 {
            return num as i64;
        }
        fibo_helper(num-1) + fibo_helper(num-2)
    }

    println!("The {}th fibonacci number is {}", n, fibo_helper(n));
}

fn prime_checker() {
    println!("Enter a number to check for primeness.");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read"); 
    let n : u64 = n.trim().parse().unwrap();

    if n == 2 || n == 3 || n == 7{
        println!("{} is a prime!", n);
        return
    }
    if n <= 1 || n % 2 == 0 || n % 3 == 0 || n % 5 == 0 || n % 7 == 0 {
        println!("{} is not prime.", n);
        return
    }
    
    for i in 8 ..(n/2 + 1) {
        if n % i == 0 {
            println!("{} is not prime.", n);
            return
        }
    }

    println!("{} is a prime!", n);
}