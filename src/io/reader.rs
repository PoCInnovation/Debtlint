use std::fs;
use std::path::Path;

pub fn read_corpus(path: &Path) -> std::io::Result<String> {
    fs::read_to_string(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn reads_file_contents() {
        let dir = std::env::temp_dir();
        let path = dir.join("debtlint_corpus_test.txt");
        let mut file = std::fs::File::create(&path).unwrap();
        write!(file, "banana").unwrap();

        let content = read_corpus(&path).unwrap();
        assert_eq!(content, "banana");

        let _ = std::fs::remove_file(path);
    }
}
