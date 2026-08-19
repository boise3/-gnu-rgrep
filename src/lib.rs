
#[warn(unused_imports)]
use std::{error::Error, fs};

pub struct SearchEngine;

impl SearchEngine {
    
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

    pub fn search_count_o(query: String, content: String) -> Vec<String>{
        let mut result: Vec<String> = Vec::new();

        let mut numbers_of_words: u32 = 0;

        for line in content.lines(){
            if line.contains(&query){
                numbers_of_words += 1;
            }
        } 

        let numbers_of_words = numbers_of_words.to_string();

        result.push(numbers_of_words);

        result
    }

}


pub fn run (config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(&config.file_path)?;
    let _content = fs::read(&config.file_path)?; //binary format for future

    // if config.file_path.ends_with(".exe"){
    //     eprintln!("Binary file plik matches");
    //     process::exit(1);
    // }

    let results = if config.ignore_case_i {
        SearchEngine::search_case_insensitive(config.query, contents)
    }else if config.only_matching_o {
        SearchEngine::search_word(config.query, contents)
    }else if config.count_c {
        SearchEngine::search_count_o(config.query, contents)
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
    pub ignore_case_i: bool,
    pub only_matching_o: bool,
    pub count_c: bool,
}


impl Config{
    pub fn build(args: &[String]) -> Result<Config, String> {

        if args.len() < 3 {
           return Err("not enough arguments".to_string());
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let mut ignore_case_i = false;
        let mut only_matching_o = false; 
        let mut count_c: bool = false;
        
        if args.len() == 4{
            match args[3].as_str(){
                "-i" | "--ignore-case" => {
                    ignore_case_i = true;
                }
                "-c" | "--count" => {
                    count_c = true;
                }
                "-V" | "--version" => {
                    eprintln!("rgrep (macos compatible) 1.0.0-rgrep");
                }
                "-o" | "--only-matching"=> {
                   only_matching_o = true; 
                }         
                _ => {
                    return Err("use --help to show flags".to_string());
                }
            }
        }
        if args.len() > 4{
            return Err("too many arguments".to_string())
        }

        Ok(Config{query, file_path, ignore_case_i, only_matching_o, count_c})
    }
}

