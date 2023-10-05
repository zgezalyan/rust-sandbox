struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(u8, u8, u8);

struct Unit;

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
	let home = IpAddr::V4(127, 0, 0, 1);
	let loopback = IpAddr::V6(String::from("::1"));
	
	let some_number = Some(5);
	let some_string = Some("a string");
	let absent_number: Option<i32> = None;
    println!("Tested");
}
