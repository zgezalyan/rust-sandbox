fn largest<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

trait Summarizable {
    fn summary(&self) -> String;
}

struct NewsArticle {
    headline: String,
    // ... other fields ...
}

impl Summarizable for NewsArticle {
    fn summary(&self) -> String {
        format!("(News) {}", self.headline)
    }
}

fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn some_function<'a, T>(arg: &'a T) where T: Summarizable + 'a {
    println!("{}", arg.summary());
}

fn main() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
	println!("{}", largest(&arr));
	
	let article = NewsArticle {
		headline: String::from("Rust is Awesome!"),
		// ... other fields ...
	};

	println!("Article: {}", article.summary());
	
	let a = "Slow";
	let b = "Slower";
	
	println!("{}", longest(&a, &b));
	
	some_function(&article);
}
