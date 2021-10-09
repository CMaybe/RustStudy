// 8.1. if/else

fn main() {
    let n = 5;
	// 괄호가 없는 C/C++ style입니다.
	// 다른건 모르겠고 개인적으로 예제의 개행이 보기 흉합니다.
    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
			// if를 통해 값을 할당 할 수 있습니다. 
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            n / 2
            // 세미콜론을 넣으면 값이 할당되지 않습니다.
        };

    println!("{} -> {}", n, big_n);
}
