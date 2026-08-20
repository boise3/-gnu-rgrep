use rgrep::Config;
use rgrep::run;
use std::{env::args};
use std::process;

fn main() {


    let args: Vec<String>= args().collect();

    let config = Config::build(&args).unwrap_or_else(|error|{
        eprintln!("Problem with parsing args: {error}");
        process::exit(1);
    });
   
    if let Err(e) = run(config){
        eprintln!("application error: {e}");
        process::exit(2)
    }
}

#[cfg(test)]
mod tests{

    use rgrep::SearchEngine;
    
    #[test]
    fn test1(){
        let query = String::from("duct");
        let contents = String::from("\
Rust:
safe, fast, productive.
Pick three.");

        assert_eq!(vec!["safe, fast, productive."], SearchEngine::search(query, contents));
    }

    #[test]
    fn test2() {
        let query = String::from("rUsT");
        let contents = String::from("\
Rust:
safe, fast, productive.
Pick three.
Trust me.");

        assert_eq!(
            vec!["Rust:", "Trust me."],
            SearchEngine::search_case_insensitive(query, contents)
        );
    }
}
