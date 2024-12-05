use std::{fs::File, io::{BufRead, BufReader}, process::exit};



fn main() {
    //file opening
    let path="input.txt";
    let file = match File::open(path) {
        Ok(file) =>file,
        Err(e)=>{print!("unable to open {} file, err: {}",path,e);
        exit(1)},     
    };
    //file reading
    let buffer=BufReader::new(file);
    let mut list1:Vec<i32>=Vec::new();
    let mut list2:Vec<i32>=Vec::new();
    let lines = buffer.lines();
    //splitting to 2 diffrent list
    for line in lines{
        let line = line.unwrap();
        let values:Vec<&str> =line.split("   ").collect();
        list1.push(values[0].parse().unwrap());
        list2.push(values[1].parse().unwrap());        
    }
    //sorting list
    list1.sort();
    list2.sort();

    let mut distances:Vec<i32> = Vec::new(); 
    for idx in 0..list1.len(){
        distances.push((list1[idx]-list2[idx]).abs())
    }

    let sum:i32 = distances.into_iter().sum();
    println!("sum of distances: {}",sum)
}
