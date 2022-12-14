use super::file_system::{Directory, File, FileSystemNode, Path};

pub(super) enum InputLine<'a> {
    ListCommand,
    ChangeDirectoryCommand(ChangeDirectoryTarget<'a>),
    DirectoryName(&'a str),
    File(u64),
}

impl<'a> InputLine<'a> {
    pub fn parse_lines(input: &'a str) -> Result<Vec<Self>, String> {
        input
            .lines()
            .map(InputLine::try_from)
            .collect::<Result<Vec<InputLine>, String>>()
    }
}

impl<'a> TryFrom<&'a str> for InputLine<'a> {
    type Error = String;

    fn try_from(input_line: &'a str) -> Result<Self, Self::Error> {
        let (prefix, rest_of_line) = input_line
            .split_once(' ')
            .ok_or_else(|| String::from("Unable to parse input line without space"))?;

        match prefix {
            "$" => {
                if rest_of_line == "ls" {
                    return Ok(InputLine::ListCommand);
                }

                let (command, target_str) = rest_of_line.split_once(' ').ok_or_else(|| {
                    String::from("Change directory command did not contain at least 1 space")
                })?;

                if command != "cd" {
                    return Err(format!("Unknown command found: {command}"));
                }

                let target = ChangeDirectoryTarget::from(target_str);

                Ok(InputLine::ChangeDirectoryCommand(target))
            }
            "dir" => Ok(InputLine::DirectoryName(rest_of_line)),
            file_size_str => {
                let file_size = file_size_str
                    .parse::<u64>()
                    .map_err(|_| String::from("Could not parse file size from file input line"))?;

                Ok(InputLine::File(file_size))
            }
        }
    }
}

pub(super) enum ChangeDirectoryTarget<'a> {
    Root,
    Parent,
    Target(&'a str),
}

impl<'a> From<&'a str> for ChangeDirectoryTarget<'a> {
    fn from(target_str: &'a str) -> Self {
        use ChangeDirectoryTarget::*;

        match target_str {
            "/" => Root,
            ".." => Parent,
            target => Target(target),
        }
    }
}

impl<'a> TryFrom<Vec<InputLine<'a>>> for Directory {
    type Error = String;

    fn try_from(input_lines: Vec<InputLine>) -> Result<Self, Self::Error> {
        let mut root_directory = Directory::new("/");
        let mut current_path = Path::new();

        for line in input_lines.iter().skip(1) {
            match line {
                InputLine::ListCommand => continue,
                InputLine::ChangeDirectoryCommand(target) => match target {
                    ChangeDirectoryTarget::Root => current_path.clear(),
                    ChangeDirectoryTarget::Parent => {
                        current_path.pop().ok_or_else(|| {
                            String::from("Attempted to navigate to parent from root directory")
                        })?;
                    }
                    ChangeDirectoryTarget::Target(directory_name) => {
                        current_path.push(String::from(*directory_name));
                    }
                },
                InputLine::DirectoryName(name) => {
                    let current_directory = root_directory.traverse(&current_path)?;
                    let sub_directory = Directory::new(name);
                    current_directory
                        .children
                        .push(FileSystemNode::Directory(sub_directory));
                }
                InputLine::File(file_size) => {
                    let current_directory = root_directory.traverse(&current_path)?;
                    let file = File::new(*file_size);
                    current_directory.children.push(FileSystemNode::File(file));
                }
            }
        }

        Ok(root_directory)
    }
}
