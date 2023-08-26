// use std::process::Command;

use std::io::prelude::*;
use std::fs::File;

struct FileName {
    path: String,
    filename: String,
    fileextension: String,
}

impl FileName {
    // fn new(fn_inp: String) -> FileName {
    //     FileName {

    //         filename,
    //         fileextension,
    //         path,
    //     }
    // }
}

// Implement `Display` for `FileName`.
impl std::fmt::Display for FileName {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}/{}.{}", self.path, self.filename, self.fileextension)
    }
}

fn get_file_bytes(filename: FileName) -> Vec<u8> {
    let mut file = File::open(format!("{}/{}.{}",filename.path, filename.filename, filename.fileextension)).expect("Couldn't Open File");
    let mut output = Vec::new();
    file.read_to_end(&mut output);
    output
}

// fn read_file_bytes(str::)


fn main() {
    let filename: FileName = FileName {
        filename: "boop".to_string(),
        fileextension: "txt".to_string(),
        path: "resources".to_string(),
    };

    println!("{}", filename);

    println!("{:?}",get_file_bytes(FileName {
            filename: "boop".to_string(),
            fileextension: "txt".to_string(),
            path: "resources".to_string(),
        }));
}

#[cfg(test)]
mod tests {
    use super::*;

    const filename: FileName = FileName {
        filename: "boop".to_string(),
        fileextension: "txt".to_string(),
        path: "resources".to_string(),
    };

    #[test]
    fn get_file_bytes_runs() {
        let result = get_file_bytes(filename);
        println!("{:?}", result);
        assert_eq!(result, result);
    }
}