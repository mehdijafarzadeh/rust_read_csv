use std::error::Error;

// Function to read from a CSV file and print each record to the console
fn read_from_file(path: &str) -> Result<(), Box<dyn Error>> {
    // Initialize a CSV reader using the specified file path
    let mut reader = csv::Reader::from_path(path)?;

    // Iterate over each record in the CSV file
    for result in reader.records() {
        // Check if the record is valid
        match result {
            Ok(record) => println!("{:?}", record), // Print the record if it's valid
            Err(e) => return Err(Box::new(e)),      // Return an error if reading a record fails
        }
    }
    Ok(())
}

fn main() {
    // Attempt to read from the CSV file and handle any potential errors
    if let Err(e) = read_from_file("./customers.csv") {
        eprintln!("{}", e); // Print the error to the standard error output
    }
}
