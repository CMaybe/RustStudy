// 6.3. To and from Strings
use std::fmt;

struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
	// to_string은 Display의 구현체를 따라갑니다.
    println!("{}", circle.to_string());
	
	
	// 좌항의 타입이 정해진 경우 parse()만 해주면 됩니다.
	let parsed: i32 = "5".parse().unwrap();
	
	// 하지만 좌항이 literal일 경우 type을 결정해줘야 합니다.
    let turbo_parsed = "10".parse::<i32>().unwrap();
	
    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
}

