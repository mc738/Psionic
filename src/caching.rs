use std::collections::HashMap;
use std::sync::Arc;

pub struct StaticCache {
    cache: HashMap<String, StaticCacheItem>     
}

pub struct StaticCacheItem {
    data: Vec<u8>
}

impl StaticCache {
    
    /// Attempt to get an item from the cache.
    /// If found, the data will be cloned and returned as owned.
    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        match self.cache.get(key) {
            Some(i) => Some(i.get()),
            None => None
        }
    }
}

impl StaticCacheItem {
    pub fn get(&self) -> Vec<u8> {
        self.data.clone()
    }
}



