mod config;
mod mod_checker;

use config::*;
use mod_checker::*;

fn main()  {
    let mut config = Config::init();
    get_mod_info(&mut config);
}
