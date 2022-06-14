use std::process::Command;

pub fn sys_command(command:String) -> std::process::Output {

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

