use rand::prelude::*;

fn main() {
    const N: usize = 10_000_000;
    let mut rng = thread_rng();
    let data = (0 .. N)
        .map(|_| (rng.gen::<u64>(), rng.gen::<u64>()) )
        .collect::<Vec<_>>();
    for i in &[10, 100, 1000, 10_000, 100_000] {
        let mut htables = Vec::with_capacity(N / i);
        let start = std::time::Instant::now();
        for d in data.chunks(*i) {
            let mut ht = std::collections::HashMap::new();
            for (key, value) in d {
                ht.insert(key, value);
            }
            htables.push(ht);
        }
        let ht_creation_time = std::time::Instant::now()
            .duration_since(start)
            .as_secs_f64();

        let mut rtables = Vec::with_capacity(N / i);
        let start = std::time::Instant::now();
        for d in data.chunks(*i) {
            let mut rb = rbtree::RBTree::new();
            for (key, value) in d {
                rb.insert(key, value);
            }
            rtables.push(rb);
        }
        let rb_creation_time = std::time::Instant::now()
            .duration_since(start)
            .as_secs_f64();

        let mut hsums: Vec<u64> = Vec::with_capacity(N / i);
        let start = std::time::Instant::now();
        for ht in htables.iter() {
            hsums.push(ht.iter().map(|(_, value)| *value).sum());
        }
        let ht_sum_time = std::time::Instant::now()
            .duration_since(start)
            .as_secs_f64();

        let mut rsums: Vec<u64> = Vec::with_capacity(N / i);
        let start = std::time::Instant::now();
        for rb in rtables.iter() {
            rsums.push(rb.iter().map(|(_, value)| *value).sum());
        }
        let rb_sum_time = std::time::Instant::now()
            .duration_since(start)
            .as_secs_f64();

        let edist_of_sums: f64 = hsums.iter().zip(rsums.iter())
            .map(|(rs, hs)| {
                let diff = (*rs as f64) - (*hs as f64);
                diff * diff
            }).sum();
        println!("generated {} + {} maps of size {}", htables.len(), rtables.len(), i);
        println!("Euclidian distance between sums: {}", edist_of_sums);
        println!("htable: ({:.4}s, {:.4}s) (GenTime, SumTime)", ht_creation_time, ht_sum_time);
        println!("rtable: ({:.4}s, {:.4}s) (GenTime, SumTime)", rb_creation_time, rb_sum_time);
    }
}
