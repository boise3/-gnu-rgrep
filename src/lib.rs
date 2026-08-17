use std::{error::Error, fs};

pub struct SearchEngine;
impl SearchEngine{
   
pub fn search(query: String, content: String) -> Vec<String>{
    
    let mut result: Vec<String> = Vec::new();

    for line in content.lines(){
        if line.contains(&query){
            result.push(line.to_string());
        }
    }
    result
}

pub fn search_word(query: String, content: String) -> Vec<String>{

    let mut result: Vec<String> = Vec::new();

    for line in content.lines(){
        if line.contains(&query){
            result.push(query.to_string());
        }
    }
    result
}

pub fn search_case_insensitive(query: String, content: String) -> Vec<String>{

    let mut result: Vec<String> = Vec::new();

    let query = query.to_lowercase();
    
    for line in content.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line.to_string());
        }
    }
    result
}

}


pub fn run (config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        SearchEngine::search_case_insensitive(config.query, contents)
    }else if config.only_word {
        
        SearchEngine::search_word(config.query, contents)

    }else {
        SearchEngine::search(config.query, contents)
    };

    for line in results{
        println!("{line}")
    }

    Ok(())
}

pub struct Config{
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
    pub only_word: bool,
}


impl Config{
    pub fn build(args: &[String]) -> Result<Config, String> {

        if args.len() < 3 {
           return Err("not enough arguments".to_string());
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let mut ignore_case = false;
        let mut only_word = false; 
        
        if args.len() == 4{
            match args[3].as_str(){
                "-i" => {
                    ignore_case = true;
                }
                "-o" => {
                   only_word = true; 
                }         
                _ => {
                    return Err("use --help to show flags".to_string());
                }
            }
        }
        if args.len() > 4{
            return Err("too many arguments".to_string())
        }

        Ok(Config{query, file_path, ignore_case, only_word})
    }
}
