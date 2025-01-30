use std::{fs::OpenOptions, io::Write, thread, time::Duration};
use std::path::Path;

fn main() {
    let path = Path::new("/tmp/hello.txt");

    loop {
        // Open the file in append mode
        let mut file = OpenOptions::new()
            .create(true)  // Create the file if it doesn't exist
            .append(true)  // Open the file in append mode
            .open(path)
            .expect("Failed to open the file");

        // Write a message to the file
        writeln!(file, "Appended message at {:?}", chrono::Local::now())
            .expect("Failed to write to the file");

        println!("Message appended to file at {:?}", chrono::Local::now());

        // Sleep for 60 seconds (1 minute)
        thread::sleep(Duration::from_secs(60));
    }
}
