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
pub struct Arguments {
    /// The pattern you want to search for
    #[arg()]
    pattern: String,

    /// How deep do you want to dig into?
    #[arg(short, long, default_value_t = 3)]
    depth: u8,

    /// Search through text-based file's contents
    #[arg(short = 'f', long, group = "internal")]
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
}

fn main() -> Result<()> {
    let instant = Instant::now();

    let args: Arguments = Arguments::parse();
    let mut tree: Tree = Tree::new(&".".to_string());
    let mut results: Results = Results::new();

    let stdout: Stdout = io::stdout();
    let mut buf_writer: BufWriter<Stdout> = io::BufWriter::new(stdout);

    if args.infile {
        tree.search_infile(args.depth, tree.path(), &args, &mut results);
    } else {
        tree.quick_fill(args.depth, tree.path(), &args, &mut results);
    }
    
    results.write(&mut buf_writer, args.verbose)?;
    let duration = instant.elapsed();

    writeln!(
        buf_writer,
        "\nFound {} results.\nSearched through {} file(s) and {} folder(s) in {} ms.",
        format!("{}", results.get_entries().len()),
        format!("{}", results.get_filecount()),
        format!("{}", results.get_foldercount()),
        format!("{}", duration.as_millis()),
    )?;

    buf_writer.flush()?;
    Ok(())
}
