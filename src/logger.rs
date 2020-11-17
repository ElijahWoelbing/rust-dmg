use std::fs::File;
use std::io::ErrorKind;
pub struct Logger {
    file: File
}


impl Logger {
    pub fn new(log_name: &str) -> Self {
        let f = File::open(log_name);
        let file=  match f   {
            Ok(file)=> file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound =>  match File::create(log_name){
                    Ok(file) => file,
                    Err(_) => panic!("unable to create file")
                },
                other_error=> {
                    panic!("problem opening the file {:?}", other_error)
                }
            }
        };
        Self {
            file
        }
    }
}