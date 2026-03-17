// ============================================================
//  src/party.rs — A participant in the MPC protocol
// ============================================================
//
// In a REAL multi-party computation system each party:
//   • Has their own private secret  (salary, age, PIN, …)
//   • Receives one share of every other party's secret
//     via an encrypted, authenticated private channel.
//   • Performs LOCAL computations on those shares (no communication).
//   • Broadcasts masked values during multiplication rounds.
//
// In our SIMULATION we model all parties on one machine, but we
// carefully separate what each party knows.  The `shares` HashMap
// represents the private memory of one party — other parties'
// code cannot read it without going through the protocol functions.

use std::collections::HashMap;
use crate::shamir::Share;

/// One participant in the MPC computation.
pub struct Party {
    /// 1-indexed identifier.  Also used as the x-coordinate of every
    /// share this party holds  (share.x == party.id).
    pub id: usize,

    /// Human-readable name for display output.
    pub name: String,

    /// Private storage: label → share value (y-coordinate only).
    /// The x-coordinate is always `self.id`, so we only store y.
    shares: HashMap<String, i128>,
}

impl Party {
    /// Create a new party.
    pub fn new(id: usize, name: &str) -> Self {
        Party { id, name: name.to_string(), shares: HashMap::new() }
    }

    /// Store a share for a named value (called by the protocol layer
    /// when distributing shares).
    pub fn receive_share(&mut self, label: &str, y: i128) {
        self.shares.insert(label.to_string(), y);
    }

    /// Read the y-value of a named share from private storage.
    pub fn get_value(&self, label: &str) -> i128 {
        *self.shares.get(label)
            .unwrap_or_else(|| panic!("Party '{}' has no share for '{}'", self.name, label))
    }

    /// Return the full Share struct  (x = party id, y = stored value).
    pub fn get_share(&self, label: &str) -> Share {
        Share { x: self.id as i128, y: self.get_value(label) }
    }
}
