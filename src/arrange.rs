// Ascending order
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
	let file = std::env::args().nth(1).expect("Give a file to sort!");
	make_file("docs/ascending.csv".to_string());
	make_file("docs/descending.csv".to_string());
	read(file);
	println!("Have a look at docs/ascending.csv and/or docs/descending.csv!");
}

fn read(filename: String) {
	let mut file = std::fs::File::open(&filename).unwrap();
	let mut contents = String::new();
	file
		.read_to_string(&mut contents)
		.unwrap();
	let contents = &contents.to_string();

	let split = contents.split(", "); //splitting
	let vec: Vec<&str> = split.collect(); //collecting
	let mut vect = vec![]; //vect1 (for f64 val)
	let mut vect2 = vec![]; //vector2 (for String val)
	let mut vect4: Vec<String> = vec![];

	for num in 0..vec.len() { //Adding to vector
		let fnum: f64 = vec[num].parse().unwrap();
		vect.push(fnum);
	}

	vect.sort_by(|a, b| a.partial_cmp(b).unwrap()); //Sorting

	for num in 0..vect.len() { //for String
		let a_num = vect[num].to_string(); //conversion
		vect2.push(a_num); //pushing
	}

	let vect3: Vec<f64> = vect.into_iter().rev().collect(); //reversing the vec

	for num in 0..vect3.len() {
		let ab = vect3[num].to_string();
		vect4.push(ab);
	}

	let vect2 = vect2.join(", "); //joining as string
	let vect4 = vect4.join(", "); //joining as string
	write_to_file("docs/ascending.csv".to_string(), vect2);
	write_to_file("docs/descending.csv".to_string(), vect4);
}

fn make_file(file_name: String) {
    std::fs::File::create(&file_name).expect("\nCreation failed");
    println!("\n\nCreated file named {} \nPATH: docs/{}\n", &file_name, &file_name);
}

fn write_to_file(file: String, text: String) {
	let mut file = OpenOptions::new()
		.append(true)
		.open(&file)
		.expect("\nCannot Open File.");

	file
		.write_all(text.as_bytes())
		.expect("\nWrite Failed");

	println!("Successfully appended sorted list of numbers to file {:?}.", file);
}
