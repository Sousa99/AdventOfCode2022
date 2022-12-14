use std::collections::{HashMap, HashSet};

// =========================================== TYPE AND STRUCT DEFINITIONS ===========================================

type DirectoryObjectSize = usize;

enum CommandFunction {
    ChangeDirectory,
    ListDirectory
}

struct Command {

    function: CommandFunction,
    arguments: Vec<String>,
    outputs: Vec<String>
}

struct File {

    path: String,
    size: DirectoryObjectSize,
    _parent_path: String
}

struct Folder {

    path: String,
    _parent_path: String,
    child_paths: Vec<String>
}

trait DirectoryObject {

    fn is_file(&self) -> bool;
    fn is_folder(&self) -> bool;

    fn get_path(&self) -> String;

    fn set_child_paths(&mut self, paths: Vec<String>);

    fn get_directory_object_size(&self, directory: &HashMap<String, Box<dyn DirectoryObject>>) -> DirectoryObjectSize;
}

pub struct FileSystem {

    _commands: Vec<Command>,
    directory: HashMap<String, Box<dyn DirectoryObject>>

}

// =============================================== AUXILIARY FUNCTIONS ===============================================




// ================================================= IMPLEMENTATIONS =================================================

impl DirectoryObject for File {

    fn is_file(&self) -> bool { true }
    fn is_folder(&self) -> bool { false }

    fn get_path(&self) -> String { self.path.clone() }
    fn set_child_paths(&mut self, _: Vec<String>) { panic!("🚨 Child paths cannot be added to a file!") }

    fn get_directory_object_size(&self, _directory: &HashMap<String, Box<dyn DirectoryObject>>) -> DirectoryObjectSize {
        return self.size;
    }
}

impl DirectoryObject for Folder {

    fn is_file(&self) -> bool { false }
    fn is_folder(&self) -> bool { true }

    fn get_path(&self) -> String { self.path.clone() }
    fn set_child_paths(&mut self, paths: Vec<String>) { self.child_paths = paths }

    fn get_directory_object_size(&self, directory: &HashMap<String, Box<dyn DirectoryObject>>) -> DirectoryObjectSize {
        return self.child_paths.iter()
            .map(|child_path| directory.get(child_path).unwrap())
            .map(|directory_object| directory_object.get_directory_object_size(directory))
            .sum();
    }
}

impl FileSystem {

    pub fn new(lines: &Vec<String>) -> FileSystem {

        let commands: Vec<Command> = FileSystem::develop_commands(lines);
        let directory: HashMap<String, Box<dyn DirectoryObject>> = FileSystem::develop_directory(&commands);

        return FileSystem { _commands: commands, directory };
    }

    fn develop_commands(lines: &Vec<String>) ->  Vec<Command> {

        let mut commands: Vec<Command> = Vec::new();
        let mut current_command: Option<Command> = None;

        for line in lines.iter() {

            let new_command: bool = line.starts_with('$');
            if new_command {

                if current_command.is_some() { commands.push(current_command.unwrap()) }
                let mut command_split: Vec<String> = line.split_whitespace()
                    .map(|split| split.to_owned())
                    .collect();

                command_split.remove(0);
                let command_function: CommandFunction = match command_split.remove(0).as_str() {
                    "cd" => CommandFunction::ChangeDirectory,
                    "ls" => CommandFunction::ListDirectory,
                    argument => panic!("🚨 Command function argument '{}' not supported!", argument)
                };

                current_command = Some(Command {
                    function: command_function,
                    arguments: command_split,
                    outputs: Vec::new()
                });

            } else {

                match current_command.as_mut() {
                    Some(current_command_some) => {
                        current_command_some.outputs.push(line.to_string());
                    },
                    None => (),
                }
            }
        }

        if current_command.is_some() { commands.push(current_command.unwrap()) }
        return commands;

    }

    fn develop_directory(commands: &Vec<Command>) -> HashMap<String, Box<dyn DirectoryObject>> {

        let mut current_path: Vec<String> = Vec::new();
        let mut directory: HashMap<String, Box<dyn DirectoryObject>> = HashMap::new();

        directory.insert(current_path.join("/"), Box::new(Folder {
            path: current_path.join("/"),
            _parent_path: "".to_owned(),
            child_paths: Vec::new()
        }));

        for command in commands.iter() {

            match command.function {
                CommandFunction::ChangeDirectory => {

                    match command.arguments.get(0).unwrap().as_str() {
                        "/" => { current_path = Vec::new(); },
                        ".." => { current_path.pop(); },
                        directory_sub_folder => { current_path.push(directory_sub_folder.to_owned()); }
                    }
                },

                CommandFunction::ListDirectory => {

                    let mut new_directory_objects: Vec<Box<dyn DirectoryObject>> = Vec::new();
                    for directory_info in command.outputs.iter() {

                            let directory_info_split: Vec<String> = directory_info.split_whitespace()
                                .map(|info_split| info_split.to_owned())
                                .collect();

                            if directory_info_split.get(0).unwrap() == "dir" {

                                let directory_name: String = directory_info_split.get(1).unwrap().to_string();
                                
                                let mut new_path: Vec<String> = current_path.clone();
                                new_path.push(directory_name);

                                
                                new_directory_objects.push(Box::new(Folder {
                                    path: new_path.join("/"),
                                    _parent_path: current_path.join("/"),
                                    child_paths: Vec::new()
                                }));

                            } else {

                                let file_size: DirectoryObjectSize = directory_info_split.get(0).unwrap().parse().unwrap();
                                let file_name: String = directory_info_split.get(1).unwrap().to_owned();

                                let mut new_path: Vec<String> = current_path.clone();
                                new_path.push(file_name);

                                new_directory_objects.push(Box::new(File {
                                    path: new_path.join("/"),
                                    size: file_size,
                                    _parent_path: current_path.join("/"),
                                }));
                            }
                    }

                    let current_directory: &mut Box<dyn DirectoryObject> = directory.get_mut(&current_path.join("/")).unwrap();
                    current_directory.set_child_paths(new_directory_objects.iter()
                        .map(|directory_object| directory_object.get_path())
                        .collect());

                    directory.extend(new_directory_objects.into_iter()
                        .map(|directory_object| (directory_object.get_path(), directory_object)));
                }
            }
        }

        return directory;
    }

    pub fn directories_under_threshold(&self, threshold: DirectoryObjectSize) -> HashSet<(&String, DirectoryObjectSize)> {

        return self.directory.iter()
            .filter(|(_, object)| object.is_folder())
            .map(|(path, object)| (path, object.get_directory_object_size(&self.directory)))
            .filter(|(_, size)| *size <= threshold)
            .collect();
    }

    pub fn get_directory_to_delete_for_update(&self, total_disk_space: DirectoryObjectSize, update_size: DirectoryObjectSize) -> (&String, DirectoryObjectSize) {

        let space_used: DirectoryObjectSize = self.directory.get("").unwrap().get_directory_object_size(&self.directory);
        let space_to_free: DirectoryObjectSize = update_size - (total_disk_space - space_used);

        return self.directory.iter()
            .filter(|(_, object)| object.is_folder())
            .map(|(path, object)| (path, object.get_directory_object_size(&self.directory)))
            .filter(|(_, size)| *size >= space_to_free)
            .min_by_key(|(_, size)| *size)
            .unwrap();

    }
}