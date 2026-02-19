use anyhow::Result;
use notify::RecursiveMode;
use notify_debouncer_full::{new_debouncer, DebounceEventResult, Debouncer, FileIdMap};
use std::path::Path;
use std::sync::mpsc::{channel, Receiver};
use std::time::Duration;

pub struct FileWatcher {
    _debouncer: Debouncer<notify::RecommendedWatcher, FileIdMap>,
    rx: Receiver<DebounceEventResult>,
}

impl FileWatcher {
    pub fn new(beads_dir: &Path) -> Result<Self> {
        let (tx, rx) = channel();

        let mut debouncer = new_debouncer(Duration::from_millis(100), None, move |result| {
            let _ = tx.send(result);
        })?;

        debouncer.watch(beads_dir, RecursiveMode::NonRecursive)?;

        Ok(FileWatcher {
            _debouncer: debouncer,
            rx,
        })
    }

    pub fn poll(&self) -> Option<()> {
        match self.rx.try_recv() {
            Ok(Ok(events)) => {
                for event in events {
                    for path in &event.paths {
                        if let Some(filename) = path.file_name() {
                            if filename == "last-touched" {
                                return Some(());
                            }
                        }
                    }
                }
                None
            }
            _ => None,
        }
    }
}
