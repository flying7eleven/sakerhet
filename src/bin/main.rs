use chrono::Local;
use clap::{crate_authors, crate_description, crate_version, load_yaml, App};
use log::{error, LevelFilter};
use sakerhet::configuration::Configuration;
use sha2::{Digest, Sha512};
use std::fs::{read_dir, DirEntry, File};
use std::io::{self, copy};
use std::path::Path;

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn handle_file(dir_entry: &DirEntry) {
    let mut file = File::open(&dir_entry.path()).unwrap();
    let mut hasher = Sha512::new();

    let n = copy(&mut file, &mut hasher).unwrap();
    let hash = hasher.result();

    println!("{} -> {:x}", dir_entry.path().to_str().unwrap(), hash);
}

fn setup_logger(log_level: LevelFilter) {
    let _ = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stdout())
        .apply();
}

fn main() {
    // configure the command line parser
    let configuration_parser_config = load_yaml!("cli.yml");
    let matches = App::from_yaml(configuration_parser_config)
        .author(crate_authors!())
        .version(crate_version!())
        .name("Säkerhet")
        .about(crate_description!())
        .get_matches();

    // read the configuration
    let configuration = Configuration::from_defaut_locations();

    // do not initialize the logger for the config sub-command
    if matches.subcommand_matches("config").is_none() {
        setup_logger(LevelFilter::Debug);
    }

    // check which subcommand should be executed and call it
    if let Some(_) = matches.subcommand_matches("config") {
        println!("{}", serde_yaml::to_string(&configuration).unwrap());
    } else {
        error!("No known subcommand was selected. Please refer to the help for information about how to use this application.");
    }
}
