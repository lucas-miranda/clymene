use std::cmp::Ordering;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum FrameIndices {
    Value(u32),
    Range { from: u32, to: u32 },
}

impl FrameIndices {
    pub fn is_inside(&self, indices: &FrameIndices) -> bool {
        match self {
            FrameIndices::Value(n) => match indices {
                FrameIndices::Value(other_n) => other_n == n,
                FrameIndices::Range { from, to } => from == n && to == n,
            },
            FrameIndices::Range { from, to } => match indices {
                FrameIndices::Value(other_n) => !(other_n < from || other_n > to),
                FrameIndices::Range {
                    from: other_from,
                    to: other_to,
                } => !(other_from < from || other_to > to),
            },
        }
    }

    pub fn union(&mut self, indices: &FrameIndices) -> Option<()> {
        match *self {
            FrameIndices::Value(v) => match *indices {
                FrameIndices::Value(other_v) => {
                    if !((v > 1 && other_v < v - 1) || other_v > v + 1) {
                        match other_v.cmp(&v) {
                            Ordering::Greater => {
                                *self = FrameIndices::Range {
                                    from: v,
                                    to: other_v,
                                }
                            }
                            Ordering::Less => {
                                *self = FrameIndices::Range {
                                    from: other_v,
                                    to: v,
                                }
                            }
                            Ordering::Equal => (),
                        }

                        return Some(());
                    }
                }
                FrameIndices::Range {
                    from: other_from,
                    to: other_to,
                } => {
                    if !((v > 1 && other_to < v - 1) || other_from > v + 1) {
                        if other_from != v || other_to != v {
                            *self = FrameIndices::Range {
                                from: v.min(other_from),
                                to: v.max(other_to),
                            };
                        }

                        return Some(());
                    }
                }
            },
            FrameIndices::Range { from, to } => {
                match *indices {
                    FrameIndices::Value(other_v) => {
                        if !((from > 1 && other_v < from - 1) || other_v > to + 1) {
                            if other_v != from || other_v != to {
                                *self = FrameIndices::Range {
                                    from: from.min(other_v),
                                    to: to.max(other_v),
                                };
                            }

                            return Some(());
                        }
                    }
                    FrameIndices::Range {
                        from: other_from,
                        to: other_to,
                    } => {
                        if !((from > 1 && other_to < from - 1) || other_from > to + 1) {
                            // union is continuous

                            if from == to && other_from == from && other_to == to {
                                // degenerates to a value
                                *self = FrameIndices::Value(from);
                            } else {
                                *self = FrameIndices::Range {
                                    from: from.min(other_from),
                                    to: to.max(other_to),
                                };
                            }

                            return Some(());
                        }
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_inside() {
        // -> value, value

        // value contains value
        assert!(FrameIndices::Value(5).is_inside(&FrameIndices::Value(5)));

        // value not contains value
        assert!(!FrameIndices::Value(5).is_inside(&FrameIndices::Value(4)));

        // -> value, range

        // value contains range
        assert!(FrameIndices::Value(5).is_inside(&FrameIndices::Range { from: 5, to: 5 }));

        // value not contains range
        assert!(!FrameIndices::Value(5).is_inside(&FrameIndices::Range { from: 3, to: 7 }));

        assert!(!FrameIndices::Value(2).is_inside(&FrameIndices::Range { from: 3, to: 7 }));

        // -> range, value

        // range contains value
        assert!(FrameIndices::Range { from: 3, to: 7 }.is_inside(&FrameIndices::Value(5)));

        // range not contains value
        assert!(!FrameIndices::Range { from: 3, to: 7 }.is_inside(&FrameIndices::Value(10)));

        // -> range, range

        // range contains range
        assert!(FrameIndices::Range { from: 0, to: 10 }
            .is_inside(&FrameIndices::Range { from: 3, to: 7 }));

        // range not contains range
        assert!(!FrameIndices::Range { from: 10, to: 20 }
            .is_inside(&FrameIndices::Range { from: 15, to: 25 }));

        assert!(!FrameIndices::Range { from: 10, to: 20 }
            .is_inside(&FrameIndices::Range { from: 5, to: 25 }));

        assert!(!FrameIndices::Range { from: 10, to: 20 }
            .is_inside(&FrameIndices::Range { from: 1, to: 9 }));
    }

    #[test]
    fn test_union() {
        // -> value, value

        // value union value resulting in value
        {
            let mut v = FrameIndices::Value(5);
            assert!(v.union(&FrameIndices::Value(5)).is_some());
            assert_eq!(v, FrameIndices::Value(5));
        }

        // value union value resulting in range
        {
            let mut v = FrameIndices::Value(5);
            assert!(v.union(&FrameIndices::Value(6)).is_some());
            assert_eq!(v, FrameIndices::Range { from: 5, to: 6 });
        }

        // value union value failing to build a continuous range
        {
            let mut v = FrameIndices::Value(5);
            assert!(v.union(&FrameIndices::Value(7)).is_none());
            assert_eq!(v, FrameIndices::Value(5));
        }

        // -> value, range

        // value union range resulting in value
        {
            let mut v = FrameIndices::Value(8);
            assert!(v.union(&FrameIndices::Range { from: 8, to: 8 }).is_some());
            assert_eq!(v, FrameIndices::Value(8));
        }

        // value union range resulting in range
        {
            let mut v = FrameIndices::Value(12);
            assert!(v.union(&FrameIndices::Range { from: 9, to: 11 }).is_some());
            assert_eq!(v, FrameIndices::Range { from: 9, to: 12 });
        }

        {
            let mut v = FrameIndices::Value(8);
            assert!(v.union(&FrameIndices::Range { from: 4, to: 11 }).is_some());
            assert_eq!(v, FrameIndices::Range { from: 4, to: 11 });
        }

        // value union range failing to build a continuous range
        {
            let mut v = FrameIndices::Value(20);
            assert!(v.union(&FrameIndices::Range { from: 1, to: 5 }).is_none());
            assert_eq!(v, FrameIndices::Value(20));
        }

        // -> range, value
        // same as value, range; but starting with range

        // value union range resulting in value
        {
            let mut v = FrameIndices::Range { from: 8, to: 8 };
            assert!(v.union(&FrameIndices::Value(8)).is_some());
            assert_eq!(v, FrameIndices::Range { from: 8, to: 8 });
        }

        // value union range resulting in range
        {
            let mut v = FrameIndices::Range { from: 9, to: 11 };
            assert!(v.union(&FrameIndices::Value(12)).is_some());
            assert_eq!(v, FrameIndices::Range { from: 9, to: 12 });
        }

        {
            let mut v = FrameIndices::Range { from: 4, to: 11 };
            assert!(v.union(&FrameIndices::Value(8)).is_some());
            assert_eq!(v, FrameIndices::Range { from: 4, to: 11 });
        }

        // value union range failing to build a continuous range
        {
            let mut v = FrameIndices::Range { from: 1, to: 5 };
            assert!(v.union(&FrameIndices::Value(20)).is_none());
            assert_eq!(v, FrameIndices::Range { from: 1, to: 5 });
        }

        // -> range, range

        // range union range resulting in value
        {
            let mut v = FrameIndices::Range { from: 10, to: 10 };
            assert!(v.union(&FrameIndices::Range { from: 10, to: 10 }).is_some());
            assert_eq!(v, FrameIndices::Value(10));
        }

        // range union range resulting in range
        {
            let mut v = FrameIndices::Range { from: 1, to: 10 };
            assert!(v.union(&FrameIndices::Range { from: 11, to: 20 }).is_some());
            assert_eq!(v, FrameIndices::Range { from: 1, to: 20 });
        }

        {
            let mut v = FrameIndices::Range { from: 5, to: 10 };
            assert!(v.union(&FrameIndices::Range { from: 1, to: 8 }).is_some());
            assert_eq!(v, FrameIndices::Range { from: 1, to: 10 });
        }

        {
            let mut v = FrameIndices::Range { from: 5, to: 50 };
            assert!(v.union(&FrameIndices::Range { from: 20, to: 30 }).is_some());
            assert_eq!(v, FrameIndices::Range { from: 5, to: 50 });
        }

        // range union range failing to build a continuous range
        {
            let mut v = FrameIndices::Range { from: 11, to: 21 };
            assert!(v.union(&FrameIndices::Range { from: 3, to: 9 }).is_none());
            assert_eq!(v, FrameIndices::Range { from: 11, to: 21 });
        }
    }
}
