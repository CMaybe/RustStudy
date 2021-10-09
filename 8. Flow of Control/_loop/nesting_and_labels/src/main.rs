// 8.2.1. Nesting and labels

#![allow(unreachable_code)]

fn main() {
	// 이렇게 라벨을 붙여 ('label) 원하는 루프에 접근 할 수 있습니다.
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // 내부 loop break
            //break;

            // 'outer break
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}
