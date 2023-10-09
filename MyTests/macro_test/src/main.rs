macro_rules! say_hello {
    () => {
        println!("Hello, World!");
    };
}

macro_rules! test {
    ($x:expr) => {
        println!("Expression is: {}", $x);
    };
    ($x:expr, $y:expr) => {
        println!("Expressions are: {}, {}", $x, $y);
    };
}

macro_rules! create_array {
    ($($x:expr),*) => {
        [ $($x),* ]
    };
}

fn main() {
    say_hello!();
	test!(5);
	test!(5, 8);
	let a = create_array!(1, 2, 3, 4);
	for x in a {
		println!("{}", x);
	}
}
