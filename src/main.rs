use std::fs;
use filetime::FileTime;


fn main() {
    
    let metadata = fs::metadata("foo.txt").unwrap();

    let mtime = FileTime::from_last_modification_time(&metadata);

    println!("{}",mtime.unix_seconds());

}
