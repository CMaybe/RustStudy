// 8.2.2. Returning from loops

#![allow(unreachable_code)]

fn main() {
    let mut counter = 0;
	// if문과 동일하게 loop 내에서 값을 받을 수 있습니다.
    let result = loop {
        counter += 1;

        if counter == 10 {
			// break가 return의 역할의 해줍니다
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
