// Sorting

// Importing std
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;

pub fn sort() {
    //Main
    let file = std::env::args().nth(2).expect("Give a file to sort!"); //Getting file to sort
                                                                       //Making Ascending and descending.csv files.
    make_file("ascending.csv".to_string());
    make_file("descending.csv".to_string());
    read(file); //reading file
    println!("Have a look at ascending.csv and/or descending.csv!"); //After all
}

fn read(filename: String) {
    let mut file = std::fs::File::open(&filename).unwrap(); //Getting file and unwrapping
    let mut contents = String::new(); //Making a string for contents
    file.read_to_string(&mut contents) //reading
        .unwrap();
    let contents = &contents.to_string(); //Converting to string(as it is stored in f64 format).

    let split = contents.split(",\n"); //splitting the contents
    let vec: Vec<&str> = split.collect(); //collecting them
    let mut vect = vec![]; //vect1 (for f64 val) [ascending]
    let mut vect2 = vec![]; //vector2 (for String val) [ascending]
    let mut vect4: Vec<String> = vec![]; // for descending

    for num in 0..vec.len() {
        //Adding to vector
        let fnum: f64 = vec[num].parse().unwrap(); //collecting the numbers seperately
        vect.push(fnum); //pushing to vector
    }

    vect.sort_by(|a, b| a.partial_cmp(b).unwrap()); //Sorting

    for num in 0..vect.len() {
        //for loop [for String]
        let a_num = vect[num].to_string(); //conversion to string
        vect2.push(a_num); //pushing to vector
    }

    let vect3: Vec<f64> = vect.into_iter().rev().collect(); //reversing the vec [for descending]

    for num in 0..vect3.len() {
        //for loop [for descending]
        let ab = vect3[num].to_string(); //converting to string
        vect4.push(ab); //pushing
    }

    let vect2 = vect2.join(",\n"); //joining as string
    let vect4 = vect4.join(",\n"); //joining as string
    write_to_file("ascending.csv".to_string(), vect2); //Writing
    write_to_file("descending.csv".to_string(), vect4); //Writing
}

fn make_file(file_name: String) {
    //Making file (same as in main.rs)
    std::fs::File::create(&file_name).expect("\nCreation failed");
    println!(
        "\n\nCreated file named {} \nPATH: {}\n",
        &file_name, &file_name
    );
}

fn write_to_file(file: String, text: String) {
    let mut file = OpenOptions::new() //Opening
        .append(true) //Appening
        .open(&file)
        .expect("\nCannot Open File."); //Error response

    file.write_all(text.as_bytes()) //writing as bytes
        .expect("\nWrite Failed"); //Error response

    println!(
        "Successfully appended sorted list of numbers to file {:?}.",
        file
    ); //printing ans(for user)
}
