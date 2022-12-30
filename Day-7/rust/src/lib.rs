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

use std::borrow::BorrowMut;

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

enum CommandLine<'a> {
    File(File<'a>),
    Folder(Folder<'a>),
    Command(Command<'a>),
}

enum Type<'a> {
    File(usize),
    Folder(&'a str)
}

struct Command<'a> {
    raw_string: &'a str,
    // ls, cd, ect
    command_type: &'a str,
    location: &'a str,
    arguments: Vec<&'a str>,
}

impl<'a> Command<'a> {
    fn new(raw_string: &'a str, command_type: &'a str, location: &'a str, arguments: Vec<&'a str>) -> Self {
        Self {
            raw_string,
            command_type,
            location,
            arguments,
        }
    }
}


struct File<'a> {
    filename: &'a str,
    size: usize,
}

impl<'a> File<'a> {
    pub fn new(filename: &'a str, size: &'a str) -> Self {
        let size = size.parse::<usize>().unwrap();
        Self { filename, size }
    }
}

struct Folder<'a> {
    name: &'a str,
    size: Option<usize>,
    start: usize,
    stop: usize,
}

impl<'a> Folder<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            size: None,
            start: 0,
            stop: 0,
        }
    }
    // pub fn compute_size(&mut self, level: Option<usize>, commands: &mut [CommandLine<'_>]) -> (Option<usize>, &[(&'a str, usize)]) {
        // let tab = " ".repeat(SPACE_TAB);
        // let level = level.unwrap_or(0);
        // let file_len = self.nested_files.len();
        // let tabs = tab.repeat(level);
        // for (i, &file_ref) in self.nested_files.iter().enumerate() {
        //     match commands[file_ref] {
        //         CommandLine::File(file) => {
        //             let (name, size) = (file.filename, file.size);
        //             println!("{tabs}{tab}LEVEL: {level}: FILE {i} of {file_len} FILE {name} WITH SIZE OF {size}");
        //             self.size = Some(self.size.unwrap_or(0) + size)
        //         }
        //         CommandLine::Folder(ref mut folder) => {
        //             let name = folder.name;
        //             let (fsize, folders) = folder.compute_size(Some(level+1), commands);
        //             println!("{tabs}LEVEL: {level} FOLDER {i} of {file_len} FOLDER NAME {name}");
        //             self.size = Some(self.size.unwrap_or(0) + fsize.unwrap_or(0));
        //             for &sub_folder in folders.iter() {
        //                 self.folders_matching.push(sub_folder);
        //             }
        //         }
        //         _ => {}
        //     }
        // }
        // (self.size, &self.folders_matching)
        // for (i, file) in self.file_list.iter().enumerate() {
        //     let (filename, fsize) = match &commands[*file] {
        //         CommandLine::File(file) => (file.filename, file.size),
        //         _ => {("", 0)}
        //     };
        //     println!(
        //         "{tabs}{tab}LEVEL: {level} FILE {i} of {file_len} FILE {filename} WITH SIZE OF {fsize}"
        //     );
        //     self.size = Some(self.size.unwrap_or(0) + fsize);
        // }
        // for (i, folder) in self.folder_list.iter_mut().enumerate() {
        //     println!("{tabs}LEVEL: {level} FOLDER {i} of {folder_len} OPENING FOLDER {} INSIDE OF FOLDER {}", folder.name, self.name);
        //     let (folder_size, folders) = folder.compute_size(Some(level + 1));
        //     self.size = Some(self.size.unwrap_or(0) + folder_size.unwrap_or(0));
        //     for &subfolder in folders.iter() {
        //         self.folders_matching.push(subfolder);
        //     }
        //     // folders.iter().for_each(|&folder| self.folders_matching.push(folder));
        // }
        // (self.size, &self.folders_matching)
    // }
}

struct DirectoryBrowser<'a> {
    raw_string: &'a str,
    // This will have directories minus the root
    directories_matching: Vec<usize>,
    // Where we will be in current directory
    dir_stack: Vec<&'a Folder<'a>>,
    // Parseable stream of commands (and their outputs)
    command_stream: Vec<CommandLine<'a>>,
    root_size: usize,
    // TODO: need to think about if root folder should own the struct of if we can get away with a
    //  ref (or mut ref) and have command stream own everything
    root_folder: Folder<'a>,
    files: Vec<&'a File<'a>>,
}

impl<'a> DirectoryBrowser<'a> {
    pub fn new(raw_string: &'a str) -> Self {
        Self {
            raw_string,
            directories_matching: vec![],
            dir_stack: vec![],
            command_stream: vec![],
            root_size: 0,
            root_folder: Folder {
                name: "",
                size: None,
                start: 0,
                stop: 0,
            },
            files: vec![],
        }
    }
    pub fn parse(&mut self) {
        for line in self.raw_string.split('\n') {
            match line.chars().next() {
                Some('$') => {
                    let mut command_chunked = line.split(' ').skip(0);
                    let command = command_chunked.next().unwrap_or("");
                    let arguments = command_chunked.collect::<Vec<&str>>();
                    self.command_stream.push(
                        CommandLine::Command(
                            Command::new(line, command, "", arguments)
                        )
                    );
                }
                Some(_) => {
                    // file list
                    let mut file_chunks = line.split(' ');
                    let file = match file_chunks.next() {
                        Some(str) => {
                            match str.parse::<usize>() {
                                Ok(size) => {
                                    (Type::File(size), file_chunks.next().unwrap_or_default())
                                }
                                Err(_) => {
                                    (Type::Folder(str), file_chunks.next().unwrap_or_default())
                                }
                            }
                        }
                        _ => {
                            // WARN: We should never end up here, this is to make rust happy.
                            (Type::Folder(""), "")
                        }
                    };
                }
                None => {}
            }
        }
    }
    pub fn compute(&mut self) {
        // for (i0, command) in self.command_stream.iter_mut().enumerate() {
        //     match command {
        //         CommandLine::Folder(folder) => {
        //             for (i1, &file) in folder.nested_files.iter().enumerate() {
        //
        //             }
        //         }
        //         // TODO: See if its possible to disable mutability in these last two references
        //         // This does not need to mutable, we arent going to do anything with the struct
        //         CommandLine::File(file) => {
        //
        //         }
        //         // This also does not need to mutable, but we mutability by default
        //         CommandLine::Command(command) => {}
        //     }
        // }
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
