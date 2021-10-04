// 3.3 Constants

static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10; // 상수 선언
/*
이거 보고 짜피 let쓰고 mut안쓰면 상수 아닌가 생각을 했었는데,
let은 전역변수로 선언이 안됩니다.
또한 const 변수의 값은 상수 표현식만 가능합니다.
*/

// let a :i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn main() {
    let n = 16;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
	// 상수는 수정할 수 없습니다.
    // THRESHOLD = 5;
}
