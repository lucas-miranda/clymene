use super::Track;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(transparent)]
pub struct TrackList {
    entries: Vec<Track>,
}

impl TrackList {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn entries(&self) -> &Vec<Track> {
        &self.entries
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn register(&mut self, mut track: Track) {
        // insert any inner Track, which is contained, to the new track

        // collect all track indices which is contained by new track
        let indices: Vec<usize> = self
            .entries
            .iter()
            .enumerate()
            .filter_map(|(i, t)| track.indices().contains(t.indices()).then(|| i))
            .collect();

        // remove items by index using reverse order
        // and register to new track
        indices
            .iter()
            .rev()
            .map(|i| self.entries.remove(*i))
            .rev()
            .for_each(|t| track.tracks.register(t));

        // try insert to the first inner track which contains it
        for t in &mut self.entries {
            if t.indices().contains(track.indices()) {
                t.tracks.register(track);
                return;
            }
        }

        self.entries.push(track);
    }
}
