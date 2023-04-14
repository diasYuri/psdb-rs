mod bit_array;
mod counter_bloom_filter;
mod mmh3;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use crate::counter_bloom_filter::CounterBloomFilter;

    #[test]
    fn test_insert_and_contains() {
        let path = "test_insert_and_contains.cbf";
        let mut cbf = CounterBloomFilter::new(1000, 5, path).unwrap();

        cbf.insert(b"hello");
        cbf.insert(b"world");

        assert!(cbf.contains(b"hello"));
        assert!(cbf.contains(b"world"));
        assert!(!cbf.contains(b"rust"));

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_remove() {
        let path = "./test_remove.cbf";
        let mut cbf = CounterBloomFilter::new(1000, 5, path).unwrap();

        cbf.insert(b"hello");
        cbf.insert(b"world");

        cbf.remove(b"hello");

        assert!(!cbf.contains(b"hello"));
        assert!(cbf.contains(b"world"));

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_false_positive_rate() {
        let num_items = 1000;
        let filter_size = 10_000;
        let num_hash_functions = 5;
        let path = "test_false_positive_rate.cbf";
        let mut cbf = CounterBloomFilter::new(filter_size, num_hash_functions, path).unwrap();

        for i in 0..num_items {
            cbf.insert(format!("item{}", i).as_bytes());
        }

        let mut false_positives = 0;
        let num_tests = 10_000;

        for i in num_items..num_tests {
            if cbf.contains(format!("item{}", i).as_bytes()) {
                false_positives += 1;
            }
        }

        let false_positive_rate = false_positives as f64 / num_tests as f64;
        println!("False positive rate: {}", false_positive_rate);
        assert!(false_positive_rate < 0.05);

        fs::remove_file(path).unwrap();
    }
}
