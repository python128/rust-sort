// A simple code to make 'x' amount of random numbers(floats).
// Equivalent-ish of "/mnt/c/Users/tejas/Downloads/Tejas/PYTHON PROGRAMS/1000_Numbers.py"

/* Test writing format:-
 * 	'1' = test number
 *  '2' = amount of numbers
 *  '3'(opt) = range(0..range)
 *
 * Eg:
 *  't4-100-100.csv'
 *  t4 = test4 = test number 4
 *  100{1} = amount of numbers.
 *  100{2} = range(opt).
 */

use rand::Rng;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
	let help = "
 Help Message:
 Arguments:
	Necessary:
		File-Name = A name for the file. Append it with '.csv'
		Amount of Numbers = Amount of random numbers to be generated.
	Optional:
		range = range of random numbers. Without, it will be 0-1000(float)
";

	let file = std::env::args().nth(1).expect(help);
	let numofnums: i32 = std::env::args()
		.nth(2)
		.expect(help)
		.parse()
		.unwrap();
	let range: f64 = std::env::args()
		.nth(3)
		.expect(help)
		.parse()
		.unwrap_or(1000.0);

	let mut rng = rand::thread_rng();
	let mut nums = vec![];

	for _num in 0..numofnums {
		let num = rng.gen_range(0.1..range);
		let num = num.to_string();
		nums.push(num);
	}
	
	let fnums = nums.join(", ");
	println!("{} numbers", nums.len());

	let filetowrite = &file;
	create_file(filetowrite.to_string());
	write_to_file(filetowrite.to_string(), fnums, numofnums);
}

fn create_file(file_name: String) {
    std::fs::File::create(&file_name).expect("\nCreation failed");
    println!("\n\nCreated file named {}\n", file_name);
}

fn write_to_file(file_name: String, text: String, numofnums: i32) {
	let mut file = OpenOptions::new()
		.append(true)
		.open(&file_name)
		.expect("\nCannot Open File.");

	file
		.write_all(text.as_bytes())
		.expect("\nWrite Failed");

	println!("Successfully appended {} random numbers to file {}.", numofnums, file_name);
}
