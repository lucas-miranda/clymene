use std::cell::Ref;
use super::CacheEntry;

pub enum CacheStatus<'a> {
    Found(Ref<'a, CacheEntry>),
    NotFound,
    Outdated
}
