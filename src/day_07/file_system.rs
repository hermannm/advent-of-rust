pub(super) type Path = Vec<String>;

pub(super) enum FileSystemNode {
    Directory(Directory),
    File(File),
}

pub(super) struct Directory {
    pub name: String,
    pub children: Vec<FileSystemNode>,
}

impl Directory {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
            children: Vec::<FileSystemNode>::new(),
        }
    }

    pub fn traverse(&mut self, path: &Path) -> Result<&mut Directory, String> {
        if path.is_empty() {
            return Ok(self);
        }

        let mut path = path.clone();
        let path_first = path.remove(0);

        self.children
            .iter_mut()
            .flat_map(|child| match child {
                FileSystemNode::Directory(directory) => Some(directory),
                _ => None,
            })
            .find(|directory| directory.name == path_first)
            .ok_or_else(|| {
                format!(
                    "Directory '{}' had no sub-directory '{}'",
                    self.name, path_first
                )
            })?
            .traverse(&path)
    }
}

pub(super) struct File {
    pub size: u64,
}

impl File {
    pub fn new(size: u64) -> Self {
        Self { size }
    }
}
