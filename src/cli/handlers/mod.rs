//! Module containing handlers which are responsible for logic of the databoxer's subcommand
//! actions

use clap::ArgMatches;
use std::collections::VecDeque;
use std::path::PathBuf;

mod base;
mod profile;
mod key;

pub use base::*;
pub use profile::*;
pub use key::*;

/// Converts from the passed arguments strings to vector of paths
pub fn get_path_vec(args: &ArgMatches, arg_id: &str) -> Option<Vec<PathBuf>> {
    if let Some(strings) = args.get_many::<String>(arg_id) {
        return Some(strings
            .map(|s| PathBuf::from(s))
            .collect::<Vec<PathBuf>>()
        )
    }
    None
}

/// Converts from the passed arguments strings to deque of paths
pub fn get_path_deque(args: &ArgMatches, arg_id: &str) -> Option<VecDeque<PathBuf>> {
    if let Some(strings) = args.get_many::<String>(arg_id) {
        let mut deque = VecDeque::new();

        for s in strings {
            deque.push_back(PathBuf::from(s))
        }

        return Some(deque)
    }
    None
}