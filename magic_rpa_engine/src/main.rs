mod inspector;
mod rpa_engine;
use sysinfo::{System, SystemExt};

fn main() {
     
    inspector::Inspector::inspect_element_point();
}
