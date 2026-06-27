use std::collections::HashMap;

pub fn languages() -> HashMap<String, u32> {
    let mut m = HashMap::new();
    m.insert("C".to_string(), 1972);
    m.insert("C++".to_string(), 1979);
    m.insert("Python".to_string(), 1991);
    m.insert("Java".to_string(), 1995);
    m.insert("JavaScript".to_string(), 1995);
    m.insert("C#".to_string(), 2000);
    m.insert("Go".to_string(), 2009);
    m.insert("Kotlin".to_string(), 2011);
    m.insert("Rust".to_string(), 2010);
    m.insert("Swift".to_string(), 2014);
    m.insert("TypeScript".to_string(), 2012);
    m.insert("Julia".to_string(), 2012);
    m.insert("Zig".to_string(), 2016);
    m.insert("Nim".to_string(), 2008);
    m.insert("Crystal".to_string(), 2014);
    m
}

pub fn normalize(languages: &mut HashMap<String, u32>) -> HashMap<String, u32> {
    let max_year = *languages.values().max().unwrap_or(&0);
    let min_year = *languages.values().min().unwrap_or(&0);
    let range = (max_year - min_year) as f64;
    languages.iter().map(|(k, &v)| {
        let weight = if range == 0.0 {
            50
        } else {
            (1.0 + 99.0 * (v as f64 - min_year as f64) / range) as u32
        };
        (k.clone(), weight)
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_languages {
        #[test]
        fn test_languages_contains_rust() {
            let langs = languages();
            assert!(langs.contains_key("Rust"));
        }

        #[test]
        fn test_languages_count() {
            let langs = languages();
            assert_eq!(langs.len(), 15);
        }

        #[test]
        fn test_languages_year() {
            let langs = languages();
            assert_eq!(langs.get("Python"), Some(&1991));
        }
    }

    mod step_02_normalize {
        #[test]
        fn test_normalize_returns_weights() {
            let mut langs = languages();
            let weights = normalize(&mut langs);
            assert_eq!(weights.len(), 15);
        }

        #[test]
        fn test_normalize_weights_in_range() {
            let mut langs = languages();
            let weights = normalize(&mut langs);
            for (_, w) in &weights {
                assert!(*w >= 1 && *w <= 100, "Weight {w} out of range");
            }
        }

        #[test]
        fn test_normalize_latest_gets_max() {
            let mut langs = languages();
            let weights = normalize(&mut langs);
            let rust_weight = weights.get("Rust").unwrap();
            let c_weight = weights.get("C").unwrap();
            assert!(rust_weight > c_weight);
        }
    }
}
