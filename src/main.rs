mod tree;
mod results;

use crate::results::Results;
use clap::Parser;
use std::time::Instant;
use std::io::{self, BufWriter, Stdout, Write, Result};
use tree::Tree;

#[derive(Parser)]
#[command(name = "srch")]
#[command(author = "Rakib M. <rakibmondal2155@gmail.com>")]
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
    #[arg(short='f', long)]
    infile: bool,

    /// Search through hidden folders
    #[arg(short, long)]
    all: bool,

    /// Use ignore files to ignore certain files and folders
    #[arg(short='i', long)]
    useignore: bool,

    /// Give me the first n results
    #[arg(short, long)]
    count: Option<usize>
}

fn main() -> Result<()> {
    let instant = Instant::now();

    let args: Arguments = Arguments::parse();
    let mut tree: Tree = Tree::new();
    let mut results: Results = Results::new(&args.pattern);
    
    let stdout: Stdout = io::stdout();
    let mut buf_writer: BufWriter<Stdout> = io::BufWriter::new(stdout);

    
    // Can only print either infile or non-infile
    if args.infile {
        tree.infile(args.depth, tree.path(), &args, &mut results);
        
    } else {
        // if count is provided, use it to filter it
        tree.quick_fill(args.depth, tree.path(), &args, &mut results);
    }

    results.colorize(&mut buf_writer)?;
    
    // so this is how long the program took to run :D
    let duration = instant.elapsed();

    writeln!(buf_writer, "\nFound {} results.\nSearched through {} file(s) and {} folder(s) in {} ms.", 
        results.pathlist.len(),
        results.filecount, 
        results.foldercount, 
        duration.as_millis())?;
    
    buf_writer.flush()?;
    Ok(())
}