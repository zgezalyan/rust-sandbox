fn main() {
    let number = 5;

	if number < 5 {
		println!("Number is less than five");
	} else if number == 5 {
		println!("Number is five");
	} else {
		println!("Number is greater than five");
	}
	
	let value = if number == 5 { 10 } else { 15 }; // `value` will be 10
	println!("Value: {}", value);
	
	let mut counter = 0;
	
	loop {
		println!("Current loop {}", counter);
		if counter > 5 {
			break;  // Without this line, it truly would loop forever
		}
		counter += 1;
	}
	
	counter = 0;	
	
	while counter < 5 {
		println!("Counter: {}", counter);
		counter += 1;
	}
	
	for number in (1..4).rev() {
		println!("Number: {}", number); // This will print 3, 2, then 1
	}
	
	let value = 4;

	match value {
		1 => println!("One"),
		2 => println!("Two"),
		3 | 4 => println!("Three or Four"), // Matches either 3 or 4
		5..=10 => println!("Five through Ten"), // Matches any number between 5 and 10 inclusive
		_ => println!("Anything else"), // Catch-all case
	}
	
	let fun_test = example_function(4);
	println!("Function result: {}", fun_test);
}

fn example_function(value: i32) -> i32 {
    if value == 0 {
        return -1;
    }
    value * 2  // Implicit return if `value` is not 0
}
