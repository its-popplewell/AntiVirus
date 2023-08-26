// use std::process::Command;

use std::io::prelude::*;
use std::fs::File;

struct FileName {
    filename: String,
    fileextension: String,
    path: String,
}

// impl FileName {
//     fn new(fn_inp: String) -> FileName {
//         FileName {

//             filename,
//             fileextension,
//             path,
//         }
//     }
// }

fn get_file_bytes(filename: FileName) -> Vec<u8> {
    let mut file = File::open(format!("{}/{}.{}",filename.path, filename.filename, filename.fileextension)).expect("Couldn't Open File");
    let mut output = Vec::new();
    file.read_to_end(&mut output);
    output
}

// fn read_file_bytes(str::)


fn main() {
    println!("{:?}",get_file_bytes(FileName {
            filename: "boop".to_string(),
            fileextension: "txt".to_string(),
            path: "resources".to_string(),
        }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_file_bytes_runs() {
        let result = get_file_bytes(FileName {
            filename: "boop".to_string(),
            fileextension: "txt".to_string(),
            path: "resources".to_string(),
        });
        println!("{:?}", result);
        assert_eq!(result, result);
    }
}