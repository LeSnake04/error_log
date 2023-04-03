use crate::{Entries, Entry, EntryContent, ErrorLog};
use alloc::vec::Vec;

impl<T, E> ErrorLog<T, E> {
    /// Get immmutable reference to Vector of [`Entries`]
    /// NOTE: Does not filter entries
    pub fn entries(&self) -> &Entries<E> {
        &self.entries
    }
    /// Get mutable reference to Vector of [`Entries`]
    /// NOTE: Does not filter entries lower than maximum [`LevelFilter`][crate::LevelFilter]
    pub fn entries_mut(&mut self) -> &mut Entries<E> {
        &mut self.entries
    }
    /// Get owned [`Entries`], Removing them from Instace
    pub fn entries_owned(&mut self) -> Entries<E> {
        let mut out = Vec::new();
        out.append(&mut self.entries);
        self.filter_entries(&mut out);
        out
    }
    /// Filter out entries below the max_level
    fn filter_entries(&self, entries: &mut Entries<E>) {
        entries.retain(|e| {
            !matches!(e.content,
                EntryContent::Message { level, .. } if level > self.max_level
            )
        });
    }
    /// Get cloned vector of messages. Filters out errors.
    pub fn messages_clone(&self) -> Entries<E> {
        let mut out = Entries::new();
        for ent in &self.entries {
            if let EntryContent::Message { level, message } = &ent.content {
                if level <= &self.max_level {
                    out.push(Entry::new_message(*level, message.clone()))
                }
            }
        }
        out
    }
    /// Get owned log messages as [`Entries`], firing out all
    pub fn messages_owned(mut self) -> Entries<E> {
        let mut out = Entries::new();
        let mut entries = self.entries_owned();
        for i in entries.len()..0 {
            if let EntryContent::Message { level, message } = entries.remove(i).content {
                if level <= self.max_level {
                    out.push(Entry::new_message(level, message))
                }
            }
        }
        out
    }
}

impl<T, E: Clone> ErrorLog<T, E> {
    /// Clone Entries.
    /// Filters messages lower than the [`max_level`][Self::max_level]
    pub fn entries_cloned(&self) -> Entries<E> {
        let mut out = self.entries.clone();
        self.filter_entries(&mut out);
        out
    }
    /// Get cloned errors as vector. Filters out all Messages lower than the [`max_level`][Self::max_level].
    pub fn errors_cloned(&self) -> Vec<E> {
        let mut out = Vec::new();
        for ent in &self.entries {
            if let EntryContent::Error(err) = &ent.content {
                out.push(err.clone())
            }
        }
        out
    }
}
