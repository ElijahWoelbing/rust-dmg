use std::fs::File;
use std::io::{ErrorKind, Write};
pub struct Logger {
    file: File,
    log_length: u64,
    log_count: u64,
}

impl Logger {
    pub fn new(log_length: u64) -> Self {
        let log_path = "./logs/my-log.text";
        let f = File::open(log_path);
        let file = match f {
            Ok(file) => file,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => match File::create(log_path) {
                    Ok(file) => file,
                    Err(_) => panic!("unable to create file"),
                },
                other_error => panic!("problem opening the file {:?}", other_error),
            },
        };
        Self {
            file,
            log_length,
            log_count: 0,
        }
    }

    pub fn log(&mut self, data: Vec<u16>) {
        if self.log_count < self.log_length {
            for num in data {
                let hex = format!("{:#x}", num);
                match self.file.write(hex.as_bytes()) {
                    Ok(_) => (),
                    Err(error) => panic!("error writing to file {}", error),
                };
                match self.file.write(b"\n") {
                    Ok(_) => (),
                    Err(error) => panic!("error writing to file {}", error),
                }
            }
            match self.file.write(b"\n") {
                Ok(_) => (),
                Err(error) => panic!("error writing to file {}", error),
            }
        } else {
            panic!("log count reached")
        }
        self.log_count += 1;
    }
}
