// 9.1. Methods

struct Point {
    x: f64,
    y: f64,
}

impl Point {
	// "associated function"라고 부릅니다.
	// 객체를 만들지 않아도 접근이 가능합니다.
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // method입니다.
	// "associated function"와의 차이는 self를 인자로 받습니다.
	// python 문법과 유사합니다.
	// 반드시 객체를 만들어서 접근해야 합니다.
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // mut 예약어를 통해 값을 변경 할 수 있습니다.
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
		// 소유권이 넘어갑니다.
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);
		// first 와 second가 지워집니다.
    }
}

fn main() {
    let rectangle = Rectangle {
        // ::을 통해 associated function에 접근 할 수 있습니다.
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };


    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

}