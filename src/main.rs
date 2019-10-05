extern crate clap;
extern crate xdg;
use clap::{App, AppSettings, Arg};
use std::env;
use std::fs::File;
use std::io::Write;
use std::process;
#[macro_use]
extern crate run_script;

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("savep").unwrap();
    let config_dir = xdg_dirs.get_config_home();
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
        //let xdg_dirs = xdg::BaseDirectories::with_prefix("savep").unwrap();
        let config_path = xdg_dirs
            .place_config_file("restore.sh")
            .expect("cannot create configuration directory");
        println!("{:?}", xdg_dirs.get_config_home());
        let mut config_file = File::create(config_path).unwrap();
        writeln!(config_file, "#!/bin/bash").expect("could not write to file");
        let (code, output, error) = run_script!(
            r#"
        find ~+ ~/ -depth  -printf 'chmod  %m %p \n'
        exit 0
        "#
        )
        .unwrap();

        println!("Output: \n{}", output);
        println!("Error: \n{}", error);
        println!("Exit Code: \n{}", code);
        config_file
            .write_all(output.as_bytes())
            .expect("unable to write");
        println!("{:?}", xdg_dirs.get_config_home());
    } else if matches.is_present("restore") {
        let dir = env::set_current_dir(&config_dir);
        match dir {
            Ok(dir) => println!(
                "Successfully changed working directory to {} {:?}",
                config_dir.display(),
                dir
            ),
            Err(error) => {
                println!("Please run the save option first!\nError: {}", error);
                process::exit(0);
            }
        }
        println!("Restoring permissions please wait...");
        let (code, output, error) = run_script!(
            r#"
         chmod +x ./restore.sh
        ./restore.sh
        exit 0
        "#
        )
        .unwrap();

        println!("Exit Code: {}", code);
        println!("Output: {}", output);
        println!("Error: {}", error);
    }
}
