# permRS
### Permission Restore


## Utlity to save and restore unix permissions

```

USAGE:
    permrs [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -r, --restore    Restore unix permissions
    -s, --save       Save unix permissions
    -V, --version    Prints version information

OPTIONS:
    -p, --path <path>    Set path to save permissions [Default: ~/]
```

## Examples


1. To Save all permissions starting from root

   `sudo permrs -sp /`


2. To Save all permissions in your home folder

   `permrs -sp ~/`


3. To restore your permissions

   `permrs -r`
   
4. If you ran permrs -s as root use restore as root.

   `sudo permrs -r`
   
## Notes

permRS will automatically skip any directories or files you do not have access to as a regular user.

Run permRS as root if you want your permissions on all your directorties or files.

Do not use relative paths use full paths!

## Building
Requirements: Rust, Cargo

`git clone https://github.com/gort818/permrs`

`cd permrs`

`cargo build --release`

`Binary will be created in /permrs/target/release/`
