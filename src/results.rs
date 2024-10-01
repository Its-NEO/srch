use chrono::{DateTime, Local};
use std::fs::Metadata;
use std::io::{self, BufWriter, Stdout, Write};
use std::path::PathBuf;
use std::time::SystemTime;

use crate::Arguments;
const THRESHOLD: usize = 100;

pub struct Results {
    pub entries: Vec<Entry>,
    filecount: usize,
    foldercount: usize,
    pub writer: BufWriter<Stdout>,
    args: Arguments,
    index: usize,
}

impl Results {
    pub fn new(new_writer: BufWriter<Stdout>, new_args: Arguments) -> Self {
        Self {
            entries: Vec::new(),
            filecount: 0,
            foldercount: 0,
            writer: new_writer,
            args: new_args,
            index: 0,
        }
    }

    pub fn push(&mut self, entry: Entry) {
        if self.index == THRESHOLD
        {
            println!("Too many results... Writing it to a file.");
            // TODO: write the results to a file

            return;
        }
        if let Err(e) = self.write_consl(&entry) {
            eprintln!("Failed to write entry: {}", e);
        }
        self.entries.push(entry);
        self.index += 1;
    }

    pub fn add_filecount(&mut self) {
        self.filecount += 1;
    }

    pub fn get_filecount(&self) -> usize {
        self.filecount
    }

    pub fn add_foldercount(&mut self) {
        self.foldercount += 1;
    }

    pub fn get_foldercount(&self) -> usize {
        self.foldercount
    }

    pub fn get_entries(&self) -> &Vec<Entry> {
        &self.entries
    }

    fn write_file(&self)
    {
        // later
    }

    fn write_consl(&mut self, entry: &Entry) -> io::Result<()> {
        if self.args.pathonly {
            if entry.path.contains(":") {
                writeln!(self.writer, "{}", entry.path.split(":").next().unwrap())?;
            } else {
                writeln!(self.writer, "{}", entry.path)?;
            }

            return Ok(());
        }

        writeln!(self.writer, "{}", entry.path)?;

        if self.args.verbose {
            if let Some(x) = &entry.metadata {
                self.write_metadata(x)?;
            }
        }

        self.writer.flush()?;

        Ok(())
    }

    fn write_metadata(
        &mut self,
        metadata: &Metadata,
    ) -> io::Result<()> {

        let system_time = if let Ok(x) = metadata.created() {
            if x.elapsed().is_ok() {
                Some(x)
            } else {
                None
            }
        } else {
            None
        };

        let created = format_dt(system_time);
        let last_modified = if let Ok(x) = metadata.modified() {
            format_dt(Some(x))
        } else {
            "_".to_string()
        };

        writeln!(self.writer, "{:<15} {}", "Created:", created,)?;

        writeln!(self.writer, "{:<15} {}", "Last modified:", last_modified,)?;

        let file_type = metadata.file_type();
        let perm = metadata.permissions();

        if file_type.is_file() {
            writeln!(self.writer, "{:<15} {}", "File type:", "file",)?;

            if metadata.len() > 1024 * 1024 * 1024 {
                writeln!(
                    self.writer,
                    "{:<15} {}",
                    "File size:",
                    format!("{}GB", metadata.len() / (1024 * 1024 * 1024)),
                )?;
            } else if metadata.len() > 1024 * 1024 {
                writeln!(
                    self.writer,
                    "{:<15} {}",
                    "File size:",
                    format!("{}MB", metadata.len() / (1024 * 1024)),
                )?;
            } else {
                writeln!(
                    self.writer,
                    "{:<15} {}",
                    "File size:",
                    format!("{}KB", metadata.len() / 1024),
                )?;
            }
        } else {
            writeln!(self.writer, "{:<15} {}", "File type:", "dir",)?;
        }

        writeln!(
            self.writer,
            "{:<15} {}",
            "Permissions:",
            format!("{}", if perm.readonly() { "read only" } else { "all" }),
        )?;

        Ok(())
    }
}

#[derive(Clone)]
pub struct Entry {
    path: String,
    metadata: Option<Metadata>,
}

impl Entry {
    pub fn new(path: &String) -> Self {
        let metadata = PathBuf::from(if path.contains(":") {
            path.split(":").next().unwrap()
        } else {
            path
        })
        .metadata();

        if metadata.is_err() {
            return Self {
                path: path.clone(),
                metadata: None
            }
        }

        Self {
            path: path.clone(),
            metadata: Some(metadata.unwrap()),
        }
    }

    pub fn is_dir(&self) -> bool {
        if let Some(x) = &self.metadata {
            return x.is_dir()
        } else { false }
    }
}

fn format_dt(system_time: Option<SystemTime>) -> String {
    if let Some(x) = system_time {
        let datetime: DateTime<Local> = x.into();
        datetime.format("%d/%m/%y %T").to_string()
    } else {
        "_".to_owned()
    }
}
