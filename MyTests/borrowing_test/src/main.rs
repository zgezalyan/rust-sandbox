fn main() {
    let s = String::from("hello");
	println!("{}", s);
	let r1 = &s;
	println!("{}", r1);
	let r2 = &s;
	println!("{}", r2);
	// let r3 = &mut s; // This would cause an error since we already have immutable references
}
