use crate::tests::common::reader;
use super::*;

#[test]
fn sample() -> Result<(), Box<dyn std::error::Error>> {
    let test = reader("../sample.txt")?;

    Ok(())
}
#[test]
fn test_dir_stack() {
    let input = "$ cd /\n$ ls\ndir a\ndir b\n$ cd a\n".to_string();
}

