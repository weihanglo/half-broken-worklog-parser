# A parser for my own half-broken worklog format 

## Requirements

- [Rust](https://rust-lang.org)
- [xsv](https://github.com/BurntSushi/xsv)
- [sort](https://en.wikipedia.org/wiki/Sort_(Unix))

## Recipe

```console
cargo run -- work.log work.log.csv
sort -u -t, -k5,5 work.log.csv | sort -t, -k1,1 | xsv frequency -n
```
