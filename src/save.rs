use std::fs;
use std::fs::File;
use std::io::Write;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process;
use walkdir::WalkDir;

pub fn save(xdg_loc: &xdg::BaseDirectories, dir_arg: &str) {
    let dir_exists = Path::new(dir_arg).exists();
    if !dir_exists {
        println!("That path does not exist, Exiting.");
        process::exit(0);
    }
    let real_path = fs::canonicalize(&dir_arg).unwrap();
    let config_path = xdg_loc
        .place_config_file("restore.sh")
        .expect("cannot create configuration directory");
    let mut config_file = File::create(config_path).unwrap();
    writeln!(config_file, "#!/bin/bash").expect("could not write to file");
    for entry in WalkDir::new(real_path).into_iter().filter_map(|e| e.ok()) {
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
        let output = format!("chmod {} \"{}\"\n", last_four, entry.path().display());
        let echo = format!(
            "echo running chmod {} on \"{}\"\n",
            last_four,
            entry.path().display()
        );
        let final_output = format!("{}{}", output, echo);
        print!("{}", output);
        config_file
            .write_all(final_output.as_bytes())
            .expect("unable to write");
    }
}
