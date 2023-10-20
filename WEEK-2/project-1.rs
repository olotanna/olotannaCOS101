fn main() {
	let principal = 520_000_000.0;
	let rate = 10.0;
	let n = 5.0;

	//compound interest
	let amount = principal * (1.0 + (rate / 100.0))*(n);
	let compound_interest = amount - principal;
	println!("The compound interest is {}", compound_interest); 
}