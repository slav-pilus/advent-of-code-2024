use std::fs::File;

pub(crate) fn get_file(filename: &str) -> File {
    let file = match File::open(filename) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            std::process::exit(1);
        }
    };
    file
}
