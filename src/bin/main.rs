use chrono::Local;
use clap::{crate_authors, crate_description, crate_version, load_yaml, App};
use log::{error, LevelFilter};
use sakerhet::configuration::Configuration;
use sakerhet::subcommands::rebuild::run_subcommand as run_rebuild;

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
    } else if let Some(_) = matches.subcommand_matches("rebuild") {
        run_rebuild(&configuration);
    } else {
        error!("No known sub-command was selected. Please refer to the help for information about how to use this application.");
    }
}
