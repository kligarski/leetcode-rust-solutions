use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};

const NO_BUCKETS: usize = 199;
struct RandomizedSet {
    buckets: Vec<Vec<i32>>,
    filled_buckets_vec: Vec<usize>,
    filled_buckets_map: [bool; NO_BUCKETS],
    rng: ThreadRng,
}

impl RandomizedSet {
    fn new() -> Self {
        let mut buckets: Vec<Vec<i32>> = Vec::with_capacity(NO_BUCKETS);
        for _ in 0..NO_BUCKETS {
            buckets.push(Vec::new());
        }

        RandomizedSet {
            buckets,
            filled_buckets_vec: Vec::new(),
            filled_buckets_map: [false; NO_BUCKETS],
            rng: thread_rng(),
        }
    }

    fn get_hash(&self, val: i32) -> usize {
        val as usize % NO_BUCKETS
    }

    fn get_hash_and_bucket(&mut self, val: i32) -> (usize, &mut Vec<i32>) {
        let hash = self.get_hash(val);
        (hash, &mut self.buckets[hash])
    }

    fn insert(&mut self, val: i32) -> bool {
        let (hash, bucket) = self.get_hash_and_bucket(val);

        if bucket.contains(&val) {
            return false;
        } else {
            bucket.push(val);
            if !self.filled_buckets_map[hash] {
                self.filled_buckets_map[hash] = true;
                self.filled_buckets_vec.push(hash);
            }
            return true;
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        let (hash, bucket) = self.get_hash_and_bucket(val);

        let mut element_index = None;
        for (index, el) in bucket.iter().enumerate() {
            if *el == val {
                element_index = Some(index);
                break;
            }
        }

        if let Some(index) = element_index {
            bucket.swap_remove(index);
            if bucket.len() == 0 {
                self.filled_buckets_map[hash] = false;

                let mut filled_bucket_index = None;
                for (filled_index, el) in self.filled_buckets_vec.iter().enumerate() {
                    if *el == hash {
                        filled_bucket_index = Some(filled_index);
                        break;
                    }
                }
                self.filled_buckets_vec.swap_remove(filled_bucket_index.unwrap());
            }
            return true;
        } else {
            return false;
        }
    }

    fn get_random(&mut self) -> i32 {
        let bucket_index = self.filled_buckets_vec.choose(&mut self.rng).unwrap();
        let bucket = &self.buckets[*bucket_index];

        bucket.choose(&mut self.rng).copied().unwrap()
    }
}

fn main() {
    let mut obj = RandomizedSet::new();
    let ret_1: bool = obj.insert(1);
    let ret_2: bool = obj.remove(2);
    let ret_3: bool = obj.insert(2);
    let ret_4: i32 = obj.get_random();
    let ret_5: bool = obj.remove(1);
    let ret_6: bool = obj.insert(2);
    let ret_7: i32 = obj.get_random();

    dbg!(ret_1, ret_2, ret_3, ret_4, ret_5, ret_6, ret_7);
    assert_eq!(ret_1, true);
    assert_eq!(ret_2, false);
    assert_eq!(ret_3, true);
    assert_eq!(ret_5, true);
    assert_eq!(ret_6, false);
    assert_eq!(ret_7, 2);
}
