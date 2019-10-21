#[macro_use]
extern crate clap;
use clap::App;
extern crate dirs;
extern crate xdg;
mod save;
use save::save;
mod restore;
use restore::restore;
use std::env;
#[macro_use]
extern crate run_script;

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME")).unwrap();
    let config_dir = xdg_dirs.get_config_home();
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let home_dir = dirs::home_dir().unwrap();
    let home = home_dir.to_string_lossy();
    let dir_path = matches.value_of("path").unwrap_or(&home);
    if matches.is_present("save") {
        save(&xdg_dirs, &dir_path);
    } else if matches.is_present("restore") {
        restore(&config_dir);
    }
}
