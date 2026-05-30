use std::collections::HashMap;

pub fn languages() -> HashMap<String, u32> {
    todo!()
}

pub fn normalize(languages: &mut HashMap<String, u32>) -> HashMap<String, u32> {
    todo!()
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
