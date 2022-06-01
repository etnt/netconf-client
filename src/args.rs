use clap::{Arg, Command};
use log::LevelFilter;
use std::env;

#[derive(Clone)]
pub struct Args {
    pub infile: String,
    pub outfile: String,
    pub silent: bool,
    pub host: String,
    pub port: String,
    pub user: String,
    pub passwd: String,
    pub log_level: LevelFilter,
    pub get_config: bool,
}

impl Args {
    pub fn parse() -> Self {
        let matches = Command::new("mypipe")
            .arg(Arg::new("infile").help("Read from a file instead of stdin."))
            .arg(
                Arg::new("outfile")
                    .short('o')
                    .long("outfile")
                    .takes_value(true)
                    .help("Write to a file instead of stdout."),
            )
            .arg(
                Arg::new("silent")
                    .short('s')
                    .long("silent")
                    .help("Produce no output."),
            )
            .arg(
                Arg::new("host")
                    .short('h')
                    .long("host")
                    .default_value("127.0.0.1")
                    .help("server Host IP address"),
            )
            .arg(
                Arg::new("port")
                    .short('p')
                    .long("port")
                    .default_value("2022")
                    .help("server Host TCP port"),
            )
            .arg(
                Arg::new("user")
                    .short('u')
                    .long("user")
                    .default_value("admin")
                    .help("user id"),
            )
            .arg(
                Arg::new("passwd")
                    .short('w')
                    .long("passwd")
                    .default_value("admin")
                    .help("user password"),
            )
            .arg(
                Arg::new("get-config")
                    .long("get-config")
                    .help("Get all config in the running datastore."),
            )
            .arg(
                Arg::new("debug")
                    .long("debug")
                    .help("turn on debug logging"),
            )
            .get_matches();
        let infile = matches.value_of("infile").unwrap_or_default().to_string();
        let outfile = matches.value_of("outfile").unwrap_or_default().to_string();
        let silent = if matches.is_present("silent") {
            true
        } else {
            !env::var("SILENT").unwrap_or_default().is_empty()
        };
        let host = matches.value_of("host").unwrap_or_default().to_string();
        let port = matches.value_of("port").unwrap_or_default().to_string();
        let user = matches.value_of("user").unwrap_or_default().to_string();
        let passwd = matches.value_of("passwd").unwrap_or_default().to_string();
        let log_level = if matches.is_present("debug") {
            LevelFilter::Debug
        } else {
            LevelFilter::Off
        };
        let get_config = matches.is_present("get-config");
        Self {
            infile,
            outfile,
            silent,
            host,
            port,
            user,
            passwd,
            log_level,
            get_config,
        }
    }
}
