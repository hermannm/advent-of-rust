use std::slice::Iter;

use super::file_system::{Directory, FileSystemNode};

pub struct DirectorySizes {
    pub root_size: u64,
    sizes: Vec<u64>,
}

impl From<Directory> for DirectorySizes {
    fn from(directory: Directory) -> Self {
        let mut sizes = DirectorySizes {
            root_size: 0u64,
            sizes: Vec::<u64>::new(),
        };

        let root_size = sizes.add(directory);
        sizes.root_size = root_size;

        sizes
    }
}

impl DirectorySizes {
    fn add(&mut self, directory: Directory) -> u64 {
        let mut size = 0u64;

        for child in directory.children {
            size += match child {
                FileSystemNode::File(file) => file.size,
                FileSystemNode::Directory(sub_directory) => self.add(sub_directory),
            }
        }

        self.sizes.push(size);

        size
    }
}

impl<'a> IntoIterator for &'a DirectorySizes {
    type Item = &'a u64;
    type IntoIter = Iter<'a, u64>;

    fn into_iter(self) -> Self::IntoIter {
        self.sizes.iter()
    }
}
