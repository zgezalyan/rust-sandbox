fn main() {
    let x = 5; // x is immutable
	println!("{}", x);
	let mut y = 7; // y is mutable
	println!("{}", y);
	y = 8; // This is valid because y is mutable
	println!("{}", y);
	let tup: (i32, f64, char) = (500, 6.4, 'J');
	println!("{}, {}, {}", tup.0, tup.1, tup.2);
	let arr: [i32; 5] = [1, 2, 3, 4, 5];
	for x in arr {
        print!("{} ", x);
    }
	println!();
	let mut s = String::from("Hello");
	println!("{}", s);
	s.push_str(", world!");
	println!("{}", s);
	let guess: u32 = "42".parse().expect("Not a number!");
	println!("{}", guess);
}
