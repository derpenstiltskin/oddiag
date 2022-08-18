# oddiag

OneDrive utility written in Rust.

## Help

```
USAGE:
    oddiag.exe [OPTIONS]

OPTIONS:
        --account <USERNAME>        Scopes backup and report to specified user account
        --backup <PATH>             Backup local saved OneDrive files (preserves folder structure)
        --disablehealthreporting    Disables OneDrive health reporting
        --enablehealthreporting     Enables OneDrive health reporting, must be enabled at
                                    https://config.office.com
        --fixhiddenlogin            Fixes missing OneDrive login window on MFA'ed accounts
    -h, --help                      Print help information
        --report <PATH>             Generate CSV report of local saved OneDrive files
    -V, --version                   Print version information
```

## Todo

- Better path handling
- Clap validators for input
