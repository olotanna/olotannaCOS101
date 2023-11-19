// Rust Program For Otunene Family Health Centre by Obiakor Dino Lotanna
use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
    let mut input4 = String::new();
    let mut input5 = String::new();
    let mut input6 = String::new();
    let mut input7 = String::new();
    let mut input8 = String::new();
    let mut input9 = String::new();

    println!("\t\tHealth Diagnosis\t\t\tAmount");
    println!(" Alzheimer\t    -N1,200,000");
    println!(" Arrhythmia\t         -N550,000");
    println!(" Chronic Kidney Disease\t\t            -N1,500,000");
    println!(" Diabetes\t\t              -N800,000");
    println!(" Arthritis\t\t           -N450,000");


    println!("Enter your name: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");

    println!("Enter your Date of Birth (Format dd/mm): ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");

    println!("Enter your year of birth: ");
    io::stdin().read_line(&mut input9).expect("Not a valid string");
    let year:i32 = input9.trim().parse().expect("Not a valid number");

    println!("Enter your Email Adress: ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");

    println!("Enter your phone number: ");
    io::stdin().read_line(&mut input4).expect("Not a valid string");

    println!("How many siblings do you have? ");
    io::stdin().read_line(&mut input5).expect("Not a valid string");
    let number_of_siblings:i32 = input5.trim().parse().expect("Not a valid number");

    println!("How many children do you have? ");
    io::stdin().read_line(&mut input6).expect("Not a valid string");
    let number_of_children:i32 = input6.trim().parse().expect("Not a valid number");

    println!("Medical diagnosis:  ");
    io::stdin().read_line(&mut input7).expect("Not a valid string");
    let diagnosis = input7.trim().parse().expect("Not a valid number");

    println!("Village of Residence:  ");
    io::stdin().read_line(&mut input8).expect("Not a valid string");
    let village = input8.trim().parse().expect("Not a valid number");

    let age:i32 = 2023 - year;
    println!("You are {} years old.",age );

    if diagnosis == "Alzheimer" && age > 50 && number_of_children >= 4 && village == "Akpabom"
    {
        let amount:i32 = 1200000 ;
        let discount:f32 = (20 / 100 * amount) as f32 ;
        println!("{}", input1 );
        println!("{}" , input2 );
        println!("{}", input3 );
        println!("{}", input4 );
        println!("{}", number_of_siblings );
        println!("{}", number_of_children );
        println!("{}", diagnosis );
        println!("{}",village );
        println!("Amount is N{}",discount );
    } else {
        println!("{}", input1 );
        println!("{}", input2);
        println!("{}", input3 );
        println!("{}", input4 );
        println!("{}", number_of_siblings );
        println!("{}", number_of_children );
        println!("{}", diagnosis );
        println!("{}",village );
        println!("Amount is 1200000");

    }
    else if diagnosis == "Arrythmia" && age = 30 && number_of_siblings = 4 && village == "Ngbauji"
    {
        let amount2:i32 = 550000 ;
        let discount2:f32 = 5 / 100 * amount2 ;

        println!("{}", input1 );
        println!("{}", input2 );
        println!("{}", input3 );
        println!("{}", input4 );
        println!("{}", number_of_siblings );
        println!("{}", number_of_children );
        println!("{}", diagnosis );
        println!("{}",village );
        println!("Amount is N{}",discount2 );

    } else{
        println!("{}", input1 );
        println!("{}", input2 );
        println!("{}", input3 );
        println!("{}", input4 );
        println!("{}", number_of_siblings );
        println!("{}", number_of_children );
        println!("{}", diagnosis );
        println!("{}",village );
        println!("Amount is 550000" );
    } 
    else if diagnosis == "Chronic Kidney Disease" && age > 40 && number_of_children > 3 && number_of_siblings > 3 && village = "Atabrikang"
    {
        let amount3:i32 = 1500000 ;
        let discount3:f32 = 15 / 100 * amount3 ;
        println!("{}", input1 );
        println!("{}", input2 );
        println!("{}", input3 );
        println!("{}", input4 );
        println!("{}", number_of_siblings );
        println!("{}", number_of_children );
        println!("{}", diagnosis );
        println!("{}",village );
        println!("Amount is N{}",discount3 );
    } else {
        println!("{}", input1 );
        println!("{}", input2 );
        println!("{}", input3 );
        println!("{}", input4 );
        println!("{}", number_of_siblings );
        println!("{}", number_of_children );
        println!("{}", diagnosis );
        println!("{}",village );
        println!("Amount is 1500000" );

    }
    else if diagnosis == "Diabetes" && age > 28 && age < 45 && number_of_children >= 2 && number_of_children <= 4 && village = "Okorobilom" 
    {
        let amount4:i32 = 800000;
        let discount4:f32 = 10 / 100 * amount4 ;
        println!("{}", input1 );
        println!("{}", input2 );
        println!("{}", input3 );
        println!("{}", input4 );
        println!("{}", number_of_siblings );
        println!("{}", number_of_children );
        println!("{}", diagnosis );
        println!("{}",village );
        println!("Amount is N{}", discount4 );

    } else {
        println!("{}", input1 );
        println!("{}", input2 );
        println!("{}", input3 );
        println!("{}", input4 );
        println!("{}", number_of_siblings );
        println!("{}", number_of_children );
        println!("{}", diagnosis );
        println!("{}",village );
        println!("Amount is 800000" );
    }
    else if diagnosis == "Arthritis" && age > 58 && number_of_children = 5 && number_of_siblings = 5 && village = "Emeremen"
    {
    let amount5:i32 = 450000;
    let discount5:f32 = 10 / 100 * amount5;
        println!("{}", input1 );
        println!("{}", input2 );
        println!("{}", input3 );
        println!("{}", input4 );
        println!("{}", number_of_siblings );
        println!("{}", number_of_children );
        println!("{}", diagnosis );
        println!("{}",village );
        println!("Amount is N{}", discount5 );
    } else {
        println!("{}", input1 );
        println!("{}", input2);
        println!("{}", input3 );
        println!("{}", input4 );
        println!("{}", number_of_siblings );
        println!("{}", number_of_children );
        println!("{}", diagnosis );
        println!("{}",village );
        println!("Amount is N450000" );
    }    


}
