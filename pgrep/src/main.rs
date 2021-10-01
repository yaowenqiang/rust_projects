use clap::{clap_app, crate_version};
use regex::Regex;
use std::path::Path;
#[derive(Debug)]
struct Record {
    line:usize,
    tx:String,
}

fn process_file<P: AsRef<Path>>(p:P, re: Regex) -> Result<Vec<Record>, String> {
    let mut res = Vec::new();
    let bts = std::fs::read(p).map_err(|e| "could not readd string".to_string())?;
    if let Ok(ss) = String::from_utf8(bts) {
        for (i,l) in ss.lines().enumerate() {
            if re.is_match(l) {
                res.push(Record{
                    line: i,
                    tx: l.to_string(),
                });
            }

        }
    }

    Ok(res)
}
fn main()-> Result<(), String> {
    let cp = clap_app!(
        pgrep => 
        (version: crate_version!())
        (about: "A Grep like program")
        (author: "jack")
        (@arg pattern: +required  "the regex pattern to search for")
        (@arg file: -f --file +takes_value "The file to test")
        )
        .get_matches();

    let re = Regex::new(cp.value_of("pattern").unwrap()).map_err(|_| "bad regex")?;
    let p = process_file(cp.value_of("file").ok_or("No file chosen")?, re);
    println!("{:?}", p);
    Ok(())
}
