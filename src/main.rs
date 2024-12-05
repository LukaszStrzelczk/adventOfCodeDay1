use std::{fs::File, io::{BufRead, BufReader}, process::exit};


fn main() {
    //file opening
    let path=".txt";
    let file = match File::open(path) {
        Ok(file) =>file,
        Err(e)=>{print!("unable to open {} file, err: {}",path,e);
        exit(1)},     
    };
    //file reading
    let buffer=BufReader::new(file);
    let mut list1:Vec<String>=Vec::new();
    let mut list2:Vec<String>=Vec::new();
    let lines = buffer.lines();
    //splitting to 2 diffrent list
    for line in lines{
        let line = line.unwrap();
        let values:Vec<&str> =line.split("   ").collect();
        list1.push(String::from(values[0]));
        list2.push(String::from(values[1]));        
    }

    println!("list 1: {list1:?}");
    println!("list 2: {list2:?}");

}
