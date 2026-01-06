use crate::core::events::Event;
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Persister {
    pub events_path: PathBuf,
    #[allow(dead_code)]
    pub snapshot_path: PathBuf,
}

impl Persister {
    pub fn new(events_path: PathBuf, snapshot_path: PathBuf) -> Self {
        Self {
            events_path,
            snapshot_path,
        }
    }

    pub fn append_event(&self, event: &Event) -> std::io::Result<()> {
        let mut f = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.events_path)?;
        let line = serde_json::to_string(event)?;
        f.write_all(line.as_bytes())?;
        f.write_all(b"\n")?;
        Ok(())
    }

    pub fn read_events(&self) -> std::io::Result<Vec<Event>> {
        if !self.events_path.exists() {
            return Ok(Vec::new());
        }
        let f = File::open(&self.events_path)?;
        let reader = BufReader::new(f);
        let mut out = Vec::new();
        for line in reader.lines() {
            let l = line?;
            let ev: Event = serde_json::from_str(&l)?;
            out.push(ev);
        }
        Ok(out)
    }

    #[allow(dead_code)]
    pub fn save_snapshot<T: Serialize>(&self, snapshot: &T) -> std::io::Result<()> {
        let mut f = File::create(&self.snapshot_path)?;
        let s = serde_json::to_string(snapshot)?;
        f.write_all(s.as_bytes())?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn load_snapshot<T: for<'de> Deserialize<'de>>(&self) -> std::io::Result<Option<T>> {
        if !self.snapshot_path.exists() {
            return Ok(None);
        }
        let f = File::open(&self.snapshot_path)?;
        let reader = BufReader::new(f);
        let mut s = String::new();
        for line in reader.lines() {
            s.push_str(&line?);
        }
        let v = serde_json::from_str(&s)?;
        Ok(Some(v))
    }
}
