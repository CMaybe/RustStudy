fn main() {
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
	println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // C/C++과 python의 적절한 조화같습니다.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    println!("{number:>width$}", number=1, width=6);
    println!("{number:0>width$}", number=1, width=6);
    println!("My name is {0}, {1} {0}", "Bond", "James");
	
	// Activites
	let pi = 3.141592;
	
	println!("PI is roughly {:.3}",pi);
    #[allow(dead_code)]
    struct Structure(i32);
	
	// 출력에 대한 부분을 따로 처리 해주어야 합니다.
    // println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.
}
