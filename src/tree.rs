use crate::results::Entry;
use crate::{results::Results, Arguments};
use std::fs::{self, DirEntry, Metadata};
use std::io::Read;

pub struct Tree {
    path: String,
    children: Vec<Tree>,
}

impl Tree {
    pub fn new(path: &String) -> Tree {
        Tree {
            path: path.clone(),
            children: Vec::new(),
        }
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn quick_fill(&mut self, depth: u8, path: String, args: &Arguments, results: &mut Results) {
        if depth == 0 {
            return;
        }

        match fs::metadata(&path) {
            Ok(x) => if !x.is_dir() { return; },
            _ => return,
        }

        let entries: Vec<DirEntry> = {
            match fs::read_dir(path) {
                Ok(x) => x,
                _ => return,
            }
        }.flatten().collect();

        let ignorelist = [".ignore", ".gitignore"];
        let mut ignore: Vec<String> = Vec::new();
        if let Some(entry) = entries
            .iter()
            .find(|x| ignorelist.contains(&x.file_name().to_str().unwrap()))
        {
            let separators: &[_] = &['\\', '/'];
            for line in fs::read_to_string(entry.path()).unwrap().lines() {
                ignore.push(line.to_owned().trim_matches(separators).to_owned());
            }
        }

        for entry in entries {
            let pathname: String = match entry.path().to_str() {
                Some(x) => x.to_string(),
                None => continue,
            };

            let filename: String = match entry.file_name().to_str() {
                Some(x) => x.to_string(),
                None => continue,
            };
                
            if (!args.all && filename.starts_with(".")) || 
            (args.useignore && ignore.contains(&filename))
            {
                continue;
            }
            
            let mut entry = Entry::new(&pathname);
            let metadata = fs::metadata(&pathname);
            if metadata.is_ok() {
                entry.set_metadata(&metadata.unwrap());

                if entry.is_dir() {
                    results.add_foldercount();
                } else {
                    results.add_filecount();
                }
            }
            
            if filename.contains(&args.pattern) {
                results.push(entry);
            }
            
            let mut tree: Tree = Tree::new(&pathname);
            tree.quick_fill(depth - 1, pathname, args, results);

            self.children.push(tree);
        }
    }

    pub fn search_infile(&mut self, depth: u8, path: String, args: &Arguments, results: &mut Results) {
        let metadata: Metadata = {
            match fs::metadata(&path) {
                Ok(x) => x, 
                _ => return
            }
        };

        if metadata.is_file() {
            let mut buffer: [u8; 1024] = [0; 1024];
            if let Ok(x) = fs::File::open(&path).as_mut() {
                if x.read(&mut buffer).is_err() {
                    return;
                }
            } else { return; }

            if content_inspector::inspect(&buffer[..]).is_binary() {
                return;
            }

            let content: String = {
                if let Ok(x) = fs::read_to_string(&path) {
                    if x.contains(&args.pattern) { x } else { return; }
                } else { return; }
            };

            for (linecount, result) in content.lines().enumerate() {
                if !result.contains(&args.pattern) { 
                    continue; 
                }

                let splitted_result: Vec<&str> = result.split(&args.pattern).collect();
                let mut colcount = 0;
                for chunk in splitted_result.iter().take(splitted_result.len() - 1) {
                    colcount += chunk.len();
                    let mut entry = Entry::new(
                        &format!("{}:{}:{}", path, linecount + 1, colcount)
                    );
                    entry.set_metadata(&metadata);
                    results.push(entry);
                }
            }

            return;
        }

        if depth == 0 {
            return;
        }

        let entries: Vec<DirEntry> = {
            match fs::read_dir(&path) {
                Ok(x) => x,
                _ => return
            }
        }.flatten().collect();

        let ignorelist = [".ignore", ".gitignore"];
        let mut ignore: Vec<String> = Vec::new();
        if let Some(entry) = entries
            .iter()
            .find(|x| ignorelist.contains(&x.file_name().to_str().unwrap()))
        {
            let separators: &[_] = &['\\', '/'];
            for line in fs::read_to_string(entry.path()).unwrap().lines() {
                ignore.push(line.to_owned().trim_matches(separators).to_owned());
            }
        }

        for entry in entries {
            let entry_path: String = match entry.path().to_str() {
                Some(x) => x,
                _ => continue
            }.to_string();

            let entry_file: String = match entry.file_name().to_str() {
                Some(x) => x,
                _ => continue
            }.to_string();

            if (!args.all && entry_file.starts_with("."))
            || (ignore.contains(&entry_file) && args.useignore) {
                continue;
            }

            if fs::metadata(&entry_path).is_ok() {
                if fs::metadata(&entry_path).unwrap().is_dir() {
                    results.add_foldercount();
                } else {
                    results.add_filecount();
                }
            }

            self.search_infile(depth - 1, entry_path, args, results);
        }
    }
}
