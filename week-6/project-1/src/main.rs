use std::io;

fn main() {
    let mut count = 0;

    loop{
        count+=1;
        if count >= 150 {
            println!("Voting has ended. ");
            break;
        }
    }

    let mut class = String::new();
    let mut cgpa = String::new();
    let mut is_class_rep = String::new();
    let mut name = String::new();
    let mut email = String::new();
    let mut department = String::new();
    let mut soo = String::new();

    println!("Please enter your level (100,200,300 or 400): ");
    io::stdin().read_line(&mut class).expect("failed to read input");
    let class: i32 = class.trim().parse().expect("Not a valid integer");

    println!("Please enter your CGPA: ");
    io::stdin().read_line(&mut cgpa).expect("failed to read input");
    let cgpa: f32 = cgpa.trim().parse().expect("Not a valid integer");

    println!("Are you a class rep (true or false): ");
    io::stdin().read_line(&mut is_class_rep).expect("Enter true or false");
    let is_class_rep: bool = is_class_rep.trim().parse().expect("Not a valid integer");

    println!("Please enter your name: ");
    io::stdin().read_line(&mut name).expect("failed to read input");
    let name = name.trim();

    println!("Please enter your emal adress: ");
    io::stdin().read_line(&mut email).expect("failed to read input");
    let email = email.trim();

    println!("Please enter your department: ");
    io::stdin().read_line(&mut department).expect("failed to read input");
    let department = department.trim();

    println!("Please enter your State of Origin: ");
    io::stdin().read_line(&mut soo).expect("failed to read input");
    let soo = soo.trim();

    if is_class_rep && class != 100 && cgpa >= 4.0 && class >= 100
    {
        println!("You can vote");
        println!("{}",name );
        println!("{}",email );
        println!("{}",department );
        println!("{}",soo );
    } else {
        println!("Sorry, you are not eligible to vote");
    }
    








}
