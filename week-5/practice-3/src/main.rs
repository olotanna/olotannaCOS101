 // Rust program to read the height of a person
 // and then print if the person is tall, dwarf,
 // or average height person

 use std::io;

fn main() 
{
    let mut input = String::new;

    println!("Enter your height (in centimetres): ");
    io::stdin().read_line(&mut input).expect("Not a valid string");
    let height:f32 = input.trim().parse().expect("Not a valid number");

    if height >= 150.0 && height <= 170.0
    {
         println!("You are an average height person");
    }

    else if height > 170.0 && height > 100.0
    {
        println!("You are tall");
    }
    else if height < 150.0 && height > 100.0
    {
        println!("You are a dwarf");
    }

    else
    {
        println!("Abnormal height");
    }
}
