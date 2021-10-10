// 8.5.3. Binding

fn age() -> u32 {
    15
}

fn some_number() -> Option<u32> {
    Some(42)
}

fn main() {
    println!("Tell me what type of person you are");

    match age() {
        0 => println!("I haven't celebrated my first birthday yet"),
        // @를 통해 해당하는 값을 n에 넣을 수 있습니다.
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }
	
	match some_number() {
        // 42를 확인합니다..
        Some(n @ 42) => println!("The Answer: {}!", n),
        // 다른 다른 숫자를 넣어봅시다.
        Some(n) => println!("Not interesting... {}", n),
        _ => (),
    }
}
