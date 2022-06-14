// use std::fs;
// use filetime::FileTime;
use std::env;
use std::path::Path;
mod helpers;
use helpers::sys_command;

fn move_to_trash(filename: &str, rec_path: &str) {
    
    // Check that the given file exists
    assert!(Path::new(filename).exists(),"{} does not exist",filename);

    // Generate the mv command
    let full_command = format!("mv {} {}",filename, rec_path);

    // issue the shell command
    sys_command(full_command);
}

// fn delete_old(filename: &str) {
    //
// }

fn main() {
    
    // Path to the recycle bin
    let rec_path: &str = "~/.trash/";
    
    // Create the recycle bin if it doesn't exist
    if !Path::exists(Path::new(rec_path)) {
       let cmd = format!("mkdir {}",rec_path);
       sys_command(cmd);
    } 
    
    // Get command line arguments
    let args: Vec<String> = env::args().collect();
    assert!(args.len() >= 2, "Not enough input arguments");

    // Move requested files to the recycle bin 
    for i in 1..args.len() {
        move_to_trash(&args[i][..],rec_path);
    }
    
    
    // let ref filename = &args[1];
    // let metadata = fs::metadata("foo.txt").unwrap();
    // let unix_time = FileTime::from_last_modification_time(&metadata).unix_seconds();
}



