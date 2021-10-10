#![allow(unused)]
fn main() {
	let mut optional = Some(0);

	loop {
		match optional {
			Some(i) => {
				if i > 9 {
					println!("Greater than 9, quit!");
					optional = None;
				} else {
					println!("`i` is `{:?}`. Try again.", i);
					optional = Some(i + 1);
				}
			},
			// i가 10이 되어서 optional이 None이 될 때 끝납니다.
			_ => { break; }
		}
	}
	
	// 위의 코드를 간결화 시킨 결과입니다.
	// while let = loop + match
    let mut optional = Some(0);
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
    }
}