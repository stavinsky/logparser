//use std::sync::{Arc, RwLock};
//use std::collections::HashMap;
//pub struct Cache<K,V>
//{
//    cache: Arc<RwLock<HashMap<K,V>>>,
//    new_value: Box<Fn(&K) -> V + Send + Sync>,
//}
//impl<K,V> Cache<K,V>
//where K: std::cmp::Eq+ std::hash::Hash,
//      V: std::clone::Clone
//{
//    pub fn new(f: Box<Fn(&K)->V + Send + Sync>)->Self {
//        Cache {
//            cache: Arc::new(RwLock::new(HashMap::new())),
//            new_value: f,
//        }
//    }
//    pub fn get(&self, k: K) -> V {
//        let mut v = None;
//        {
//            let cache = self.cache.read().unwrap();
//            v = cache.get(&k).map(|v| v.to_owned());
//        }
//        match v {
//            None => {
//                let mut cache = self.cache.write().unwrap();
//                let v = (self.new_value)(&k);
//                cache.insert(k, v.clone());
//                v.to_owned()
//            },
//            Some(v) => {v},
//        }
//    }
//}
use std::sync::{Arc};
use std::rc::Rc;
use parking_lot::RwLock;
use std::collections::HashMap;
pub struct Cache<K,V>
{
    cache: Arc<RwLock<HashMap<K,Arc<V>>>>,
    new_value: Box<Fn(&K) -> V + Send + Sync>,
}
impl<K,V> Cache<K,V>
where K: std::cmp::Eq+ std::hash::Hash + std::clone::Clone,
      V: std::clone::Clone ,
{
    pub fn new(f: Box<Fn(&K)->V + Send + Sync>)->Self {
        Cache {
            cache: Arc::new(RwLock::new(HashMap::new())),
            new_value: f,
        }
    }
    pub fn get(&self, k: &K) -> Arc<V> {
        let mut v = None;
        {
            let cache = self.cache.read();
            v = cache.get(&k).map(|v| v.clone());
        }
        match v {
            None => {
                let mut cache = self.cache.write();
                let v = Arc::new((self.new_value)(&k));
                cache.insert(k.to_owned(), v.clone());
                v
            },
            Some(v) => {v},
        }
    }
}
