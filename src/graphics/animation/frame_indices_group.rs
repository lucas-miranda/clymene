use super::FrameIndices;
use serde::{Deserialize, Serialize};
use std::iter::IntoIterator;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(transparent)]
pub struct FrameIndicesGroup {
    values: Vec<FrameIndices>,
}

impl FrameIndicesGroup {
    pub fn with(index: u32) -> Self {
        let mut indices = Self::default();
        indices.insert(FrameIndices::Value(index));

        indices
    }

    pub fn with_range(from: u32, to: u32) -> Self {
        if from == to {
            return Self::with(from);
        }

        let mut indices = Self::default();
        indices.insert(FrameIndices::Range { from, to });

        indices
    }

    /// Checks if completely contains another FrameIndicesGroup
    pub fn contains(&self, indices: &FrameIndicesGroup) -> bool {
        indices
            .into_iter()
            .all(|i| self.into_iter().any(|e| e.is_inside(i)))
    }

    pub fn insert(&mut self, indices: FrameIndices) {
        let pos = {
            if self.values.is_empty() {
                0usize
            } else {
                let mut spot_index = self.values.len();

                for (i, entry) in self.values.iter_mut().enumerate().rev() {
                    if entry.union(&indices).is_some() {
                        // `from`, `to` joined `entry`
                        self.validate_union_neighbors(i);
                        return;
                    }

                    let found_spot = match *entry {
                        FrameIndices::Value(v) => match indices {
                            FrameIndices::Value(other_v) => other_v < v,
                            FrameIndices::Range { to: other_to, .. } => other_to < v,
                        },
                        FrameIndices::Range { from, .. } => match indices {
                            FrameIndices::Value(other_v) => other_v < from,
                            FrameIndices::Range { to: other_to, .. } => other_to < from,
                        },
                    };

                    if found_spot {
                        spot_index = i;
                        break;
                    }
                }

                spot_index
            }
        };

        self.values.insert(pos, indices)
    }

    /// An union happened, when inserting an element, and it's neighbors must be validate to ensure there is no overlapping.
    fn validate_union_neighbors(&mut self, mut union_index: usize) {
        if union_index > 0 {
            let left_side_element = self.values.get(union_index - 1).unwrap().clone();
            let union_element = self.values.get_mut(union_index).unwrap();

            if union_element.union(&left_side_element).is_some() {
                self.values.remove(union_index - 1);
                union_index -= 1;

                // move recursively to the left until there is no available union
                self.validate_union_neighbors(union_index);
            }
        }

        if union_index < self.values.len() - 1 {
            let right_side_element = self.values.get(union_index + 1).unwrap().clone();
            let union_element = self.values.get_mut(union_index).unwrap();

            if union_element.union(&right_side_element).is_some() {
                self.values.remove(union_index + 1);

                // move recursively to the right until there is no available union
                self.validate_union_neighbors(union_index);
            }
        }
    }
}

impl<'a> IntoIterator for &'a FrameIndicesGroup {
    type Item = <std::slice::Iter<'a, FrameIndices> as Iterator>::Item;
    type IntoIter = std::slice::Iter<'a, FrameIndices>;

    fn into_iter(self) -> Self::IntoIter {
        self.values.as_slice().iter()
    }
}
