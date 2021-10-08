// 7. Expressions
fn main() {
    // statement
    // statement
    // statement
	
    let x = 5;

    x;
    x + 1;
    15;
	
	let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 세미클론을 쓰지 않습니다.
        x_cube + x_squared + x
    };

    let z = {
        // 여기서 세미콜론을 쓰면 `()` 가 `z`에 할당됩니다.
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
