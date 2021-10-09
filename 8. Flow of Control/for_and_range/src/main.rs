// 8.4. for and range

fn main() {
    // 1..N은 N미만의 범위 입니다.
    for n in 1..101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
	
	// `1..=N은 N을 포함하는 범위입니다.
    for n in 1..=100 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }
	
	let names = vec!["Bob", "Frank", "Ferris"];

	// iterator를 제공합니다.
	// iter는 소유권을 넘겨받지 않습니다.
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // 소유권을 받지 않기 때문에 &로 대여합니다.
            _ => println!("Hello {}", name),
        }
    }
    
    println!("names: {:?}", names);
	
	let names = vec!["Bob", "Frank", "Ferris"];

	// into_iter는 소유권을 이동시킵니다.
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    
	// 소유권이 넘어갔기 때문에 names에 접근할 수 없습니다.
    // println!("names: {:?}", names);
	
	
	let mut names = vec!["Bob", "Frank", "Ferris"];

	// mut을 이용하여 값을 변경합니다.
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }

    println!("names: {:?}", names);
}
