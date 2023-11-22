use std::{
    vec::Vec,
    hash::{
        Hash,
        Hasher,
    },
    collections::{
        LinkedList,
        hash_map::DefaultHasher,
        HashMap,
    }
};
use criterion::{criterion_group, criterion_main, Criterion, black_box};


criterion_main!(collision_add_benchmark);

///Add Small
fn collision_add_list_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("Collision_Add_List");
    let num_buckets = 10;
    let num_items = 100;
    let mut map = CustomListMap::<u32, u32>::new(num_buckets);
    
    group.bench_function("custom_linked_list", |b| {
        b.iter(|| {
            for i in 0..num_items {
                map.insert(i % 5, i);
            }
        });
    });

    group.finish();
}

fn collision_add_vec_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("Collision_Add_Vector");
    let num_buckets = 10;
    let num_items = 100;
    let mut map = CustomVecMap::<u32, u32>::new(num_buckets);

    group.bench_function("custom_vector_map", |b| {
        b.iter(|| {
            for i in 0..num_items {
                map.insert(i % 5, i);
            }
        });
    });

    group.finish();
}

fn collision_add_std_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("Collision_Add_Standard_HashMap");
    let num_items = 100;
    let mut map = HashMap::new();

    group.bench_function("standard_hashmap", |b| {
        b.iter(|| {
            for i in 0..num_items {
                map.insert(i % 5, i);
            }
        });
    });

    group.finish();
}

//Add Large
fn collision_add_list_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("Collision_Add_List_Large");
    let num_buckets = 10;
    let num_items = 100000;
    let mut map = CustomListMap::<u32, u32>::new(num_buckets);
    
    group.bench_function("custom_linked_list", |b| {
        b.iter(|| {
            for i in 0..num_items {
                map.insert(i % 5, i);
            }
        });
    });

    group.finish();
}

fn collision_add_vec_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("Collision_Add_Vector_Large");
    let num_buckets = 10;
    let num_items = 100000;
    let mut map = CustomVecMap::<u32, u32>::new(num_buckets);

    group.bench_function("custom_vector_map", |b| {
        b.iter(|| {
            for i in 0..num_items {
                map.insert(i % 5, i);
            }
        });
    });

    group.finish();
}

fn collision_add_std_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("Collision_Add_Standard_HashMap_Large");
    let num_items = 100000;
    let mut map = HashMap::new();

    group.bench_function("standard_hashmap", |b| {
        b.iter(|| {
            for i in 0..num_items {
                map.insert(i % 5, i);
            }
        });
    });

    group.finish();
}

//Get small
fn get_list_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("Get_List_Small");
    let num_buckets = 10;
    let num_items = 100;

    let mut map = CustomListMap::new(num_buckets);
    for i in 0..num_items {
        map.insert(i % 5, i);
    }

    group.bench_function("get_list_map", |b| {
        b.iter(|| {
            for i in 0..num_items {
                black_box(map.get(&i));
            }
        });
    });

    group.finish()
}

fn get_vec_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("Get_Vec_Small");
    let num_buckets = 10;
    let num_items = 100;

    let mut map = CustomVecMap::new(num_buckets);
    for i in 0..num_items {
        map.insert(i % 5, i);
    }

    group.bench_function("get_vec_map", |b| {
        b.iter(|| {
            for i in 0..num_items {
                black_box(map.get(&i));
            }
        });
    });

    group.finish()
}

fn get_std_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("Get_STD_Small");
    let num_items = 100;

    let mut map = HashMap::new();
    for i in 0..num_items {
        map.insert(i % 5, i);
    }

    group.bench_function("get_std_map", |b| {
        b.iter(|| {
            for i in 0..num_items {
                black_box(map.get(&i));
            }
        });
    });

    group.finish()
}

//Get large
fn get_list_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("Get_List_Large");
    let num_buckets = 10;
    let num_items = 100000;

    let mut map = CustomListMap::new(num_buckets);
    for i in 0..num_items {
        map.insert(i % 5, i);
    }

    group.bench_function("get_list_map", |b| {
        b.iter(|| {
            for i in 0..num_items {
                black_box(map.get(&i));
            }
        });
    });

    group.finish()
}

fn get_vec_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("Get_Vec_Large");
    let num_buckets = 10;
    let num_items = 100000;

    let mut map = CustomVecMap::new(num_buckets);
    for i in 0..num_items {
        map.insert(i % 5, i);
    }

    group.bench_function("get_vec_map", |b| {
        b.iter(|| {
            for i in 0..num_items {
                black_box(map.get(&i));
            }
        });
    });

    group.finish()
}

fn get_std_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("Get_STD_Large");
    let num_items = 100000;

    let mut map = HashMap::new();
    for i in 0..num_items {
        map.insert(i % 5, i);
    }

    group.bench_function("get_std_map", |b| {
        b.iter(|| {
            for i in 0..num_items {
                black_box(map.get(&i));
            }
        });
    });

    group.finish()
}

//Remove small
fn remove_list_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("Remove_List_Small");
    let num_buckets = 10;
    let num_items = 100;

    let mut map = CustomListMap::new(num_buckets);
    for i in 0..num_items {
        map.insert(i % 5, i);
    }

    group.bench_function("remove_list_map", |b| {
        b.iter(|| {
            for i in 0..num_items {
                black_box(map.remove(&i));
            }
        });
    });

    group.finish()
}

fn remove_vec_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("Remove_Vec_Small");
    let num_buckets = 10;
    let num_items = 100;

    let mut map = CustomVecMap::new(num_buckets);
    for i in 0..num_items {
        map.insert(i % 5, i);
    }

    group.bench_function("remove_vec_map", |b| {
        b.iter(|| {
            for i in 0..num_items {
                black_box(map.remove(&i));
            }
        });
    });

    group.finish()
}

fn remove_std_small(c: &mut Criterion) {
    let mut group = c.benchmark_group("Get_STD_Small");
    let num_items = 100;

    let mut map = HashMap::new();
    for i in 0..num_items {
        map.insert(i % 5, i);
    }

    group.bench_function("remove_std_map", |b| {
        b.iter(|| {
            for i in 0..num_items {
                black_box(map.remove(&i));
            }
        });
    });

    group.finish()
}

//Remove Large
fn remove_list_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("Remove_List_Large");
    let num_buckets = 10;
    let num_items = 100000;

    let mut map = CustomListMap::new(num_buckets);
    for i in 0..num_items {
        map.insert(i % 5, i);
    }

    group.bench_function("remove_list_map", |b| {
        b.iter(|| {
            for i in 0..num_items {
                black_box(map.remove(&i));
            }
        });
    });

    group.finish()
}

fn remove_vec_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("Remove_Vec_Large");
    let num_buckets = 10;
    let num_items = 100000;

    let mut map = CustomVecMap::new(num_buckets);
    for i in 0..num_items {
        map.insert(i % 5, i);
    }

    group.bench_function("remove_vec_map", |b| {
        b.iter(|| {
            for i in 0..num_items {
                black_box(map.remove(&i));
            }
        });
    });

    group.finish()
}

fn remove_std_large(c: &mut Criterion) {
    let mut group = c.benchmark_group("Get_STD_Large");
    let num_items = 100000;

    let mut map = HashMap::new();
    for i in 0..num_items {
        map.insert(i % 5, i);
    }

    group.bench_function("remove_std_map", |b| {
        b.iter(|| {
            for i in 0..num_items {
                black_box(map.remove(&i));
            }
        });
    });

    group.finish()
}

criterion_group!(
    collision_add_benchmark,
    collision_add_list_small,
    collision_add_vec_small,
    collision_add_std_small,
    collision_add_list_large,
    collision_add_vec_large,
    collision_add_std_large,
    get_list_small,
    get_vec_small,
    get_std_small,
    get_list_large,
    get_vec_large,
    get_std_large,
    remove_list_small,
    remove_vec_small,
    remove_std_small,
    remove_list_large,
    remove_vec_large,
    remove_std_large,
);


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
