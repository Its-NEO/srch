mod results;
mod tree;

use crate::results::Results;
use clap::Parser;
use std::io::{self, BufWriter, Result, Stdout, Write};
use std::time::Instant;
use tree::Tree;

#[derive(Parser)]
#[command(name = "srch")]
#[command(version = "1.0")]
#[command(about = "A feature-rich search tool to find all you want.")]
#[derive(Clone)]
pub struct Arguments {
    /// The pattern you want to search for
    #[arg()]
    pattern: String,

    /// How deep do you want to dig into?
    #[arg(short, long, default_value_t = 3)]
    depth: u8,

    /// Search through text-based file's contents
    #[arg(short = 'f', long)]
    infile: bool,

    /// Search through hidden folders
    #[arg(short, long)]
    all: bool,

    /// Use ignore files to ignore certain files and folders
    #[arg(short = 'i', long)]
    useignore: bool,

    /// Display file information along with the path
    #[arg(short, long)]
    verbose: bool,

    /// Only view the paths
    #[arg(short, long)]
    pathonly: bool,
}

fn main() -> Result<()> {
    let instant = Instant::now();
    let args: Arguments = Arguments::parse();
    let mut tree: Tree = Tree::new(&".".to_string());

    let stdout: Stdout = io::stdout();
    let buf_writer: BufWriter<Stdout> = io::BufWriter::new(stdout);
    let mut results: Results = Results::new(buf_writer, args.clone());

    if args.infile {
        tree.search_infile(args.depth, tree.path(), &args, &mut results);
    } else {
        tree.quick_fill(args.depth, tree.path(), &args, &mut results);
    }

    let duration = instant.elapsed();
    let entries_len = results.get_entries().len();
    let filecount = results.get_filecount();
    let foldercount = results.get_foldercount();
    let duration_ms = duration.as_millis();

    if !args.pathonly {
        writeln!(
            &mut results.writer,
            "\nFound {} results.\nSearched through {} file(s) and {} folder(s) in {} ms.",
            format!("{}", entries_len),
            format!("{}", filecount),
            format!("{}", foldercount),
            format!("{}", duration_ms),
        )?;
    }

    Ok(())
}
