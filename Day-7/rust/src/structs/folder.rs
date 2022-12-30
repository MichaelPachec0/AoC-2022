pub struct Folder<'a> {
    pub name: &'a str,
    size: Option<usize>,
    folder_list: Vec<usize>,
    file_list: Vec<usize>,
    location: usize,
}

impl<'a> Folder<'a> {
    pub fn new(name: &'a str, location: usize) -> Self {
        Self {
            name,
            size: None,
            folder_list: vec![],
            file_list: vec![],
            location,
        }
    }
    pub fn name_eq(&self, name: &str) -> bool {
        self.name == name
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
    pub fn file_folder_indices(&self) -> (&Vec<usize>, &Vec<usize>) {
        (&self.file_list, &self.folder_list)
    }
}
impl<'a> PartialEq for Folder<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location
    }
}
