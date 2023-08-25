mod inspector;
mod rpa_engine;
use env_logger::Env;

extern crate log;


fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);
    inspector::Inspector::start();

    
   
}
