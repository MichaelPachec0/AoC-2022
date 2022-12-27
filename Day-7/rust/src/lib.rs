// NOTES
// The stream of terminal output is the puzzle input.
// $ denotes commands ie $ ls.
//
// the output of ls will have as follows.
// dir donotes directories following their filename.
// files will have the number of bytes followed by the filename.
//
// the output of cd will be nothing, but will change the location.
// .. should move the current directory to the parent folder.
// a or any other letter will then open a folder that is in current directory.
//
// Root directory will Always be /

// fn stack(stream: &str) -> &str {
//     stream.split('\n')
//         .filter(|&line| line.starts_with('$'))
//         .map(|line| {
//             let tmp  = line.split(' ')
//                 .collect::<Vec<&str>>();
//             if tmp[1] == "cd" {
//                 match tmp { }
//             }
//         })
// }

struct File<'a> {
    filename: &'a str,
    size: usize,
}

impl<'a> File<'a> {
    pub fn new(filename: &'a str, size: &'a str) -> Self {
        let size = size.parse::<usize>().unwrap();
        Self {
            filename,
            size
        }
    }
}


struct Folder<'a> {
    name: &'a str,
    size: Option<usize>,
    file_list: Vec<File<'a>>,
}

impl<'a> Folder<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            size: None,
            file_list: Vec::new(),
        }
    }
}

struct DirectoryBrowser<'a>{
    raw_string: &'a str,
    // This will have directories minus the root
    directories: Vec<Folder<'a>>,
    dir_stack: Vec<&'a Folder<'a>>,
}

impl<'a> DirectoryBrowser<'a> {
    pub fn new(raw_string: &'a str) -> Self {
        Self {
            raw_string,
            directories: Vec::new(),
            dir_stack: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn reader(path: &str) -> Result<String, Box<dyn std::error::Error>> {
        Ok(fs::read_to_string(path)?.parse()?)
    }


    #[test]
    fn test_dir_stack() {
        let input = "$ cd /\n$ ls\ndir a\ndir b\n$ cd a\n".to_string();

    }

    #[test]
    fn sample() -> Result<(), Box<dyn std::error::Error>> {
        let test = reader("../sample.txt")?;

        Ok(())
    }
}
