fn main() {
	let qt:f64 = 2.0;
	let t:f64 = 450_000.0;
	let qm:f64 = 1.0;
	let m:f64 = 1_500_000.0;
	let qh:f64 = 3.0;
	let h:f64 = 750_000.0;
	let qd:f64 = 3.0;
	let d:f64 = 2_850_000.0;
	let qa:f64 = 1.0;
	let a:f64 = 250_000.0;

	// sum
	let sum = (qt * t) + (qm * m) + (qh * h) + (qd * d) + (qa * a);
	println!(" Sum is {} ", sum);
	let qty = qt + qm + qh + qd + qa;

	// average
	let a = sum/qty; 
	println!("Average is {}", a);
}