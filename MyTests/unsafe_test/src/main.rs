unsafe fn dangerous_function() {
    println!("This function is very dangerous");
}

fn main() {
    let x: i32 = 10;
	let r: *const i32 = &x;
	unsafe {
		println!("r points to: {}", *r);
		dangerous_function();
	}
}
