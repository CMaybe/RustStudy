// 5.4 Aliasing
// 타입의 이름을 재정의합니다.
type NanoSecond = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;

fn main() {
	
	// 단순하게 이름만 재정의 했기 때문에 casting 이나 연산은
	// 기존의 자료형과 동일합니다.
    let nanoseconds: NanoSecond = 5 as u64_t;
    let inches: Inch = 2 as u64_t;

    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);
}
