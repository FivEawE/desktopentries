use structopt::StructOpt;

mod configuration;
pub mod entry;

fn main() {
    let conf = configuration::Configuration::from_args();
    println!("{:?}", conf);
}
