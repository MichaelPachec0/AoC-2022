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
#[cfg(test)]
mod tests;
mod structs;
use structs::{file::File, folder::Folder, location::Location};

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
    Folder(&'a str),
}

struct Command<'a> {
    raw_string: &'a str,
    // ls, cd, ect
    command_type: &'a str,
    location: &'a str,
    arguments: Vec<&'a str>,
}

impl<'a> Command<'a> {
    fn new(
        raw_string: &'a str,
        command_type: &'a str,
        location: &'a str,
        arguments: Vec<&'a str>,
    ) -> Self {
        Self {
            raw_string,
            command_type,
            location,
            arguments,
        }
    }
}

struct DirectoryBrowser<'a> {
    raw_string: &'a str,
    // This will have directories minus the root
    directories_matching: Vec<usize>,
    // Where we will be in current directory
    location: Location<'a>,
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
            command_stream: vec![],
            root_size: 0,
            root_folder: Folder::new("", 0),
            files: vec![],
            location: Location::new(),
        }
    }
    pub fn parse(&mut self) {
        for line in self.raw_string.split('\n') {
            match line.chars().next() {
                Some('$') => {
                    let mut command_chunked = line.split(' ').skip(0);
                    let command = command_chunked.next().unwrap_or("");
                    let arguments = command_chunked.collect::<Vec<&str>>();
                    self.command_stream.push(CommandLine::Command(Command::new(
                        line, command, "", arguments,
                    )));
                }
                Some(_) => {
                    // file list
                    let mut file_chunks = line.split(' ');
                    if let Some(str) = file_chunks.next() {
                        match str.parse::<usize>() {
                            Ok(size) => {
                                // File
                                self.command_stream.push(CommandLine::File(File::new(
                                    file_chunks.next().unwrap_or_default(),
                                    size,
                                    self.command_stream.len(),
                                )));
                            }
                            Err(_) => {
                                // Folder
                                self.command_stream.push(CommandLine::Folder(Folder::new(
                                    file_chunks.next().unwrap_or_default(),
                                    self.command_stream.len(),
                                )));
                            }
                        }
                    };
                }
                None => {}
            }
        }
    }
    fn command_helper(command: Command, location: &'a mut Location<'a>) {
        let mut arg_iter = command.arguments.iter();
        match command.command_type {
            "cd" => {
                match *arg_iter.next().unwrap_or(&" ") {
                    ".." => {
                        // pop the last directory
                        if !location.is_root() {
                            location.pop_dir();
                            println!("Hello World");
                        }
                    }
                    "/" => {
                        // pop all directories, we are at the root directory
                        location.goto_root();
                    }
                    " " => {
                        //Bad input, let print it out
                        eprintln!(
                            "BAD INPUT, WITH COMMAND {} and with args {:?}",
                            command.command_type, command.arguments
                        );
                    }
                    _ => {
                        if let Some(&folder) = arg_iter.next() {
                            // let iter = location.nested_folders.clone();
                            // for nested_folder in iter  {
                            //     if nested_folder.name_eq(folder){
                            //         location.enter_directory(nested_folder);
                            //     }
                        }
                    } // if Some(folder) =
                      //     // we are entering a directory
                      //     location.push();
                }
            }

            "ls" => {
                //ignore ?
            }
            _ => {
                println!(
                    "UNKNOWN COMMAND {} WITH ARGS {}",
                    command.command_type,
                    command.arguments.join(" ")
                )
            }
        }
    }
}
