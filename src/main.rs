// A simple code to make 'x' amount of random numbers(floats).

// Importing rand, and std.
use rand::Rng;
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    //Main
    let file = std::env::args().nth(1).expect("Give File"); // Getting file name.
    let numofnums: i32 = std::env::args()
        .nth(2) //Second value
        .expect("Give Num - only i32") //Error response
        .parse()
        .unwrap(); //Converting to i32

    let mut rng = rand::thread_rng(); //making rng to get random nums
    let mut nums = vec![]; //Making an empty vector to store values.

    // for loop to make 'numofnums' rand nums
    for _num in 0..numofnums {
        let num = rng.gen_range(0.1..1000.0); //Generating
        let num = num.to_string(); //Converting nums to string (to add to file)
        nums.push(num); //Pushing to vector
    }

    let fnums = nums.join(",\n"); //Joining the vector with ',' and '\n'(newline) to convert the vector to string.
    println!("{} numbers", nums.len()); //Printing the number of numbers. ;-)

    create_file(file.to_string()); //Creating file
    write_to_file(file.to_string(), fnums, numofnums); //Writing to file
}

fn create_file(file_name: String) {
    std::fs::File::create(&file_name).expect("\nCreation failed"); //Making file
    println!("\n\nCreated file named {}\n", file_name); //Error response
}

fn write_to_file(file_name: String, text: String, numofnums: i32) {
    let mut file = OpenOptions::new()
        .append(true) //Appending
        .open(&file_name)
        .expect("\nCannot Open File."); //Error response

    file.write_all(text.as_bytes()) //As bytes
        .expect("\nWrite Failed"); //Error response

    println!(
        "Successfully appended {} random numbers to file {}.",
        numofnums, file_name
    ); //Printing confirmation line.
}
