extern crate clap;
use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
    let matches = App::new("savep")
        .version(env!("CARGO_PKG_VERSION"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .author("Alessandro Toia <gort818@gmail.com>")
        .about("Utlity to save and restore unix permissions")
        .args_from_usage(
            "-s, --save 'Save all current unix permissions,
        -r, --restore 'Restore unix permissions'",
        )
        .get_matches();
}
