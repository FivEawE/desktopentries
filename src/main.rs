use structopt::StructOpt;

mod configuration;

fn main() {
    let conf = configuration::Configuration::from_args();
    println!("{:?}", conf);
}
