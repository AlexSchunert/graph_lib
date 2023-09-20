use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn read_txt_file(filenname: String) -> String {
    // Create path
    let path = Path::new(&filenname);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("Could not open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut input_file_content = String::new();
    match file.read_to_string(&mut input_file_content) {
        Err(why) => panic!("Could not read {}: {}", display, why),
        Ok(_) => print!("Read file successfully\n"),
    };

    return input_file_content;
}

pub fn write_text_file(output_string: &String) {
    // Create path
    let path = Path::new("Histogram.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("Could not open {}: {}", display, why),
        Ok(file) => file,
    };

    // Write
    match file.write(output_string.as_bytes()) {
        Err(why) => panic!("Could not write {}: {}", display, why),
        Ok(_) => print!("File {} successfully", display),
    };
}
