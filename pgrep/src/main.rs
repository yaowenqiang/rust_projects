use clap::{clap_app, crate_version};
use regex::Regex;
use std::path::Path;
use std::fmt;
use failure::{Error, Fail};
#[derive(Debug)]
struct Record {
    line:usize,
    tx:String,
}

#[derive(Debug)]
struct ArgErr {
    arg: &'static str,
}
impl Fail for ArgErr {

}

impl fmt::Display for ArgErr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Argument Not Provided{}", self.arg)
    }
}



fn process_file<P: AsRef<Path>>(p:P, re: Regex) -> Result<Vec<Record>, Error> {
    let mut res = Vec::new();
    //let bts = std::fs::read(p).map_err(|e| "could not readd string".to_string())?;
    let bts = std::fs::read(p)?;
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
fn main()-> Result<(), Error> {
    let cp = clap_app!(
        pgrep => 
        (version: crate_version!())
        (about: "A Grep like program")
        (author: "jack")
        (@arg pattern: +required  "the regex pattern to search for")
        (@arg file: -f --file +takes_value "The file to test")
        )
        .get_matches();

    let re = Regex::new(cp.value_of("pattern").unwrap())?;
    let p = process_file(cp.value_of("file").ok_or(ArgErr {arg: "file"})?, re);
    println!("{:?}", p);
    Ok(())
}
