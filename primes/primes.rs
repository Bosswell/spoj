use std::io;

fn main() {
	let mut x = String::new();
	
	io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");
	
	let x = x.trim().parse::<u32>().unwrap();
	
	for _i in 0..x {
		let mut n = String::new();
		
		io::stdin()
			.read_line(&mut n)
			.expect("Failed to read line");
		
		let n = n.trim().parse::<u32>().unwrap();
		
		match is_prime(n) {
			true  => println!("TAK"),
			false => println!("NIE")
		}
	}
}

fn is_prime(x: u32) -> bool {
	if x < 2 {
		return false;
	}
	
	let mut i = 2;
	
	while i*i <= x {
		if x%i == 0 {
			return false;
		}
		i = i + 1;
	}
	
	return true;
}