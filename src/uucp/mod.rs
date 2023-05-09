//! Main UUCP related library
//!

mod queue;

use std::path::PathBuf;

use crate::makepath;
use crate::uucp::queue::Queue;

//#[cfg(unix)]
const UUCP_BASE: &str = "/var/spool/uucp";

/// Describes a UUCP site
///
#[derive(Debug)]
pub struct Site {
    /// UUCP name
    pub name: String,
    /// The queue itself
    q: Queue,
    /// Is this a valid site?
    valid: bool,
}

/// Stats about a given site
///
#[derive(Default, Debug)]
pub struct Stats {
    /// Total number of files
    pub nfiles: usize,
    /// Size of the directory in bytes
    pub nbytes: u64,
}

impl Site {
    /// Create a new site, valid will be true if the site directory exist & contains both
    /// `D.` and `C.` directories
    ///
    pub fn new(site: &str) -> Self {
        let fullpath: PathBuf = makepath!(UUCP_BASE, site);
        let cpath: PathBuf = makepath!(UUCP_BASE, site, "C.");
        let dpath: PathBuf = makepath!(UUCP_BASE, site, "D.");
        Site {
            name: site.to_string(),
            q: Queue::new(),
            valid: fullpath.exists() && cpath.exists() && dpath.exists(),
        }
    }

    /// Scan the given directory and create queue entries for each identified batch
    ///
    pub fn scan(&mut self) -> &mut Self {
        self
    }

    /// Return validity of a given site
    ///
    pub fn is_valid(&self) -> bool {
        self.valid
    }

    /// Return a reference to the queue
    ///
    pub fn queue(&self) -> &Queue {
        &self.q
    }
}

#[derive(Default, Debug)]
pub enum State {
    Clean { nfiles: usize },
    Damaged { dmissing: usize, cmissing: usize },
    #[default]
    Empty,
}

#[derive(Default, Debug)]
pub enum Entity {
    /// Damaged batch, `C.` without corresponding `D.` file
    Missing { qid: String, file: Batch, reason: String },
    /// Mail-related batch
    Mail { qid: String, file: Batch, marked: bool },
    /// News-related batch, not going to do much with these
    News { qid: String, file: Batch, marked: bool },
    /// Invalid entry
    #[default]
    Invalid,
}

#[derive(Default, Debug)]
pub struct Batch {
    /// Control file in the `C.` directory
    pub c: Option<PathBuf>,
    /// Data file in the `D.` directory, there MUST be a `C.` file for every `D.` one
    pub d: Option<PathBuf>,
}

