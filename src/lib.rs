
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

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


// Working with environment variables - adding an extra feature for a case INsensitive searching the user can turn on via an environment variable - via the CLI..

// Writing a Failing Test for the Case-Insensitive 'search' Function

// - we first add a new 'search_case_insensitive' function that will be called when the environment has a value. We'll follow the TDD process, we'll write a failing test FIRST. We'll then add a new test for the new
// 'search_case_insensitive' function and rename our old test from 'one_result' to 'case_sensitive' to clarify the differences btn the two tests:


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape. ";

        assert_eq!(vect!["safe, fast, productive."], search(query, contents)); 

    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
        
Rust:
safe, fast, productive.
Pick three.
Trust me. ";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}