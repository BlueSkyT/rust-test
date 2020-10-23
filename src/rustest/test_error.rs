use std::fs::File;

pub fn test_error_handle() {
    let f = File::open("text.text");
    let _f = match f {
        Ok(file) => file,
        Err(error) => panic!("problem opening the file :[{:?}]", error),
    };
}