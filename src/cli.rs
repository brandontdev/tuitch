use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    pub channel: String,
    pub user: String,
}
