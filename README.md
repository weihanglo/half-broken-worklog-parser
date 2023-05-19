# A parser for my own half-broken worklog format 

## Requirements

- [Rust](https://weihanglo.org)
- [xsv](https://github.com/BurntSushi/xsv)
- [sort](https://en.wikipedia.org/wiki/Sort_(Unix))

## Worklog format

The worklog may look like

```markdown
## 2023-05-16

* Issue triages
    * Closed https://github.com/weihanglo/cargo/issues/12140
    * Commented https://github.com/weihanglo/cargo/issues/12114
* FCP reviews
    * Merged https://github.com/weihanglo/cargo/pull/12146
* PR submissions
    * Created https://github.com/weihanglo/rust-analyzer/pull/14819

## 2023-05-15

* Issue triages
    * Closed https://github.com/weihanglo/cargo/issues/2995
    * Tracked https://github.com/weihanglo/cargo/issues/10490
    * Commented https://github.com/weihanglo/cargo/issues/4184
* PR reviews
    * Merged https://github.com/weihanglo/cargo/pull/12143
```

## Recipes

```console
cargo run -- work.log work.log.csv
sort -u -t, -k5,5 work.log.csv | sort -t, -k1,1 | xsv frequency -n
```
