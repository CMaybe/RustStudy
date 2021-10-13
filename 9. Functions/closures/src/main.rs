fn main() {
    fn function(i: i32) -> i32 { i + 1 }

    // 아래의 두 코드는 같은 동작을 합니다.
	// 둘중 무엇을 선택해도 상관은 없습니다.
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred  = |i| i + 1  ;

    let i = 1;
    println!("function: {}", function(i));
    println!("closure_annotated: {}", closure_annotated(i));
    println!("closure_inferred: {}", closure_inferred(i));

    let one = || 1;
    println!("closure returning one: {}", one());

}
