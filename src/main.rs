use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]

struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let result = std::fs::read_to_string("test.txt");
    let content = match result {
        Ok(content) => {
            content
        }
        Err(error) => {
             panic!("Can't deal with {}, just exit here", error); 
             // or maybe
        }
    };

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line.trim());
        }
    }
}
