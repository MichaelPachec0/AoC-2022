pub struct File<'a> {
    filename: &'a str,
    size: usize,
    location: usize,
}

impl<'a> File<'a> {
    pub fn new(filename: &'a str, size: usize, location: usize) -> Self {
        Self {
            filename,
            size,
            location,
        }
    }
}

impl<'a> PartialEq for File<'a>  {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location
    }
}