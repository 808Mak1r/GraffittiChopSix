use clap::Parser;


/// echo bef json
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "H <admin@sec.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: Subcommand,
}

#[derive(Debug, Parser)]
enum Subcommand {
    Get(Get),
    Post(Post),
}

/// get fn
#[derive(Debug, Parser)]
struct Get {
    /// http get url
    url: String,
}

/// post with an url data as JSON
#[derive(Debug, Parser)]
struct Post {
    /// http post url
    url: String,
    /// http post body
    body: Vec<String>,
}



fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts)
}
