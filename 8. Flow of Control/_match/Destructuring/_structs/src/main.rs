// 8.5.1.4. structs

fn main() {
    struct Foo {
        x: (u32, u32),
        y: u32,
    }


    let foo = Foo { x: (1, 2), y: 3 };

    match foo {
		// 접근하는 변수의 이름을 명시해줍니다.
        Foo { x: (1, b), y } => println!("First of x is 1, b = {},  y = {} ", b, y),

        // 순서는 중요하지 않습니다.
        Foo { y: 2, x: i } => println!("y is 2, i = {:?}", i),

        // .. 통해 다른 변수를 무시 할 수 있습니다.
        Foo { y, .. } => println!("y = {}, we don't care about x", y),
    }
}