use std::fs::File;
use std::io::{Write, Read};
use std::io;

//save string to file (returning success)
pub fn save_to_file(path: &str, content: &str)->Result<(),io::Error> {
    let mut file = File::create(path)?;
    file.write(content.as_bytes())?;
    Ok(())
}

//load file to string. returning result (can fail)
pub fn load_from_file(path: &str) -> Result<String,io::Error> {

    let mut file = File::open(path)?;
    let mut buffer = String::new();
    let reading_result=file.read_to_string(&mut buffer)?;
    Ok(buffer)
}
