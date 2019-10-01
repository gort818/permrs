extern crate clap;
extern crate xdg;
use clap::{App, AppSettings, Arg};
use std::fs::File;

fn main() {
    let matches = App::new("savep")
        .version(env!("CARGO_PKG_VERSION"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .author("Alessandro Toia <gort818@gmail.com>")
        .about("Utlity to save and restore unix permissions")
        .arg(
            Arg::with_name("save")
                .short("s")
                .long("save")
                .help("Save unix permissions")
                .conflicts_with("restore"),
        )
        .arg(
            Arg::with_name("restore")
                .short("r")
                .long("restore")
                .help("Restore unix permissions")
                .conflicts_with("save"),
        )
        .get_matches();
    if matches.is_present("save") {
        println!("save");
        let xdg_dirs = xdg::BaseDirectories::with_prefix("savep").unwrap();
        let config_path = xdg_dirs
            .place_config_file("restore.sh")
            .expect("cannot create configuration directory");
        let _config_file = File::create(config_path)
            .expect("cannot create file");

    //find ./ -depth -printf 'chmod %m %p\n' > saved_permission
    } else if matches.is_present("restore") {
        println!("restore");
    }
}
