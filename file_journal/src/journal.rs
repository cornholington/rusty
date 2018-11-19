use std::error::Error;
use std::fs::*;
use std::path::Path;

pub struct Journal {
    index_writ: File,
    index_read: File,
    blob_writ: File,
    blob_read: File,
}

impl Journal {
    pub fn open(dir: &Path) -> Result<Self, Box<Error>> {
        create_dir_all(dir)?;

        let index_write = +

        Ok(Self {
            index_write,
            index_read,
            blob_write,
            blob_read,
        })
    }
}
#[cfg(test)]
mod test {

    #[test]
    fn test_open_create() {
        assert!(true);
    }
}
