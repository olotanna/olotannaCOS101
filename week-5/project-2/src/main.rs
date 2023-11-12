use std::io;

fn main() {
    let mut input1 = String::new();
    
    println!("Enter the employees age: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let age:f32 = input1.trim().parse().expect("Not a valid number");

    if age >= 40.00
    {
        println!("You are experienced and incentive is N1,560,000.00");
    }
    else if age > 30.00 && age < 40.00 
    {
        println!("You are experienced and incentive is N1,480,000.00");
    }
    else if age >= 18.00 && age <= 28.00
    {
        println!("You are experienced and incentive is N1,300,000.00");
    } 
    else {
        println!("You are inexperienced with an incentive of N100,000.00");
    }
}
