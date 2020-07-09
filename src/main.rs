use structopt::StructOpt;

mod checker;
mod configuration;
mod entry;

fn main() {
    let conf = configuration::Configuration::from_args();
    println!("{:?}", conf);
}
