use super::CacheEntry;
use std::cell::Ref;

pub enum CacheStatus<'a> {
    Found(Ref<'a, CacheEntry>),
    NotFound,
    Outdated,
}
