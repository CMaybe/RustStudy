// 8.5.1.1. tuples

fn main() {
    let triple = (0, -2, 3);
    println!("Tell me about {:?}", triple);
    match triple {
        // 값을 분리 할 수 있습니다.
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
		
		// ..을 통해 값을 무시 할 수 있습니다.
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
		
        _      => println!("It doesn't matter what they are"),
    }
}
