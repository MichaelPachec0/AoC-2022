use crate::structs;
use structs::folder::Folder;

pub struct Location<'a> {
    dir_stack: Vec<&'a Folder<'a>>,
    // could be done as a hashset and avoid the O(n) search
    // but we keep creating and recreating it, it might not be worthwhile, specially when n is small
    nested_folders: Vec<&'a Folder<'a>>,
    root_folders: Vec<&'a Folder<'a>>,
}

impl<'a> Location<'a> {
    pub fn new() -> Self {
        Self {
            dir_stack: vec![],
            nested_folders: vec![],
            root_folders: vec![],
        }
    }
    pub fn is_root(&self) -> bool {
        self.dir_stack.len() == 1
    }
    pub fn enter_directory(&mut self, dir: &'a Folder) {
        if self.is_root() {
            self.root_folders = std::mem::take(&mut self.nested_folders);
        }
        self.dir_stack.push(dir);
    }
    pub fn add_directory_multiple(&mut self, dir_list: &[&'a Folder]) {
        self.nested_folders = Vec::from(dir_list);
    }
    pub fn add_directory(&mut self, dir: &'a Folder) {
        self.nested_folders.push(dir);
    }
    pub fn pop_dir(&mut self) {
        if !self.is_root() {
            self.dir_stack.pop();
            self.nested_folders.clear();
            if self.is_root() {
                self.nested_folders = std::mem::take(&mut self.root_folders);
            }
        }
    }
    pub fn goto_root(&mut self) {
        if !self.is_root() {
            self.dir_stack.drain(1..self.dir_stack.len());
            self.nested_folders = std::mem::take(&mut self.root_folders);
        }
    }
    // need to decide whether i should make the calling method just send me the data, or have the
    // method send me a reference to the slice, iterate through it and add the directories myself
    pub fn return_directory(&mut self, dir: &'a Folder, dir_list: &[&'a Folder]) {
        self.dir_stack.push(dir);
        self.nested_folders = Vec::from(dir_list);
    }
    pub fn pwd(&self) -> String {
        self.dir_stack
            .iter()
            .map(|&dir| String::from(dir.name))
            .collect::<Vec<String>>()
            .join("/")
    }
    pub fn current_directory(&self) -> &&'a Folder {
        self.dir_stack.last().unwrap()
    }
    pub fn is_current(&self, folder: &'a Folder) -> bool {
        *self.current_directory() == folder
    }
    pub fn iter(&'a self) -> impl Iterator<Item = &'a &'a Folder> {
        self.nested_folders.iter()
    }
    pub fn change_folder(
        &mut self,
        name: &str,
    ) -> Result<(&Vec<usize>, &Vec<usize>), &'static str> {
        for &folder in &self.nested_folders {
            if folder.name_eq(name) {
                self.enter_directory(folder);
                return Ok(folder.file_folder_indices());
            }
        }
        Err("FOLDER NOT FOUND")
    }
}
