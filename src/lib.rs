
// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     vec![]
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn one_result() {
//         let query = "duct";
//         let contents = "\
        
// Rust:
// Safe, fast, productive.
// Pick Three";
//         assert_eq!(vec!["Safe, fast, productive"], search(query, contents));

//     }
// }


//  Working with environment variables - adding an extra feature for a case INsensitive searching the user can turn on via an environment variable - via the CLI..

// Writing a Failing Test for the Case-Insensitive 'search' Function

// - we first add a new 'search_case_insensitive' function that will be called when the environment has a value. We'll follow the TDD process, we'll write a failing test FIRST. We'll then add a new test for the new
// 'search_case_insensitive' function and rename our old test from 'one_result' to 'case_sensitive' to clarify the differences btn the two tests:


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn case_sensitive() {
//         let query = "duct";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Duct tape. ";

//         assert_eq!(vec!["safe, fast, productive."], search(query, contents)); 

//     }

//     #[test]
//     fn case_insensitive() {
//         let query = "rUsT";
//         let contents = "\
        
// Rust:
// safe, fast, productive.
// Pick three.
// Trust me. ";

//         assert_eq!(
//             vec!["Rust:", "Trust me."],
//             search_case_insensitive(query, contents)
//         );
//     } 
// Note that we've edited the old test's contents too. We've added a nwe line with the text 'Duct tape'. Using a capital "D" that should't match the query 'duct'
// when we're searching in a case-sensitive amnner. Changing the old test in this way helps ensure that we don't accidentally break the case-sensitive sarch functionality that we've already implemented.
// This test shouldn pass now and should continue to pass as we work on the case-insensitive search. 
// The new test for the 'case-INsensitive' search uses 'rUsT' as its query. In the 'search_case_insensitive' function we're about to add , the query 'rUsT' should match the line containing "Rust:" with a capital R and match the line "Trust me." even though both have different casing from the query. This is our failing test, and it will fail to compile because we have'nt yet defined the 'search_case_insensitive' function.
 
// IMPLEMENTING the 'search_case_insensitive' FUNCTION

// Filename: src/lib.rs
    
//     pub fn search_case_insensitive<'a>(
//         query: &str,
//         contents: &'a str,       
//     ) -> Vec<&'a str> {
//         let query = query.to_lowercase();
//         let mut results = Vec::new();

//         for line in contents.lines() {
//             if line.to_lowercase().contains(&query) {
//                 results.push(line);
//             }
//         }

//         results
//     } // First, we lowecase the 'query' string and store it in a new variable with the same name, shadowing the original.
// //     // Calling 'to_lowercase' on the query is neccessary so that no matter whether the user's query is ( "rust, "RUST", "Rust", or "rUsT") we'll treat the query as if it were "rust" and be insensitive to the case. While to 'lowercase' will handle basic Unicode, it won't be 100% accurate.If we were writing a real application, we would need to do a bit work here, but this is about environmental variables, not Unicode, so we'll let it slide 
// //     // Note that 'query' is now a String rather than a string slice because calling 'to_lowercase' creates new data rather than referencing existing data. Say the query is "rUsT", as an example: that string slice doesn't contain a lowercase 'u' or 't' for to use, so we have to allocate a new String "rust". When we pass 'query' as an argument to the 'contains' method now, we need to add an ampersand because the signature of 'contains' is defined to take a string slice.
// //     // Next, we add a call to 'to_lowercase' on each 'line' to lowercase all characters. Nowthat we've converted 'line' and 'query' to lowercase, we'll find mathces no matter what the case of the query is.

// }

// // lib.rs

// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     contents
//         .lines()
//         .filter(|line| line.contains(query))
//         .collect()
// }

// pub fn search_case_insensitive<'a>(
//     query: &str,
//     contents: &'a str,
// ) -> Vec<&'a str> {
//     let query = query.to_lowercase();
//     let mut results = Vec::new();

//     for line in contents.lines() {
//         if line.to_lowercase().contains(&query) {
//             results.push(line);
//         }
//     }

//     results
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn case_sensitive() {
//         let query = "duct";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Duct tape.";

//         assert_eq!(vec!["safe, fast, productive."], search(query, contents));
//     }

//     #[test]
//     fn case_insensitive() {
//         let query = "rUsT";
//         let contents = "\
// Rust:
// safe, fast, productive.
// Pick three.
// Trust me.";

//         assert_eq!(
//             vec!["Rust:", "Trust me."],
//             search_case_insensitive(query, contents)
//         );
//     }
// }


// Calling the 'search_case_insensitive' function from the 'run' function.First we create a configuration option to the 'Config' struct to switch between the case-sensitive and 
// case-insensitive search. This field addition will cause compilation errors...

// Filename: src/lib.rs

// use std::env;
// use std::error::Error;
// use std::fs;

    // pub struct Config {
    //     pub query: String,
    //     pub file_path: String,
    //     pub ignore_case: bool,
    // }

// We added the 'ignore_case' field that holds the a Boolean. Next, we need the 'run' function to check the 'ignore_case' field's value and use that to decide whether to call the 'search' function or the 'search_case_insensitive' function.
// Thii still won't compile;

//Filename: src/lib.rs

use std::error::Error;
use std::env;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query)) 
        .collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}