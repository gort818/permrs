extern crate clap;
extern crate cmd_lib;
extern crate xdg;
use clap::{App, AppSettings, Arg};
use cmd_lib::run_cmd;
use std::fs::File;
//use filepath::FilePath;
use std::path::PathBuf;
//use std::io::Error;
use std::os::unix::io::{FromRawFd, IntoRawFd};
use std::process::{Command, Stdio};
use std::fs;
use std::io::Write;
//use std::io::Command;
#[macro_use]
extern crate run_script;

use run_script::ScriptOptions;

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
        let mut config_file = (File::create(config_path).unwrap());
        let (code, output, error) = run_script!(
            r#"
        find /home/alessandro/test/ -depth  -printf 'chmod  %m %p \n'
        exit 0
        "#
        )
        .unwrap();

        println!("Exit Code: {}", code);
    println!("Output: {}", output);
   // println!("Error: {}", error);
    println!("path{:#?}",config_file);
     config_file.write_all(output.as_bytes()).expect("unable to write");
    //fs::write(config_file, output).expect("Unable to write file");
   //config_file.tawdawd();
    } else if matches.is_present("restore") {
        println!("restore");
    }
}
