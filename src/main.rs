extern crate regex;
extern crate reqwest;

use failure::Error;
use pretty_env_logger;
use regex::Regex;
use std::borrow::Cow;
use std::fs::File;
use std::io::copy;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(raw(setting = "structopt::clap::AppSettings::ColoredHelp"))]
struct Opt {
    #[structopt(short = "p", default_value = "emoji")]
    file_prefix: String,

    download_url: String,
}

pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    pretty_env_logger::init();
    let opt = Opt::from_args();
    let id = extract_id(&opt.download_url).unwrap();

    // download emoji 001-040
    for i in 1..41 {
        let link = generate_link(&id, i);
        let _ = download(&link, &opt.file_prefix);
    }
    Ok(())
}

fn extract_id(url: &str) -> Result<Cow<'_, str>> {
    let re = Regex::new(r"^https://store.line.me/emojishop/product/(?P<id>[a-z0-9]+)(.*)")?;
    assert!(re.is_match(url), "Invalid url: {}", url);
    Ok(re.replace_all(url, "$id"))
}

fn generate_link(id: &Cow<'_, str>, index: i64) -> String {
    return format!(
        "https://stickershop.line-scdn.net/sticonshop/v1/sticon/{}/iPhone/{}.png",
        id.to_owned(),
        format!("{:03}", index)
    );
}

fn download(link: &str, prefix: &str) -> Result<()> {
    let splited: Vec<&str> = link.split('/').collect();
    let fname = format!("{}_{}", prefix, splited.last().unwrap());
    let mut res = reqwest::get(link)?;
    println!("downloaded: {}", fname);
    let mut out = File::create(fname)?;
    copy(&mut res, &mut out)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::extract_id;
    use crate::generate_link;
    use std::borrow::Cow;

    #[test]
    fn extract_id_success() {
        let expected = "5ba9e5ff040ab16e95045dac";
        let input = "https://store.line.me/emojishop/product/5ba9e5ff040ab16e95045dac/ja";
        let actual = extract_id(&input).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic(expected = "Invalid url: https://www.google.com")]
    fn extract_id_fail_for_invalid_url() {
        let input = "https://www.google.com";
        let _ = extract_id(&input);
    }

    #[test]
    fn generate_link_success() {
        let expected = "https://stickershop.line-scdn.net/sticonshop/v1/sticon/5ba9e5ff040ab16e95045dac/iPhone/001.png";
        let actual = generate_link(&Cow::from("5ba9e5ff040ab16e95045dac"), 1);
        assert_eq!(expected, actual);
    }
}
