use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
    // Open a file for writing
    let mut file = File::create("output.txt")?;

    // Write data to the file
    file.write_all(b"Hello, World!")?;

    // Open a file for reading
    let file = File::open("output.txt")?;
    let reader = BufReader::new(file);

    // Read the contents of the file line by line
    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}

/*
In this example, we perform the following file I/O operations:

Creating a File: We use the File::create function to create a new file named output.txt. The function returns a Result that represents the outcome of the operation.

Writing to a File: We use the write_all method on the File object to write the byte slice b"Hello, World!" to the file. The method writes all the data at once.

Opening a File for Reading: We use the File::open function to open the file output.txt for reading. The function returns a Result that represents the outcome of the operation.

Reading from a File: We create a BufReader by wrapping the opened file. The BufReader provides a buffered interface for reading lines from the file efficiently.

Reading File Line by Line: We iterate over the lines of the file using the lines method on the BufRead trait. Each line is printed to the console using println!. The ? operator is used to handle any potential errors that may occur during the line reading process.

Note: In this example, the io::Result<()> return type indicates that the main function returns a result that captures any potential I/O errors.

When running the program, it will create a file named output.txt, write the text "Hello, World!" to it, and then read and print the contents of the file.

You can modify this example to suit your specific file I/O needs, such as reading from a file, appending to a file, or processing file contents in different ways.

*/
