use anyhow::Result;
use notify::{EventKind, RecursiveMode, Watcher};
use notify_debouncer_full::{new_debouncer, DebounceEventResult, Debouncer, FileIdMap};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

pub struct FileWatcher {
    _debouncer: Debouncer<notify::RecommendedWatcher, FileIdMap>,
    rx: Receiver<DebounceEventResult>,
}

impl FileWatcher {
    pub fn new(db_path: &Path) -> Result<Self> {
        let (tx, rx) = channel();

        let mut debouncer = new_debouncer(Duration::from_millis(100), None, move |result| {
            let _ = tx.send(result);
        })?;

        // Watch the database file and WAL file
        debouncer.watch(db_path, RecursiveMode::NonRecursive)?;

        // Also watch WAL file if it exists
        let wal_path = db_path.with_extension("db-wal");
        if wal_path.exists() {
            debouncer.watch(&wal_path, RecursiveMode::NonRecursive)?;
        }

        Ok(FileWatcher {
            _debouncer: debouncer,
            rx,
        })
    }

    pub fn poll(&self) -> Option<()> {
        match self.rx.try_recv() {
            Ok(Ok(events)) => {
                // Check if any relevant events occurred
                for event in events {
                    match event.kind {
                        EventKind::Modify(_) | EventKind::Create(_) => {
                            return Some(());
                        }
                        _ => {}
                    }
                }
                None
            }
            _ => None,
        }
    }
}
