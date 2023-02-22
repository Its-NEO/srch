use std::io::{self, BufWriter, Stdout, Write};
use colored::Colorize;

pub struct Results {
    pub pathlist: Vec<String>,
    pub pattern: String,
    pub filecount: u32,
    pub foldercount: u32,
}

impl Results {
    pub fn new(pattern: &str) -> Self {
        // folder count is reduced by 1 so that it doesn't count the current dir as a folder,
        // only its contents.
        Self { pathlist: Vec::new(), pattern: pattern.to_owned(), filecount: 0, foldercount: 0 } 
    }

    pub fn push(&mut self, val: String) {
        self.pathlist.push(val);
    }

    pub fn add_filecount(&mut self) {
        self.filecount += 1;
    }

    pub fn add_foldercount(&mut self) {
        self.foldercount += 1;
    }

    /// This function takes the raw results returned by `Tree::traverse`, the pattern to filter for
    /// and the buffer to write the final results into, after highlighting it with the colors specified.
    pub fn colorize(&self, buf_writer: &mut BufWriter<Stdout>) -> io::Result<()> {
        // Two things done here:
        for result in self.pathlist.iter() {
            let mut result_vec: Vec<&str> = Vec::new();

            if result.contains(':') {
                result_vec = result.split(':').collect();
                writeln!(buf_writer, "{}:{}:{}", 
                    result_vec[0], 
                    result_vec[1].yellow(), 
                    result_vec[2].yellow())?;
                
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
        }

        Ok(())
    }
}