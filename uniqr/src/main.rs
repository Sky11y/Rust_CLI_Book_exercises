use clap::Parser;
use uniqr::Opts;

fn main() {
    let args = Opts::parse();

    if let Err(e) = uniqr::run(args) {
        eprintln!("{}", e);
        std::process::abort();
    }
}
