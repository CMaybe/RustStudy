// 4.1. Mutability

fn main() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1;
	// mut을 써줘야 가변성이 보장된다.
    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // _immutable_binding += 1;
}
