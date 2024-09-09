use std::path;

trait FileMetadata {
    fn exists(&self) -> bool;

    fn is_writeable(&self) -> bool;

    fn is_readable(&self) -> bool;
}

impl FileMetadata for path::Path {
    fn is_readable(&self) -> bool {
        self.exists() && std::fs::metadata(self).map_or(false, |meta| meta.permissions().readonly())
    }

    fn is_writeable(&self) -> bool {
        self.exists() && std::fs::metadata(self).map_or(false, |meta| meta.permissions().readonly()) == false
    }

    fn exists(&self) -> bool {
        self.exists()
    }
}

fn main() {
    let path = path::Path::new("example.txt");

    if path.exists() {
        println!("File exists");
        if path.is_readable() {
            println!("File is readable");
        } else {
            println!("File is not readable");
        }
        if path.is_writeable() {
            println!("File is writable");
        } else {
            println!("File is not writable");
        }
    } else {
        println!("File does not exist");
    }
}

#[test]
fn writeable() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    assert!(f.path().is_writeable());

    fs::remove_file(f.path()).unwrap();
}

#[test]
fn read_only() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    let mut perms = fs::metadata(f.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(f.path(), perms).unwrap();
    assert_eq!(f.path().is_writeable(), false);

    fs::remove_file(f.path()).unwrap();
}