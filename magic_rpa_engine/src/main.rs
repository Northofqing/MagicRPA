mod inspector;
mod rpa_engine;
use env_logger::Env;
use rand::Rng;
extern crate log;


fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);
    point();
    // inspector::Inspector::start();
    // inspector::Inspector::stop();
}
fn point(){
    for i in 0..10 {
    let mut rng = rand::thread_rng();
    let _x =  rng.gen_range(0..1920);
    let _y =   rng.gen_range(0..1080);
    println!("Random X: {}",_x);
    println!("Random Y: {}", _y);
    inspector::Inspector::inspect_element_point_single(rpa_engine::rpa_core::point::MagicPoint { x:_x, y: _y });
    }
}
