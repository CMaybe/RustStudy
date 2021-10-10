// 8.5.1.3. pointers/ref

fn main() {
    let reference = &4;

    match reference {
        // reference변수가 reference(&) 이기 때문에
		// val에도 &를 붙입니다.
        &val => println!("Got a value via destructuring: {:?}", val),
    }

	// *을통해 바로 값에 접근 할 수 있습니다
    match *reference {
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    let _not_a_reference = 3;

    // ref 예약어를 통해 ref 변수를 선언 할 수 있습니다.
    let ref _is_a_reference = 3;

	
    let value = 5;
    let mut mut_value = 6;

    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }
    match mut_value {
        ref mut m => {
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }
	
}
