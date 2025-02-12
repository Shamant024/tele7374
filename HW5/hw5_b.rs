use std::thread;

fn sum4(a: i32, b: i32, c: i32, d: i32) -> i32 {

	let handle1 = thread::spawn(move || {
		a + b
	});
	
	let handle2 = thread::spawn(move || {
		c + d
	});


	let sum1 = handle1.join().unwrap();
	let sum2 = handle2.join().unwrap();

	sum1 + sum2
}


fn main() {
	for i in 0..=10 {
		let result = sum4(i, i, i, i);
		println!("{}x4 = {}", i, result);
	}
	
	std::process::exit(0);
}
