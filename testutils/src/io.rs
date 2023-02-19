use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Lines;
use std::io::Result;
use std::io::Write;
use std::path::Path;
use tempfile::NamedTempFile;

pub fn read_lines(path: &Path) -> Result<Lines<BufReader<File>>> {
    let file = File::open(&path)?;
    Ok(BufReader::new(file).lines())
}

pub fn tempfile(content: &str) -> Result<NamedTempFile> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "{}", content)?;
    Ok(file)
}

pub fn persist<'a>(content: &'a str, path: &'a Path) -> Result<&'a Path> {
    let file = NamedTempFile::new()?;
    let mut persisted = file.persist(&path)?;
    writeln!(persisted, "{}", content)?;
    Ok(&path)
}
