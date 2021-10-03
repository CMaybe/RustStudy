use std::fmt; 

#[derive(Debug)]
struct MinMax(i64, i64);

// C++ 에서 out stream overriding 하는거랑 비슷한 느낌
impl fmt::Display for MinMax {
	// 람다 형식인거 같다
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "({}, {})", self.0, self.1) // 위치 표현
	}
}

#[derive(Debug)]
struct Point2D{
	x: f64,
	y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 변수명이 지정됨
        write!(f, "내가 원하는 출력 형식 x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);
    
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

	
	
    println!("Compare points:");
    println!("Display: {}", point); 
    println!("Debug: {:?}", point);

}