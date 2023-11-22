use std::{
    vec::Vec,
    hash::{
        Hash,
        Hasher,
    },
    collections::{
        LinkedList,
        hash_map::DefaultHasher,
    }
};
use criterion::{criterion_group, criterion_main, Criterion};

/*fn main() {
    let mut vecvec: Vec<Vec<u32>> = Vec::new();
    let mut innervec: Vec<u32> = Vec::new();
}*/

fn collision_add_list(c: &mut Criterion) {
    let mut group = c.benchmark_group("Collision_Add_List");
    let num_buckets = 10;
    let num_items = 100;
    
    group.bench_function("custom_linked_list", |b| {
        let mut map = CustomListMap::<u32, u32>::new(num_buckets);

        b.iter(|| {
            for i in 0..num_items {
                map.insert(i % 5, i);
            }
        });
    });

    group.finish();
}

fn collision_add_vec(c: &mut Criterion) {
    
}

fn collision_add_std(c: &mut Criterion) {
    
}

criterion_group!(
    collision_add_benchmark,
    collision_add_list,
    collision_add_vec,
    collision_add_std,
);

criterion_main!(collision_add_benchmark);

struct Entry<K, V> {
    key: K,
    value: V,
}

struct CustomListMap<K, V> {
    buckets: Vec<LinkedList<Entry<K, V>>>,
    num_buckets: usize,
}

impl<K, V> CustomListMap<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    fn new(num_buckets: usize) -> Self {
        let mut buckets = Vec::with_capacity(num_buckets);
        for _ in 0..num_buckets {
            buckets.push(LinkedList::new());
        }
        return CustomListMap { buckets, num_buckets };
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        return (hasher.finish() % self.num_buckets as u64) as usize;
    }

    fn insert(&mut self, key: K, value: V) {
        let index = self.hash(&key);
        let bucket = &mut self.buckets[index];

        for entry in bucket.iter_mut() {
            if entry.key == key {
                entry.value = value.clone();
                return;
            }
        }
        bucket.push_back(Entry { key, value });
    }

    fn get(&self, key: &K) -> Option<V> {
        let index = self.hash(key);
        self.buckets[index]
            .iter()
            .find(|entry| &entry.key == key)
            .map(|entry| entry.value.clone())
    }

    fn remove(&mut self, key: &K) {
        let index = self.hash(key);
        let bucket = &mut self.buckets[index];

        let mut temp = LinkedList::new();
        while let Some(entry) = bucket.pop_front() {
            if entry.key != *key {
                temp.push_back(entry);
            }
        }
    }

    
}

struct CustomVecMap<K, V> {
    buckets: Vec<Vec<Entry<K, V>>>,
    num_buckets: usize, 
}

impl<K, V> CustomVecMap<K, V> 
where
    K: Hash + Eq,
    V: Clone,
{
    fn new(num_buckets: usize) -> Self {
        let mut buckets = Vec::with_capacity(num_buckets);
        for _ in 0..num_buckets {
            buckets.push(Vec::new());
        }
        return CustomVecMap { buckets, num_buckets };
    }

    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        return (hasher.finish() % self.num_buckets as u64) as usize;
    }

    fn insert(&mut self, key: K, value: V) {
        let index = self.hash(&key);
        let bucket = &mut self.buckets[index];

        for entry in bucket.iter_mut() {
            if entry.key == key {
                entry.value = value.clone();
                return;
            }
        }
        bucket.push(Entry { key, value });
    }

    fn get(&self, key: &K) -> Option<V> {
        let index = self.hash(key);
        self.buckets[index]
            .iter()
            .find(|entry| &entry.key == key)
            .map(|entry| entry.value.clone())
    }

    fn remove(&mut self, key: &K) {
        let index = self.hash(key);
        let bucket = &mut self.buckets[index];
        let position = bucket.iter().position(|entry| &entry.key == key);

        if let Some(pos) = position {
            bucket.remove(pos);
        }
    }
}
