//! Options parsing module used to configure application state.
//!
//! Nothing particularly important to see here, just typical
//! parsing of things like command line arguments into something
//! more easily used internally (from the main application flow).
use crate::filters::FilterKind;
use clap::{Arg, ArgEnum, Command};
use std::ffi::OsString;

/// Options struct to store configuration state.
///
/// The options struct will basically contain anything relevant
/// to the execution of the application; things such as inputs
/// and flags to dictate behaviour will be stored here. It acts
/// (in essence) as application configuration.
#[derive(Clone, Debug)]
pub struct Options {
    pub filter: FilterKind,
    pub inputs: Vec<String>,
    pub inverted: bool,
    pub statistics: bool,
}

impl Options {
    /// Creates an `Options` struct from an iterable set of arguments.
    ///
    /// Arguments can be any kind of iterator, as long as they can be
    /// successfully cloned and parsed into an instance of `OsString`.
    pub fn from<I, T>(args: I) -> Options
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        // create a new parser for our args
        let parser = Options::create_parser();

        // parse out the arguments into matching opts
        let options = parser.get_matches_from(args);

        // attempt to parse the provided filter
        let filter = options.value_of_t::<FilterKind>("filter");

        // create opts
        Options {
            // grab and store statistics flags
            statistics: options.is_present("statistics"),

            // grab and store inversion flags
            inverted: options.is_present("invert"),

            // store the filter to use for unique detection
            filter: filter.unwrap_or(FilterKind::Digest),

            // own all inputs
            inputs: options
                .values_of("inputs")
                .unwrap()
                .map(|s| s.to_owned())
                .collect(),
        }
    }

    /// Creates a parser used to generate `Options`.
    ///
    /// All command line usage information can be found in the definitions
    /// below, and follows the API of the `clap` library.
    ///
    /// In terms of visibility, this method is defined on the struct due to
    /// the parser being specifically designed around the `Options` struct.
    fn create_parser<'a>() -> Command<'a> {
        Command::new("")
            // package metadata from cargo
            .name(env!("CARGO_PKG_NAME"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .version(env!("CARGO_PKG_VERSION"))
            // arguments and flag details
            .args(&[
                // filter: -f, --filter [naive]
                Arg::new("filter")
                    .help("Filter to use to determine uniqueness")
                    .short('f')
                    .long("filter")
                    .takes_value(true)
                    .possible_values(
                        FilterKind::value_variants()
                            .iter()
                            .filter_map(ArgEnum::to_possible_value),
                    )
                    .hide_default_value(true)
                    .ignore_case(true),
                // inputs: +required +multiple
                Arg::new("inputs")
                    .help("Input sources to filter")
                    .multiple_occurrences(true)
                    .required(true),
                // invert: -i --invert
                Arg::new("invert")
                    .help("Prints duplicates instead of uniques")
                    .short('i')
                    .long("invert"),
                // statistics: -s --statistics
                Arg::new("statistics")
                    .help("Prints statistics instead of entries")
                    .short('s')
                    .long("statistics"),
            ])
            // settings required for parsing
            .arg_required_else_help(true)
            .hide_possible_values(true)
            .trailing_var_arg(true)
    }
}
