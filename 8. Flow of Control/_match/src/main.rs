// 8.5. match

fn main() {
    let number = 13;
    // TODO ^ Try different values for `number`

    println!("Tell me about {}", number);
	// c++의 switch case 문입니다.
    match number {
        1 => println!("One!"),
		
		// 이런식으로 or 연산이 가능합니다.
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        
		// case를 범위로 설정하는 것이 가능합니다.
        13..=19 => println!("A teen"),
        
		// default 입니다.
        _ => println!("Ain't special"),
    }

    let boolean = true;
	
	// match를 통해 값을 할당 할 수 잇습니다.
    let binary = match boolean {
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);
}
