use std::io;

fn main() {
    let mut count = 0;
    while count < 500{
    
    let mut name = String::new();
    let mut number_of_paper = String::new();

    println!("Enter your name: ");
    io::stdin().read_line(&mut name).expect("Not a valid input");

    println!("Number of papers published: ");
    io::stdin().read_line(&mut number_of_paper).expect("Not a valid input");
    let number_of_paper:i32 = number_of_paper.trim().parse().expect("Not a valid integer");

    if number_of_paper >= 3 && number_of_paper <= 5{
        println!("Your incentive is N500000");
    } else if number_of_paper >= 5 && number_of_paper < 10 {
        println!("Your incentive is N800000");
    } else if number_of_paper > 10 {
        println!("Your incentive is N1000000");
    } else {
        println!("Your incentive is N100000");
    }
    count+=1;
  }  
}
