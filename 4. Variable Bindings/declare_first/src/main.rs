fn main() {
	// 선언과 정의가 분리되어도 상관은 없습니다.
    let a_binding;

    {
        let x = 2;

        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

	// 정의되지 않은 변수에 접근하면 에러를 뱉습니다.
    // println!("another binding: {}", another_binding);
    
    another_binding = 1;

    println!("another binding: {}", another_binding);
}