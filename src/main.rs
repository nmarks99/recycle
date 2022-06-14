// use std::fs;
// use filetime::FileTime;
use std::env;
use std::process::Command;
use std::path::Path;


//
// fn extra() {
    // println!("extra");
// }
//

fn sys_command(command:String) -> std::process::Output {

    // See if on windows
    let windows = cfg!(target_os = "windows");
    let prog:&str;
    let prog_flag:&str;

    if windows == true {
        prog = "cmd";
        prog_flag = "/C";
    }
    else{
        // Presumably the other option is a UNIX machine with "sh"
        // If you have git-bash on windows this works for windows too
        prog = "sh";
        prog_flag = "-c";
    }

    let output = Command::new(prog)
                .arg(prog_flag) // this is important but idk why
                .arg(command)
                .output()
                .expect("Command failed to execute");

    return output;
}

fn is_trash_dir(rec_path: &str) {
   if !Path::exists(Path::new(rec_path)) {
       let cmd = format!("mkdir {}",rec_path);
       sys_command(cmd);
   }
}


fn move_to_trash(filename: &str) {
     
    // Path to the recycle bin
    let rec_path: &str = "./trash";
    
    // Check that the given file exists
    if !Path::new(filename).exists() {
        panic!("{} does not exist",filename);
    }

    // Create the recycle bin if it doesn't exist
    is_trash_dir(rec_path);

    // Generate the mv command
    let full_command = format!("mv {} {}",filename, rec_path);

    // issue the shell command
    sys_command(full_command);
}



fn main() {
   
    let args: Vec<String> = env::args().collect();
    let ref filename = &args[1]; 
    match args.len() {
        1 => panic!("re: Missing operand"),
        2 => move_to_trash(filename),
        _ => println!("Too many arguments. Expected 1, recieved {}",args.len()),
    }
    
    // let ref filename = &args[1];
    // let metadata = fs::metadata("foo.txt").unwrap();
    // let unix_time = FileTime::from_last_modification_time(&metadata).unix_seconds();
}



