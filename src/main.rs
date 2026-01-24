mod config;
mod mod_checker;
use config::Config;

fn main()  {
    let config = Config::init();
    println!("{:?}", config);
}

