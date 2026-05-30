/// A resource that tracks its lifecycle (simulates file handle, DB connection)
pub struct Resource {
    pub id: u32,
    pub is_open: bool,
}

impl Resource {
    pub fn new(id: u32) -> Self {
        todo!()
    }

    pub fn close(&mut self) {
        todo!()
    }

    pub fn is_open(&self) -> bool {
        todo!()
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        todo!()
    }
}

/// Demonstrate RAII: resource is automatically closed when scope ends
pub fn raii_demo() -> Vec<String> {
    todo!()
}

/// Transfer ownership and demonstrate drop at end of scope
pub fn ownership_transfer() -> u32 {
    todo!()
}

/// Borrow a resource without taking ownership
pub fn borrow_resource(res: &Resource) -> u32 {
    todo!()
}

/// Return a list of RAII/OBRM concepts covered
pub fn obrm_concepts() -> Vec<&'static str> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_resource_lifecycle {
        use super::*;

        #[test]
        fn test_new_resource_is_open() {
            let res = Resource::new(1);
            assert!(res.is_open);
            assert_eq!(res.id, 1);
        }

        #[test]
        fn test_close_resource() {
            let mut res = Resource::new(2);
            assert!(res.is_open());
            res.close();
            assert!(!res.is_open());
        }

        #[test]
        fn test_double_close_does_not_panic() {
            let mut res = Resource::new(3);
            res.close();
            // closing an already-closed resource should be safe
            res.close();
            assert!(!res.is_open());
        }
    }

    mod step_02_raii_demo {
        use super::*;

        #[test]
        fn test_raii_demo_returns_messages() {
            let messages = raii_demo();
            assert!(!messages.is_empty(), "raii_demo should return lifecycle messages");
        }

        #[test]
        fn test_raii_demo_contains_open_and_close() {
            let messages = raii_demo();
            let all_text = messages.join(" ");
            assert!(all_text.to_lowercase().contains("open") || all_text.to_lowercase().contains("created"));
            assert!(all_text.to_lowercase().contains("close") || all_text.to_lowercase().contains("drop"));
        }
    }

    mod step_03_ownership {
        use super::*;

        #[test]
        fn test_ownership_transfer_returns_count() {
            let count = ownership_transfer();
            assert!(count > 0, "should transfer at least one resource");
        }

        #[test]
        fn test_borrow_resource_returns_id() {
            let res = Resource::new(42);
            let id = borrow_resource(&res);
            assert_eq!(id, 42);
            // res should still be usable after borrowing
            assert!(res.is_open());
        }

        #[test]
        fn test_borrow_resource_does_not_close() {
            let res = Resource::new(7);
            let id = borrow_resource(&res);
            assert_eq!(id, 7);
            assert!(res.is_open(), "borrowed resource should remain open");
        }
    }

    mod step_04_concepts {
        use super::*;

        #[test]
        fn test_obrm_concepts_non_empty() {
            let concepts = obrm_concepts();
            assert!(!concepts.is_empty(), "should list at least one concept");
        }

        #[test]
        fn test_obrm_concepts_includes_drop() {
            let concepts = obrm_concepts();
            let found = concepts.iter().any(|c| c.to_lowercase().contains("drop"));
            assert!(found, "concepts should include Drop trait");
        }

        #[test]
        fn test_obrm_concepts_includes_raii() {
            let concepts = obrm_concepts();
            let found = concepts.iter().any(|c| c.to_uppercase().contains("RAII"));
            assert!(found, "concepts should include RAII");
        }
    }
}
