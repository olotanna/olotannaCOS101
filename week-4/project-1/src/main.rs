use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

    println!("Enter the distance of car: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let mut d:f32 = input1.trim().parse().expect("Not a valid number");
    //change to kilometer
    d = d * 1.68934;
    println!("The distance covered by the car in km is: {} ",d);

    println!("Enter the time taken: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let t:f32 = input2.trim().parse().expect("Not a valid number");

   let s:f32 = d / t ;
   let area = s;

   println!("The  speed of the car is {} km/hr ",area );

}
 