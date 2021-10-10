// 8.6. if let

enum Foo {
    Bar,
    Baz,
    Qux(u32)
}


fn main() {
	
	// ex0
	
	// Some(value), a tuple struct that wraps a value with type T.
	// 라고 합니다. 값을 tuple로 바꾸어 줍니다.
	let optional = Some(7);

	match optional {
		Some(i) => {
			println!("This is a really long string and `{:?}`", i);
		},
		_ => {},
	};
	
	// ex1
    let number = Some(7);
	// None, to indicate failure or lack of value, and
	// 값이 없는 상태입니다.
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // `Some(i)`, number 안에 있는 값을 확인합니다.
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

	
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
		// 값이 없는(None) 경우입니다.
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
		// 일반적인 if 구문을 사용할 수 있습니다.
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with an emoticon :)!");
    }
	
	// ex2
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);
    

    if let Foo::Bar = a {
        println!("a is foobar");
    }
	
	// 다음과 같이 일반적은 if에서는 a와 enum::member의 비교가 불가능합니다.
    // if Foo::Bar == a {
    //     println!("a is foobar");
    // }
    
    if let Foo::Bar = b {
        println!("b is foobar");
    }
    
    if let Foo::Qux(value) = c {
        println!("c is {}", value);
    }

    if let Foo::Qux(value @ 100) = c {
        println!("c is one hundred");
    }
}
