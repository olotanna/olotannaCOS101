use std::io;

fn main() {
    // Read the value of n from the user
    println!("Enter the value of n:");
    
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read input");
    
    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a positive integer.");
            return;
        }
    };

    // Display the multiplication table vertically
    println!("Multiplication Table (1 to {}):", n);
    for i in 1..=n {
        for j in 1..=10 {
            let result = i * j;
            println!("{} x {} = {}", i, j, result);
        }
        println!("-----------------");
    }
}