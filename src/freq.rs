use std::collections::HashMap;

/// Returns a hashmap of the frequency of bytes in `data`
pub fn freq_analysis(data: Vec<u8>) -> HashMap<u8, u128>{
    let mut freq_count: HashMap<u8, u128> = HashMap::new();
    for char in data {
        *freq_count.entry(char).or_insert(0) += 1;
    }
    freq_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freq_analysis() {
        let data = vec![1, 2, 2, 3, 3, 3];
        let freq_count = freq_analysis(data);
        assert_eq!(freq_count.get(&1), Some(&1));
        assert_eq!(freq_count.get(&2), Some(&2));
        assert_eq!(freq_count.get(&3), Some(&3));
    }
}