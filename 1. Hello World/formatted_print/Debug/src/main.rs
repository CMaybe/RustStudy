#[derive(Debug)]
struct Structure(i32);


#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main(){
	
	// {?} 는 derive(Debug) format을 따른다. 
	println!("{:?} months in a year",12);
	println!("{1:?} {0:?} is the {actor:?} name.","Slater","Christian",actor="actor's");
	
	println!("Now {:?} will print!",Structure(3));
	println!("Now {:?} will print!",Deep(Structure(7)));
	
	let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
	
	
	// #은 개행의 차이인듯
	println!("{:?}", peter);
	println!("{:#?}", peter);
}
