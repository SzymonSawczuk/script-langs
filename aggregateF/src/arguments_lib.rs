use clap::Parser;

/// Aggregate
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Find minimum value in vector of values
    #[clap(long)]
    pub minimum: bool,

    /// Find maximum value in vector of values
    #[clap(long)]
    pub maximum: bool,

    /// Find sum of values in vector of values
    #[clap(short, long)]
    pub sum: bool,

    /// Find avarage value in vector of values
    #[clap(short, long)]
    pub avarage: bool,

    /// Count amout of values in vector
    #[clap(short, long)]
    pub count: bool,

    /// Find median value in vector of values
    #[clap(long)]
    pub median: bool,

    /// Find variance in vector of values
    #[clap(short, long)]
    pub variance: bool,

    /// Don't show short description of results
    #[clap(short, long)]
    pub nodesc: bool,
}
