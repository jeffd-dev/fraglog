# Fraglog

Simple Rust CLI script to fragment logfile and extract only lines that correspond to a given period.

* Status: beta
* Datetime format supported is ISO 8601 (short version): "YYYY-MM-DD HH:MM:SS" (timezone or milliseconds are ignored). The separator between date and time can be any single character (T, Z or space).
* If logfile contains only the time then the lines can be extracted without specifying the date

## Usage

### Compilation

This project contains only the source file, the compilation with `rustc`(or `cargo`) is required to build the app.


```sh
cargo build --release
```

This code is not published on crates.io

### CLI usage

```sh
fraglog <log_filepath> <period_start> <period_end> ['verbose']
```

```sh
fraglog help
```


#### Examples

```sh
fraglog many_days_file.log "2025-01-01T00:00:00" "2025-01-05T23:59:59"
```

```sh
fraglog single_day_file.log "10:00:00" "11:30:00" 'verbose'
```
In this case, we don't prefix time periods with "T" (as required by the ISO8601)

This app expect are lines are sorted chronologically, with the oldest first. This script is not made for unordered logfile.


## Contribute

Contributing is welcome.

* Please, don't include dependencies, fork projet and send me a message to link your projet
* Due to IA, PR or issues are rejected if account has no project and seems not be done by an human.


### Improvment ideas (backlog)

* Delete duplication (`parse_datetime_logfile()`)
* Support other date-time format
* performance/security issue fixes


## Related project

* [Grep](https://www.man7.org/linux/man-pages/man1/grep.1.html)


## License

This code is [MIT licensed](https://mit-license.org/).