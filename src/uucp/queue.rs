
//! All the queue-related stuff is here
//!

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::uucp::{Entity, State};

#[derive(Debug, Default)]
pub struct Queue {
    /// The base directory which is `UUCP_BASE/sitename`
    pub path: PathBuf,
    /// The queue itself, key is `qid`
    pub queue: HashMap<String, Entity>,
    /// Filename cache for reloads (from `C.`)
    pub fcache: HashSet<PathBuf>,
}

impl Queue {
    /// Create a new empty queue
    ///
    pub fn new() -> Self {
        Queue {
            path: PathBuf::from(""),
            queue: HashMap::new(),
            fcache: HashSet::new(),
        }
    }

    /// Queue length
    ///
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Check sanity of the UUCP queue, find which C. has no D. and such.
    ///
    pub fn check(&self) -> State {
        State::Clean { nfiles: self.queue.len() }
    }

    /// Help func to return stats about mails
    ///
    pub fn mails(&self) -> usize {
        self.queue.iter().filter(|(_n, e)| match e {
            Entity::Mail { .. } => true,
            _ => false,
        }).count()
    }

    /// Help func to return stats about news batches
    ///
    pub fn news(&self) -> usize {
        self.queue.iter().filter(|(_n, e)| match e {
            Entity::News { .. } => true,
            _ => false,
        }).count()
    }

    /// Help func to return stats about missing/damaged batches
    ///
    pub fn missing(&self) -> usize {
        self.queue.iter().filter(|(_n, e)| match e {
            Entity::Missing { .. } => true,
            _ => false,
        }).count()
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_new() {
        let q = Queue::new();
        assert_eq!(PathBuf::from(""), q.path);
        assert!(q.queue.is_empty());
        assert!(q.fcache.is_empty());
    }

    #[test]
    fn test_queue_len() {
        let q = Queue::new();
        assert_eq!(0, q.len());
    }

    #[test]
    fn test_stats() {
        let q = Queue::new();

        assert_eq!(0, q.mails());
        assert_eq!(0, q.missing());
        assert_eq!(0, q.news());
    }
}
