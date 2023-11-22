use std::collections::{
    hash_map::DefaultHasher,
    LinkedList,
};
use std::hash::{Hash, Hasher};
use criterion::{criterion_group, criterion_main, Criterion};

/*fn main() {
    let mut vecvec: Vec<Vec<u32>> = Vec::new();
    let mut innervec: Vec<u32> = Vec::new();
}*/

fn collision_add_list(c: &mut Criterion) {
    let mut group = c.benchmark_group("Collision_Add_List");

    group.bench_function("custom_linked_list", |b| {
        
    });
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



struct CustomListMap<K, V> {
    buckets: Vec<LinkedList<(K, V)>>,
    size: usize,
    capacity: usize,
}

impl<K, V> CustomListMap<K, V>
where
    K: Eq + std::hash::Hash,
{
    fn new(capacity: usize) -> Self {
        let mut buckets = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buckets.push(LinkedList::new());
        }
        CustomListMap { buckets, size: 0, capacity, }
    }

    fn insert(&mut self, key: K, value: V) {
        let hash = self.hash(&key);
        let index = hash % self.capacity;
        self.buckets[index].push_back((key, value));
        self.size += 1;
    }

    fn get(&self, key: &K) -> Option<&V> {
        let hash = self.hash(&key);
        let index = hash % self.capacity;
        for(k, v) in &self.buckets[index] {
            if k == key {
                return Some(v);
            }
        }
        None
    }    

    fn remove(&mut self, key: &K) -> Option<V> {
        let hash = self.hash(&key);
        let index = hash & self.capacity;
        if let Some(pos) = self.buckets[index].iter().position(|(k, _)| k == key) {
            self.size -= 1;
            return Some(self.buckets[index].remove(pos).1);
        }
        None
        
    }
    
    fn hash(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(hasher) as usize;
        hasher.finish() as usize
    }

}

struct CustomVecMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    size: usize,
    capacity: usize, 
}

impl<K, V> CustomVecMap<K, V> {
    fn new(capacity: usize) -> Self {
        let mut buckets = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            buckets.push(Vec::new());
        }
        CustomVecMap { buckets, size: 0, capacity, }
    }   
}

