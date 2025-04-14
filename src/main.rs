use std::env;

fn main() {
   let args: Vec<String> = env::args().collect();
   dbg!(args);

// Saving argument values in Variables

// fn main() {
   let args: Vec<String> = env::args().collect();

   let query = &args[1];
   let file_path = &args[2];

   println!("Searching for {query}");
   println!("IN file {file_path}");
//}

   use std::fs;

// fn main() {

// --snip-- 

   println!("In file {file_path}");

   let contents = fs::read_to_string(file_path)
      .expect("Should have been able to read the file");

   println!("With text: \n{contents}");
//}

}
