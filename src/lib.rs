#![feature(lazy_cell)]
#![feature(ptr_sub_ptr)]

pub mod dorifuto;

#[skyline::main(name = "DivineDragonDrifting")]
pub fn main() {
    skyline::install_hooks!(
        dorifuto::get_player_dash_stop_time_hook,
    );
    println!(
        "{}",
        format!("Divine Dragon Drifting Plugin  {}", env!("CARGO_PKG_VERSION"))
    );
    println!("Source code available at https://github.com/MistressAshai/DivineDragonDrifting");
}