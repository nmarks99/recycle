// use std::fs;
// use filetime::FileTime;
use std::env;

fn extra() {
    println!("extra");
}

fn single() {
   println!("single") 
}


fn main() {
   
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        1 => panic!("re: Missing operand"),
        2 => single(),
        3 => extra(),
        _ => println!("Not implemented"),
    }
    
    // let ref filename = &args[1];
    // let metadata = fs::metadata("foo.txt").unwrap();
    // let unix_time = FileTime::from_last_modification_time(&metadata).unix_seconds();
}
