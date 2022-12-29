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
const SPACE_TAB: usize = 4;



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
    folders_matching: Vec<(&'a str, usize)>,
    folder_list: Vec<Folder<'a>>
}

impl<'a> Folder<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            size: None,
            file_list: vec![],
            folders_matching: vec![],
            folder_list: vec![],
        }
    }
    pub fn compute_size(&mut self, level: Option<usize>)-> (Option<usize>, &[(&'a str, usize)]) {
        let tab = " ".repeat(SPACE_TAB);
        let level = level.unwrap_or(0);
        let folder_len = self.folder_list.len();
        let file_len = self.file_list.len();
        // let tabs = (0..level).map(|_| "\t").collect::<Vec<&str>>().join("");
        // let tabs = "\t".repeat(level);
        let tabs = tab.repeat(level);
        for (i, file) in self.file_list.iter().enumerate(){
            println!("{tabs}{tab}LEVEL: {level} FILE {i} of {file_len} FILE {} WITH SIZE OF {}", file.filename, file.size);
            self.size = Some(self.size.unwrap_or(0) + file.size);
        }
        for (i, folder) in self.folder_list.iter_mut().enumerate() {
            println!("{tabs}LEVEL: {level} FOLDER {i} of {folder_len} OPENING FOLDER {} INSIDE OF FOLDER {}", folder.name, self.name);
            let (folder_size, folders) = folder.compute_size(Some(level+1));
            self.size = Some(self.size.unwrap_or(0) + folder_size.unwrap_or(0));
            for &subfolder in folders.iter() {
                self.folders_matching.push(subfolder)
            }
            // folders.iter().for_each(|&folder| self.folders_matching.push(folder));
        }
        (self.size, &self.folders_matching)
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
