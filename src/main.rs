struct Cli {
    pattern: String,
    path: std::path::PathBuf
}

fn main() {
    // args[0] = grrs always

    // args[1] = string to be found
    let pattern = std::env::args().nth(1).expect("no pattern given");
    // args[2] = file path
    let path = std::env::args().nth(2).expect("no path given");

    let args = Cli {
        pattern: pattern,
        path: std::path::PathBuf::from(path),
    };
}
