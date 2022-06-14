// use std::fs;
// use filetime::FileTime;
use std::env;
use std::process::Command;

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


fn move_to_recycle(filename: &str) {
    let RECYCLE_BIN_PATH: &str = "./recycle_bin";
    let mut full_command: String = String::from("");
    full_command.push_str(&format!("mv {} {}",filename, RECYCLE_BIN_PATH)[..]);
    println!("{}",full_command); 
    // let output = sys_command(full_command);
}

fn main() {
   
    let args: Vec<String> = env::args().collect();
    let ref filename = &args[1]; 
    match args.len() {
        1 => panic!("re: Missing operand"),
        2 => move_to_recycle(filename),
        _ => println!("Not implemented"),
    }
    
    // let ref filename = &args[1];
    // let metadata = fs::metadata("foo.txt").unwrap();
    // let unix_time = FileTime::from_last_modification_time(&metadata).unix_seconds();
}
