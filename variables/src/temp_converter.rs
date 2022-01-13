use std::io;


fn convert_temp() {
	loop {
		println!("Choississez une conversion");
		println!("1- F->C");
		println!("2- C->F");

		let mut conversionMode = String::new();
		io::stdin().read_line(&mut conversionMode)
	}
}
