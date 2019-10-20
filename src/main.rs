#[macro_use]
extern crate clap;
use clap::App;
extern crate xdg;
use std::env;
use std::fs::{self, File};
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process;
use walkdir::WalkDir;
#[macro_use]
extern crate run_script;

fn main() {
    let xdg_dirs = xdg::BaseDirectories::with_prefix(env!("CARGO_PKG_NAME")).unwrap();
    let config_dir = xdg_dirs.get_config_home();
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    let mut dir_path = matches.value_of("path").unwrap();
    let dir_exists = Path::new(dir_path).exists();
    if matches.is_present("save") {
        if !dir_exists {
            println!("That path does not exist, Exiting.");
            process::exit(0);
        }
        let config_path = xdg_dirs
            .place_config_file("restore.sh")
            .expect("cannot create configuration directory");
        //println!("{:?}", xdg_dirs.get_config_home());
        let mut config_file = File::create(config_path).unwrap();
        writeln!(config_file, "#!/bin/bash").expect("could not write to file");
        for entry in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
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
