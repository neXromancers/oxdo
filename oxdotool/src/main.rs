use clap::{Arg, App, SubCommand, AppSettings, ArgMatches};
use std::time::Duration;
use std::str::FromStr;
use std::num::ParseIntError;

fn main() {
    let matches = App::new("oxdotool")
        .version("0.1.0")
        .about("Emulates inputs in X")
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)

        .subcommand(SubCommand::with_name("type")
            .version("0.1.0")
            .about("sends keyboard input")
            .arg(Arg::with_name("text")
                .required(true)
                .help("Text to type out"))
            .arg(Arg::with_name("window")
                .short("w")
                .long("window")
                .help("X window to send input to")
                .takes_value(true))
            .arg(Arg::with_name("delay")
                .short("d")
                .long("delay")
                .help("Delay between each keystroke in milliseconds")
                .takes_value(true)))

        .get_matches();

    let (cmd, args) = matches.subcommand();

    let result = match cmd {
        "type" => type_command(args.unwrap()),

        _ => unimplemented!("What"),
    };

    if let Err(e) = result {
        e.exit();
    }
}

fn clap_parse_int<I: FromStr<Err=ParseIntError>>(input: &str) -> Result<I, clap::Error> {
    I::from_str(input).map_err(|e| {
        clap::Error::with_description(&e.to_string(), clap::ErrorKind::ValueValidation)
    })
}

fn clap_parse_optional_int<I: FromStr<Err=ParseIntError>>(input: Option<&str>) -> Result<Option<I>, clap::Error> {
    if let Some(a) = input {
        clap_parse_int::<I>(a).map(Some)
    } else {
        Ok(None)
    }
}

fn clap_parse_default_int<I: FromStr<Err=ParseIntError>>(input: Option<&str>, default: I) -> Result<I, clap::Error> {
    clap_parse_optional_int(input).map(|o|o.unwrap_or(default))
}

fn type_command(args: &ArgMatches) -> Result<(), clap::Error> {
    let text = args.value_of("text").unwrap();
    let window: u32 = clap_parse_default_int(args.value_of("window"), 0)?;
    let delay: u64 = clap_parse_default_int(args.value_of("delay"), 0)?;

    let ox = oxdo::OxDo::new(None);
    ox.enter_text_window(window, text, Duration::from_millis(delay));

    Ok(())
}
