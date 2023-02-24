use std::io::{self, BufWriter, Stdout, Write};
use std::fs::Metadata;
use std::os::unix::prelude::PermissionsExt;
use std::time::SystemTime;
use colored::Colorize;
use chrono::{DateTime, Local};

pub struct Results {
    pub pathlist: Vec<String>,
    pub pattern: String,
    pub filecount: u32,
    pub foldercount: u32,
    pub metadata: Vec<Metadata>
}

fn formatted_dt(system_time: Option<SystemTime>) -> String {
    if let Some(x) = system_time {
        let datetime: DateTime<Local> = x.into();
        datetime.format("%d/%m/%y %T").to_string()
    } else { "_".to_owned() }
}

fn parse_metadata(metadata: &Metadata, buf_writer: &mut BufWriter<Stdout>, last: bool) -> io::Result<()> {
    let system_time = if let Ok(x) = metadata.created() {
        if x.elapsed().is_ok() { Some(x) } else { None }
    } else { None };

    let created = formatted_dt(system_time);
    let file_type = metadata.file_type();
    let perm = metadata.permissions();
    let last_modified = if let Ok(x) = metadata.modified() {
        formatted_dt(Some(x))
    } else { "_".to_string() };
    
    write!(buf_writer, "{}:        {}\n", 
        "Created".underline(), 
        created.bold())?;
        
    writeln!(buf_writer, "{}: {}", 
        "Last modified".underline(), 
        last_modified.bold())?;

    if file_type.is_file() {
        write!(buf_writer, "{}: {}", 
            "File type".underline(), 
            "file ".bold())?;

        if metadata.len() > 1024 * 1024 {
            writeln!(buf_writer, "{}: {}", 
                "File size".underline(), 
                format!("{}MB", metadata.len() / (1024 * 1024)).bold())?;
        } else {
            writeln!(buf_writer, "{}: {}", 
                "File size".underline(), 
                format!("{}KB", metadata.len() / 1024).bold())?;
        }

    } else  {
        write!(buf_writer, "{}: {}", 
            "File type".underline(), 
            "dir  ".bold())?;
    }

    writeln!(buf_writer, "{}: {}", 
        "Permissions".underline(), 
        format!("{:o}", perm.mode()).bold())?;
    
    if !last {
        writeln!(buf_writer)?;
    }

    Ok(())
}

impl Results {
    pub fn new(pattern: &str) -> Self {
        // folder count is reduced by 1 so that it doesn't count the current dir as a folder,
        // only its contents.
        Self { 
            pathlist: Vec::new(), 
            pattern: pattern.to_owned(), 
            filecount: 0, 
            foldercount: 0, 
            metadata: Vec::new() 
        } 
    }

    pub fn add_filecount(&mut self) {
        self.filecount += 1;
    }

    pub fn add_foldercount(&mut self) {
        self.foldercount += 1;
    }

    /// This function takes the raw results returned by `Tree::traverse`, the pattern to filter for
    /// and the buffer to write the final results into, after highlighting it with the colors specified.
    pub fn colorize(&self, buf_writer: &mut BufWriter<Stdout>, verbose: bool) -> io::Result<()> {
        // Two things done here:
        for (index, result) in self.pathlist.iter().enumerate() {
            let mut result_vec: Vec<&str> = Vec::new();
            let metadata = self.metadata[index].clone();
            
            if result.contains(':') {
                result_vec = result.split(':').collect();
                writeln!(buf_writer, "{}:{}:{}", 
                    result_vec[0], 
                    result_vec[1].green().bold(), 
                    result_vec[2].green().bold())?;

                if verbose {
                    parse_metadata(&metadata, buf_writer, 
                        index == self.pathlist.len() - 1)?;
                }
                continue;
            }

            /*
            * 1. Every result is splitted inclusively with the separators.
            *    Example: result = "abcdefghefgh"
            *             final_result = ["abcd", "e", "fgh", "e", "fgh"]
            */
            let mut last: usize = 0;
            for (index, matched) in result.match_indices(&self.pattern) {
                if last != index {
                    result_vec.push(&result[last..index]);
                }
                result_vec.push(matched);
                last = index + matched.len();
            }

            if last < result.len() {
                result_vec.push(&result[last..]);
            } // step 1 ends here
            
            /*
            * The result vector is reversed and the position of the last match is found
            * Taking the position, only the item that matches with the position is highlighted
            */
            // The result is substracted from its length because the vector is reversed in order to find its last match
            let pos: usize = result_vec.len() - result_vec.iter().rev().position(|x| x == &self.pattern).unwrap() - 1;
            for (i, res) in  result_vec.into_iter().enumerate() {    
                if i == pos {
                    write!(buf_writer, "{}", res.green().bold())?;
                } else {
                    write!(buf_writer, "{}", res)?;
                }
            }
            writeln!(buf_writer)?;

            if verbose {
                parse_metadata(&metadata, buf_writer, 
                    index == self.pathlist.len() - 1)?;
            }
        }

        Ok(())
    }
}