// 8.5.2. Guards

fn main() {
    let pair = (0, 0);
    // TODO ^ Try different values for `pair`

    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // 앞에 if문을 충족하면 뒤에 if문은 무시합니다.
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
	
	let number: u8 = 4;

    match number {
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => println!("Fell through"), // unsigned 변수 이기 때문에 사실상 의미가 없습니다
    }
}
