use tracing::info;

use crate::blocklist::Blocklist;

/// The in-memory storage for the application which holds a vector of IP addresses in a sorted way
#[derive(Default, Debug)]
pub struct Store {
	/// The Vector of Strings which represent IP addresses
	/// ["192.168.178.1", "202.100.100.2", "3.12.14.2", ...]
	blocklist: Blocklist,
}

impl Store {
	/// Updating the in-memory storage.
	///
	/// This overrides the existing list completely, and sorts in an ascending way.
	pub fn update(&mut self, blocklist: Blocklist) {
		info!("Update in-memory blocklist store");
		self.blocklist = blocklist;
		self.blocklist.0.sort_unstable();
	}

	/// Checking if a given IP address is part of the blocklist. We use binary search to do so quickly.
	pub fn contains(&mut self, ip: &str) -> bool {
		self.blocklist.0.binary_search(&ip.to_string()).is_ok()
	}
}
