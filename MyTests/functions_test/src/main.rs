mod greetings;

fn main() {
    greet();
	let sum = add(2, 15);
	println!("Sum = {}", sum);
	
	let c = Circle::new(5.0);  // Calling associated function
	println!("Area of the circle: {}", c.area());  // Calling method
	
	greetings::hello();
}

fn greet() {
    println!("Hello, world!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon means this is an expression returning a value
}

struct Circle {
    radius: f64,
}

impl Circle {
    // Associated function
    fn new(radius: f64) -> Circle {
        Circle { radius }
    }

    // Method
    fn area(&self) -> f64 {
        3.141592 * self.radius * self.radius
    }
}
