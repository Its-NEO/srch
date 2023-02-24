use std::io::{Result, Read};
use std::fs::{self, Metadata, ReadDir, DirEntry, File};
use crate::{Arguments, results::Results};

/// This represents a single node in the nodal representation of the file tree that the program implements.
/// This is the <3 heart of the whole process.
#[derive(Debug)]
pub struct Tree {
    path: String,
    children: Vec<Tree>
}

impl Tree {
    /// This function will create a new generic tree.
    pub fn new() -> Tree {
        Tree {
            path: ".".to_string(),
            children: Vec::new()
        }
    }

    /// Returns a path after cloning it.
    pub fn path(&self) -> String {
        self.path.clone()
    }

    /// Given the starting path and the depth, this function will fill up a file tree traversing upto
    /// the requested depth. This function is mutable in nature.
    /// 
    /// Arguments: 
    /// `depth` - The depth until which the recursive tree will search into,
    /// `path` - The path to start searching from
    /// 
    /// ```
    /// let tree = Tree::new();
    /// tree.fill(2, ".");
    /// ```
    pub fn quick_fill(&mut self, depth: u8, path: String, args: &Arguments, results: &mut Results) {
        // base case
        if depth == 0 {
            return
        }

        // return if the path is a file or a syslink or let's say, is not a directory
        let metadata: Result<Metadata> = fs::metadata(&path);
        if metadata.is_err() { return; }
        if !metadata.unwrap().is_dir() { 
            return; 
        }

        // loop over the contents of the directory
        let dir_entries: Result<ReadDir> = fs::read_dir(path);
        if dir_entries.is_err() { return; }

        // entries are stored in a Vector after flattening it
        let mut entries: Vec<DirEntry> = Vec::new();
        for entry in dir_entries.unwrap() {
            if entry.is_err() { continue; }
            entries.push(entry.unwrap());
        }

        // ignore list is created with all mentioned files and dirs
        let ignorelist = [".ignore", ".gitignore"];
        let mut ignore: Vec<String> = Vec::new();
        if let Some(entry) = entries.iter()
        .find(|x| ignorelist.contains(&x.file_name().to_str().unwrap())) {

            // read each line in an ignore file and push to ignore after trimming it
            for line in fs::read_to_string(entry.path()).unwrap().lines() {
                let separators: &[_] = &['\\', '/'];
                ignore.push(line.to_owned().trim_matches(separators).to_owned());
            }
        }

        for entry in entries {
            // the path to the entry
            let entry_path: String = entry.path().to_str()
                .expect("Invalid Path")
                .to_string();

            // the path to the file name
            let entry_file: String = entry.file_name().to_str()
                .expect("Invalid File Name")
                .to_string();

            // Return if the hidden flag is off and the entry name starts with "." 
            if !args.all {
                if entry_file.starts_with(".") { continue; } 
            }

            // if the entry is mentioned in the specified ignore list, ignore it OwO
            if ignore.contains(&entry_file.to_owned()).to_owned() 
                && args.useignore { continue; }
            
            // make a new tree 
            let mut tree: Tree = Tree::new();
            tree.path = entry_path.clone();

            if fs::metadata(&entry_path).is_ok() {
                let metadata = fs::metadata(&entry_path).unwrap();
                results.metadata.push(metadata.clone());
                
                if metadata.is_dir() {
                    results.add_foldercount();
                } else {
                    results.add_filecount();
                }
            }                

            if entry_file.contains(&args.pattern) {
                results.pathlist.push(entry_path.clone());
            } else if fs::metadata(&entry_path).is_ok() {
                results.metadata.pop();
            }

            // go inside this entry directory(if), then rinse and repeat
            tree.quick_fill(depth - 1, entry_path, args, results);
            self.children.push(tree);
        }
    }
    
    /// Check if the pattern exists inside the contents of a file,
    /// given that the file is not a binary file
    pub fn infile(&mut self, depth: u8, path: String, args: &Arguments, results: &mut Results) {
        // return if the path is a file or a syslink or let's say, is not a directory
        let metadata: Result<Metadata> = fs::metadata(&path);
        if metadata.is_err() { return; }
        if metadata.as_ref().unwrap().is_file() { 
            let mut file: File = match fs::File::open(&path) {Ok(x) => x, _ => return };
                
            // inspect the file to check if its a binary file >:( 
            let mut buffer = [0; 1024];
            match file.read(&mut buffer) {Ok(_) => {}, _ => return};
            if content_inspector::inspect(&buffer[..]).is_binary() { return; }
            
            // read the file to a string
            let content: String = match fs::read_to_string(&path) {Ok(x) => x, _ => return };
            
            // does it even contain the pattern?
            if !content.contains(&args.pattern) { return; }
            
            /* Here, the real magic happens:
            *  - Lines are being counted and 
            *  - Characters before the pattern in each line are being counted
            */
            for (linecount, result) in content.lines().enumerate() {
                if !result.contains(&args.pattern) { continue; }
                
                let splitted_result: Vec<&str> = result.split(&args.pattern).collect();
                let mut last = 0;
                for takeout in splitted_result.iter().take(splitted_result.len() - 1) {
                    last += takeout.len();
                    results.pathlist.push(format!(
                        "{}:{}:{}", path, last, linecount
                    ));

                    results.metadata
                        .push(metadata.as_ref()
                        .unwrap().clone());
                }
            }
            
            return;
        }
        
        // base case
        if depth == 0 {
            return;
        }

        // loop over the contents of the directory
        let dir_entries: Result<ReadDir> = fs::read_dir(&path);
        if dir_entries.is_err() { return; }
        
        // entries are stored in a Vector after flattening it
        let mut entries: Vec<DirEntry> = Vec::new();
        for entry in dir_entries.unwrap() {
            if entry.is_err() { continue; }
            entries.push(entry.unwrap());
        }

        // ignore list is created with all mentioned files and dirs
        let ignorelist = [".ignore", ".gitignore"];
        let mut ignore: Vec<String> = Vec::new();
        if let Some(entry) = entries.iter()
        .find(|x| ignorelist.contains(&x.file_name().to_str().unwrap())) {

            // read each line in an ignore file and push to ignore after trimming it
            for line in fs::read_to_string(entry.path()).unwrap().lines() {
                let separators: &[_] = &['\\', '/'];
                ignore.push(line.to_owned().trim_matches(separators).to_owned());
            }
        }
        
        for entry in entries {
            // the path to the entry
            let entry_path: String = entry.path().to_str()
            .expect("Invalid Path")
            .to_string();
        
        // the path to the file name
            let entry_file: String = entry.file_name().to_str()
                .expect("Invalid File Name")
                .to_string();
            
            // Return if the hidden flag is off and the entry name starts with "." 
            if !args.all {
                if entry_file.starts_with(".") { continue; } 
            }
            
            // if the entry is mentioned in the specified ignore list, ignore it OwO
            if ignore.contains(&entry_file.to_owned()).to_owned() 
            && args.useignore { continue; }
            
            if fs::metadata(&entry_path).is_ok() {
                if fs::metadata(&entry_path).unwrap().is_dir() {
                    results.add_foldercount();
                } else {
                    results.add_filecount();
                }
            }

            self.infile(depth - 1, entry_path, args, results);
        }
    }
}