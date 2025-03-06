use chrono::{DateTime, Local};
use std::fs::Metadata;
use std::io::{self, BufWriter, Stdout, Write};
use std::path::PathBuf;
use std::time::SystemTime;

use crate::Arguments;

pub struct Results {
    pub entries: Vec<Entry>,
    filecount: usize,
    foldercount: usize,
}

impl Results {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            filecount: 0,
            foldercount: 0,
        }
    }

    pub fn push(&mut self, entry: Entry) {
        self.entries.push(entry);
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

    pub fn write(&self, buf_writer: &mut BufWriter<Stdout>, args: &Arguments) -> io::Result<()> {
        for (index, entry) in self.entries.iter().enumerate() {
            if args.pathonly {
                if entry.path.contains(":") {
                    writeln!(buf_writer, "{}", entry.path.split(":").next().unwrap())?;
                } else {
                    writeln!(buf_writer, "{}", entry.path)?;
                }
                continue;
            }

            writeln!(buf_writer, "{}", entry.path)?;

            if args.verbose {
                if let Some(x) = &entry.metadata {
                    write_metadata(x, buf_writer, index == self.entries.len() - 1)?;
                }
            }
        }

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
        self.metadata.clone().is_some_and(|x| x.is_dir())
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

fn write_metadata(
    metadata: &Metadata,
    buf_writer: &mut BufWriter<Stdout>,
    last: bool,
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

    writeln!(buf_writer, "{:<15} {}", "Created:", created,)?;

    writeln!(buf_writer, "{:<15} {}", "Last modified:", last_modified,)?;

    let file_type = metadata.file_type();
    let perm = metadata.permissions();

    if file_type.is_file() {
        writeln!(buf_writer, "{:<15} file", "File type:")?;

        if metadata.len() > 1024 * 1024 * 1024 {
            writeln!(
                buf_writer,
                "{:<15} {}",
                "File size:",
                format_args!("{}GB", metadata.len() / (1024 * 1024 * 1024)),
            )?;
        } else if metadata.len() > 1024 * 1024 {
            writeln!(
                buf_writer,
                "{:<15} {}",
                "File size:",
                format_args!("{}MB", metadata.len() / (1024 * 1024)),
            )?;
        } else {
            writeln!(
                buf_writer,
                "{:<15} {}",
                "File size:",
                format_args!("{}KB", metadata.len() / 1024),
            )?;
        }
    } else {
        writeln!(buf_writer, "{:<15} dir", "File type:")?;
    }

    writeln!(
        buf_writer,
        "{:<15} {}",
        "Permissions:",
        format_args!("{}", if perm.readonly() { "read only" } else { "all" }),
    )?;

    if !last {
        writeln!(buf_writer)?;
    }

    Ok(())
}
