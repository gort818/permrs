#[macro_use]
extern crate clap;
use clap::{App, AppSettings, Arg};
extern crate xdg;
use std::env;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::process;
use walkdir::WalkDir;
#[macro_use]
extern crate run_script;

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME")).unwrap();
    let config_dir = xdg_dirs.get_config_home();
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
        // .version(env!("CARGO_PKG_VERSION"))
        // .setting(AppSettings::ArgRequiredElseHelp)
        // .author("Alessandro Toia <gort818@gmail.com>")
        // .about("Utlity to save and restore unix permissions")
        // .arg(
        //     Arg::with_name("save") // add option for location and if not provided use / "root"
        //         .short("s")
        //         .long("save")
        //         .help("Save unix permissions")
        //         .conflicts_with("restore"),
        // )
        // .arg(
        //     Arg::with_name("restore")
        //         .short("r")
        //         .long("restore")
        //         .help("Restore unix permissions")
        //         .conflicts_with("save"),
        // )
        //.get_matches();
    if matches.is_present("save") {
        //let xdg_dirs = xdg::BaseDirectories::with_prefix("savep").unwrap();
        let config_path = xdg_dirs
            .place_config_file("restore.sh")
            .expect("cannot create configuration directory");
        println!("{:?}", xdg_dirs.get_config_home());
        let mut config_file = File::create(config_path).unwrap();
        writeln!(config_file, "#!/bin/bash").expect("could not write to file");
        // let (code, output, error) = run_script!(
        //     r#"
        // find ~+ ~/ -depth  -printf 'chmod  %m %p \n'
        // exit 0
        // "#
        // )
        // .unwrap();

        // println!("Output: \n{}", output);
        // println!("Error: \n{}", error);
        // println!("Exit Code: \n{}", code);
        for entry in WalkDir::new("/home/alessandro/test/")
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let entry = entry;
            let permissions = entry.metadata().unwrap().permissions().mode();
            let perm_oct = format!("{:o}", permissions);
            let last_four_at = perm_oct
                .char_indices()
                .rev()
                .map(|(i, _)| i)
                .nth(3)
                .unwrap();
            let last_four = &perm_oct[last_four_at..];
            //TODO sperate output and echo command for restore.sh
            let output = format!(
                "chmod {} \"{}\"\necho running chmod {} on \"{}\"\n",
                last_four,
                entry.path().display(),
                last_four,
                entry.path().display()
            );
            print!("{}", output);
            config_file
                .write_all(output.as_bytes())
                .expect("unable to write");
        }
    } else if matches.is_present("restore") {
        let dir = env::set_current_dir(&config_dir);
        //TODO also need to check if restore.sh exists!
        match dir {
            Ok(_dir) => println!(
                "Successfully changed working directory to {} ",
                config_dir.display(),
            ),
            Err(error) => {
                println!("Please run the save option first!\nError: {}", error);
                process::exit(0);
            }
        }
        println!("Restoring permissions please wait...\n");
        let (code, output, error) = run_script!(
            r#"
         chmod +x ./restore.sh
        ./restore.sh
        exit 0
        "#
        )
        .unwrap();

        println!("Output: \n{}", output);
        println!("Error: {}", error);
        println!("Exit Code: {}", code);
    }
}
