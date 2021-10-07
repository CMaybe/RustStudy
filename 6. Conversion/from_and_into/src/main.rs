// 6.1 Conversion
use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}


// from 대해한 구현체입니다.
impl From<i32> for Number {
	// 
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let num = Number::from(30);
    println!("My number is {:?}", num);
	
	let int = 5;
    // into는 from의 역산입니다.
	// num이 Number type이기 때문에 가능한 연산입니다.
	// 개인적으로 신기한게 좌변에 의존적인 연산입니다.
    let num: Number = int.into();
    println!("My number is {:?}", num);
}
