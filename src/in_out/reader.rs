use std::fs; // for read the file (filesystem)
use std::path::Path; // for path to a file

pub fn read_corpus(path: &Path) -> std::io::Result<String> {
    // recup the ref of the path and return result string if ok or err
    fs::read_to_string(path) // read the file completly and put the content in a string
}
