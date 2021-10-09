// 8.2. loop

fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // 기본적으로 무한 루프입니다.
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
}
