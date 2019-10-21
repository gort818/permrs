use std::env;
use std::fs::File;
use std::path::Path;
use std::process;
pub fn restore(run_dir: &Path) {
    let dir = env::set_current_dir(run_dir);
    match dir {
        Ok(_dir) => println!(
            "\nSuccessfully changed working directory to {} ",
            run_dir.display(),
        ),
        Err(error) => {
            println!("Please run the save option first!\nError: {}", error);
            process::exit(0);
        }
    }

    let file = File::open("restore.sh");
    match file {
        Ok(_file) => println!("\nrestore.sh found continuing... "),
        Err(error) => {
            println!(
                "restore.sh not found please run the save option again.\nError: {}",
                error
            );
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
