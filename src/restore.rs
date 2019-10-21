use std::env;
use std::path::Path;
use std::process;
pub fn restore(run_dir: &Path) {
    println!("the dir {}", run_dir.display());
    let dir = env::set_current_dir(run_dir);
    //TODO also need to check if restore.sh exists!
    match dir {
        Ok(_dir) => println!(
            "Successfully changed working directory to {} ",
            run_dir.display(),
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
