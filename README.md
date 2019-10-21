# permRS


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

```
1. to Save all permissions starting from root
permrs -sp /

2. To Save all permissions in your home folder

permrs -sp ~/

3. To restore your permissions

permrs -r
