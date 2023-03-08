use chrono::{DateTime, Local};
use std::fs::Metadata;
use std::io::{self, BufWriter, Stdout, Write};
use std::path::PathBuf;
use std::time::SystemTime;

pub struct Results {
    entries: Vec<Entry>,
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

    pub fn write(&self, buf_writer: &mut BufWriter<Stdout>, verbose: bool) -> io::Result<()> {
        for (index, entry) in self.entries.iter().enumerate() {
            writeln!(buf_writer, "{}", entry.path)?;
            if verbose {
                write_metadata(&entry.metadata, buf_writer, index == self.entries.len() - 1)?;
            }
        }

        Ok(())
    }
}

pub struct Entry {
    path: String,
    metadata: Metadata,
}

impl Entry {
    pub fn new(path: &String) -> Self {
        let metadata = PathBuf::from(
            if path.contains(":") {
                path.split(":").next().unwrap()
            } else { path }
        ).metadata().expect("Valid metadata expected");

        Self {
            path: path.clone(),
            metadata,
        }
    }

    pub fn is_dir(&self) -> bool {
        self.metadata.is_dir()
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

    writeln!(
        buf_writer,
        "{:<15} {}",
        "Created:",
        created,
    )?;

    writeln!(
        buf_writer,
        "{:<15} {}",
        "Last modified:",
        last_modified,
    )?;

    let file_type = metadata.file_type();
    let perm = metadata.permissions();

    if file_type.is_file() {
        writeln!(
            buf_writer, 
            "{:<15} {}", 
            "File type:", 
            "file",
        )?;

        if metadata.len() > 1024 * 1024 * 1024 {
            writeln!(
                buf_writer,
                "{:<15} {}",
                "File size:",
                format!("{}GB", metadata.len() / (1024 * 1024 * 1024)),
            )?;
        } else if metadata.len() > 1024 * 1024 {
            writeln!(
                buf_writer,
                "{:<15} {}",
                "File size:",
                format!("{}MB", metadata.len() / (1024 * 1024)),
            )?;
        } else {
            writeln!(
                buf_writer,
                "{:<15} {}",
                "File size:",
                format!("{}KB", metadata.len() / 1024),
            )?;
        }
    } else {
        writeln!(
            buf_writer, 
            "{:<15} {}", 
            "File type:", 
            "dir",
        )?;
    }

    writeln!(
        buf_writer,
        "{:<15} {}",
        "Permissions:",
        format!("{}", if perm.readonly() { "read only" } else { "all" }),
    )?;

    if !last {
        writeln!(buf_writer)?;
    }

    Ok(())
}
