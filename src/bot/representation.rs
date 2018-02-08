use std::fmt;

use accounting::Entry;

pub struct EntryRepresentation(Entry);

impl From<Entry> for EntryRepresentation {
    fn from(entry: Entry) -> EntryRepresentation {
        EntryRepresentation(entry)
    }
}

impl fmt::Display for EntryRepresentation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {} {})", &self.0.product, self.0.price, self.0.time.format("%Y-%m-%d %H:%M").to_string())
    }
}
