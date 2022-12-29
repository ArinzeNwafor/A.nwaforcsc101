//Rust program to create an external .txt file
use std::io::Write;

fn main() {

   let announce = "Week 9P - Rust File Input & Output\n";
   let dept = "Department of Computer Science";

   let mut file = std::fs::File::create("data.txt").expect("Create Failed");
   file.write_all("Welcome to Rust programming\n"
         .as_bytes()).expect("write failed");

   file.write_all(announce.as_bytes()).expect("write failed");
   file.write_all(dept.as_bytes()).expect("write failed");
   println!("\nData written to file");

}
