use crate::{Entries, Entry, ErrorLog};

impl<T, E> ErrorLog<T, E> {
    /// Get mutable reference to Vector of [Entries]
    /// NOTE: Does not filter entries lower than maximum [LevelFilter][crate::LevelFilter]
    pub fn emtries_mut(&mut self) -> &mut Entries<E> {
        &mut self.entries
    }
    /// Get immmutable reference to Vector of [Entries]
    /// NOTE: Does not filter entries
    pub fn entries(&self) -> &Entries<E> {
        &self.entries
    }
    /// Get owned [Entries], Removing them from Instace
    pub fn entries_owned(&mut self) -> Entries<E> {
        let mut out = vec![];
        out.append(&mut self.entries);
        self.filter_entries(&mut out);
        out
    }
    fn filter_entries(&self, entries: &mut Entries<E>) {
        entries.retain(|e| {
            !matches!(e,
                Entry::Message { level, .. } if level > self.max_level()
            )
        });
    }
    /// Get cloned vector of messages. Filters out errors.
    pub fn messages_clone(&self) -> Entries<E> {
        let mut out = Entries::new();
        for ent in &self.entries {
            if let Entry::Message { level, message } = ent {
                if level <= &self.max_level {
                    out.push(Entry::Message {
                        level: *level,
                        message: message.clone(),
                    })
                }
            }
        }
        out
    }
    /// Get owned log messages as [Entries], firing out all
    pub fn messages_owned(mut self) -> Entries<E> {
        let mut out = Entries::new();
        let mut entries = self.entries_owned();
        for i in entries.len()..0 {
            if let Entry::Message { level, message } = entries.remove(i) {
                if level <= self.max_level {
                    out.push(Entry::Message { level, message })
                }
            }
        }
        out
    }
}

impl<T, E: Clone> ErrorLog<T, E> {
    /// Clone Entries.
    /// Filters messages based of [set_max_level()][Self::set_max_level]
    pub fn entries_cloned(&self) -> Entries<E> {
        let mut out = self.entries.clone();
        self.filter_entries(&mut out);
        out
    }
    /// Get cloned errors as vector. Filters out all messages
    pub fn errors_cloned(&self) -> Vec<E> {
        let mut out = Vec::new();
        for ent in &self.entries {
            if let Entry::Error(err) = ent {
                out.push(err.clone())
            }
        }
        out
    }
}
