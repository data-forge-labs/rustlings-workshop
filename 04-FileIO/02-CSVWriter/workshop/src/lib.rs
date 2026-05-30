use serde::{Deserialize, Serialize};

pub const DISCOUNT: f64 = 0.1;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Product {
    pub name: String,
    pub price: f64,
}

pub fn apply_discount(product: &Product) -> Product {
    todo!()
}

pub fn total_savings(products: &[Product]) -> f64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_discount {
        #[test]
        fn test_apply_discount() {
            let p = Product { name: "Widget".into(), price: 100.0 };
            let discounted = apply_discount(&p);
            assert_eq!(discounted.name, "Widget");
            assert!((discounted.price - 90.0).abs() < 1e-6);
        }

        #[test]
        fn test_apply_discount_zero() {
            let p = Product { name: "Free".into(), price: 0.0 };
            let discounted = apply_discount(&p);
            assert!((discounted.price - 0.0).abs() < 1e-6);
        }

        #[test]
        fn test_total_savings() {
            let products = vec![
                Product { name: "A".into(), price: 100.0 },
                Product { name: "B".into(), price: 200.0 },
            ];
            let savings = total_savings(&products);
            assert!((savings - 30.0).abs() < 1e-6);
        }

        #[test]
        fn test_total_savings_empty() {
            let savings = total_savings(&[]);
            assert!((savings - 0.0).abs() < 1e-6);
        }
    }
}
