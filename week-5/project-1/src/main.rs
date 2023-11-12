 // Rust program to find the roots of a quadratic equation

 use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the first coefficient: ");
    io::stdin().read_line(&mut input1).expect("Display an integer");
    let a:f32 = input1.trim().parse().expect("Display an error");

    println!("Enter the second coefficient: ");
    io::stdin().read_line(&mut input2).expect("Display an integer");
    let b:f32 = input2.trim().parse().expect("Display an error");

    println!("Enter the third coefficient: ");
    io::stdin().read_line(&mut input3).expect("Display an integer");
    let c:f32 = input3.trim().parse().expect("Display an error");

    // Find the discriminant
    let d = b * b - 4.0 * a * c ;

    // Determine the nature of the roots
    if d > 0.0 {
        println!("The equation has distinct roots");
    }
    else if d == 0.0{
        println!("The equation has one real root");
    }
    else if d < 0.0 {
        println!("The equation has o real root");
    }
    else {
        println!("The equatiom cannot be solved");
    }

    // Solving for the roots
    let x = -b + (d).sqrt();
    let x1 = x / (a * 2.0);
    println!("the value of x1 is {}",x1 );
    let y = -b - (d).sqrt();
    let y1 = y / (a * 2.0);
    println!("The value of x2 is {}",y1 );

}