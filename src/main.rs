use std::fmt::Display;
use std::fs::File;
use std::io::{BufReader, BufRead, Write};
use std::process;
use std::str::FromStr;
use std::error;
use std::fmt;
use std::env;

type BoxResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Debug, Default, Clone, Copy)]
enum Kind {
    #[default]
    IssueTriage,
    RfcReview,
    FcpReview,
    PrReview,
    PrSubmission,
    Discussion,
    Research,
}

impl FromStr for Kind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let kind = match s {
            "Issue triages" => Kind::IssueTriage,
            "RFC reviews" => Kind::RfcReview,
            "FCP reviews" => Kind::FcpReview,
            "PR reviews" => Kind::PrReview,
            "PR submissions" => Kind::PrSubmission,
            "Discussions" => Kind::Discussion,
            "Researches" => Kind::Research,
            _ => return Err(format!("Unrecognizable kind `{s}`")),
        };
        Ok(kind)
    }
}

#[derive(Debug)]
enum Action {
    Closed,
    Commented,
    Created,
    Merged,
    Tracked,
    Updated,
}

impl FromStr for Action {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = match s {
            "Closed" => Action::Closed,
            "Commented" => Action::Commented,
            "Created" => Action::Created,
            "Merged" => Action::Merged,
            "Tracked" => Action::Tracked,
            "Updated" => Action::Updated,
            _ => return Err(format!("Unrecognizable action `{s}`")),
        };
        Ok(action)
    }
}

#[derive(Debug)]
struct Record {
    date: String,
    kind: Kind,
    action: Action,
    url: String,
    canonical_url: String,
}

impl Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            date,
            kind,
            action,
            url,
            canonical_url,
        } = self;
        write!(f, "{date},{kind:?},{action:?},{url},{canonical_url}")
    }
}

#[derive(Debug)]
struct Error(String);

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "error: {}", self.0)
    }
}

impl error::Error for Error {}

/// Just to lazy to create my own error.
fn error(msg: String) -> Box<Error> {
    Box::new(Error(msg))
}

fn error_lnr(lnr: usize, err: impl fmt::Display) -> Box<Error> {
    error(format!("broken on line {lnr}: {err}"))
}


fn parse_item(date: &str, kind: Kind, text: &str) -> BoxResult<Record> {
    let (action, url) = match text
        .trim_start_matches(&['-', ' '])
        .trim()
        .split_once(' ') {
            Some(s) => s,
            None => return Err(error(format!("failed to parse `{text}`")))
        };
    let action = action.parse::<Action>()?;
    let canonical_url = url.rsplit_once('#').map(|x| x.0).unwrap_or(url).into();
    let record = Record {
        date: date.to_string(),
        kind,
        action,
        canonical_url,
        url: url.into(),
    };
    Ok(record)
}

fn main() -> BoxResult<()> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    let (in_path, out_path) = match args.as_slice() {
        [input, output] => (input, output),
        _ => {
            eprintln!("Usage: parge-log <input> <output>");
            process::exit(1);
        }
    };

    dbg!(in_path);

    let input = File::open(in_path).unwrap();
    let mut output = File::create(out_path).unwrap();

    let mut date = String::new();
    let mut kind = Kind::default();

    for (lnr, line) in BufReader::new(input).lines().enumerate() {
        let line = line.unwrap();
        if line.len() < 2 {
            continue;
        }
        let record = match line.split_at(2) {
            ("", _) => continue,
            (prefix, "") if !prefix.is_empty() => return Err(error_lnr(lnr, "")),
            ("##", text) => {
                date = text.trim().to_string();
                continue;
            }
            ("- ", text) => {
                kind = text.trim().parse().map_err(|e| error_lnr(lnr, e))?;
                continue;
            }
            ("  ", text) => parse_item(&date, kind, text).map_err(|e| error_lnr(lnr, e))?,
            _ => return Err(error_lnr(lnr, ""))
        };

        writeln!(output, "{record}").map_err(|e| error_lnr(lnr, e))?;
    }

    dbg!(out_path);
    Ok(())
}
