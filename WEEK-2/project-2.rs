fn main() {
	//amount
	let t:f64 = 450_000.0;
	let m:f64 = 1_500_000.0;
	let h:f64 = 750_000.0;
	let d:f64 = 2_850_000.0;
	let a:f64 = 250_000.0;

	//quantity
	let tq:f64 = 2.0;
	let mq:f64 = 1.0;
	let hq:f64 = 3.0;
	let dq:f64 = 3.0;
	let aq:f64 = 1.0;

	//sum
	let sum = (t * tq) + (m * mq) + (h * hq) + (d * dq) + (a * aq);
	println!("The sum is {}", sum);
	let sq = tq + mq + hq + dq + aq;
	println!("The sum of quantity is {}", sq);
	let mean = sum / sq;
	println!("The mean is {}", mean);
	}