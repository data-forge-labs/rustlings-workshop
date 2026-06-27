use mockall::automock;

#[automock]
pub trait DataSource {
    fn fetch(&self, query: &str) -> Result<Vec<String>, String>;
    fn health(&self) -> bool;
    fn schema(&self) -> Vec<String>;
    fn record_count(&self, table: &str) -> Result<u64, String>;
}

pub fn run_etl(source: &dyn DataSource, query: &str) -> Result<usize, String> {
    let rows = source.fetch(query)?;
    Ok(rows.len())
}

pub fn count_records(source: &dyn DataSource, query: &str) -> Result<u64, String> {
    let rows = source.fetch(query)?;
    Ok(rows.len() as u64)
}

pub fn is_healthy(source: &dyn DataSource) -> bool {
    source.health()
}

pub fn get_schema(source: &dyn DataSource) -> Vec<String> {
    source.schema()
}

pub fn filter_rows(source: &dyn DataSource, query: &str, prefix: &str) -> Result<Vec<String>, String> {
    let rows = source.fetch(query)?;
    Ok(rows.into_iter().filter(|r| r.starts_with(prefix)).collect())
}

pub fn batch_etl(source: &dyn DataSource, queries: &[&str]) -> Result<Vec<Vec<String>>, String> {
    queries.iter().map(|q| source.fetch(q)).collect()
}

pub fn validate_pipeline(source: &dyn DataSource) -> Result<&'static str, String> {
    if !source.health() {
        return Err("source is unhealthy".to_string());
    }
    let rows = source.fetch("SELECT 1")?;
    if rows.is_empty() {
        return Err("source returned empty result".to_string());
    }
    Ok("ok")
}

pub fn total_rows(source: &dyn DataSource, tables: &[&str]) -> Result<u64, String> {
    tables
        .iter()
        .map(|t| source.record_count(t))
        .sum::<Result<u64, String>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    #[test]
    fn test_run_etl_with_mock() {
        let mut mock = MockDataSource::new();
        mock.expect_fetch()
            .with(eq("SELECT * FROM users"))
            .returning(|_| Ok(vec!["alice".into(), "bob".into(), "carol".into()]));
        assert_eq!(run_etl(&mock, "SELECT * FROM users").unwrap(), 3);
    }

    #[test]
    fn test_run_etl_with_empty_result() {
        let mut mock = MockDataSource::new();
        mock.expect_fetch()
            .returning(|_| Ok(vec![]));
        assert_eq!(run_etl(&mock, "SELECT 1").unwrap(), 0);
    }

    #[test]
    fn test_run_etl_with_error() {
        let mut mock = MockDataSource::new();
        mock.expect_fetch()
            .returning(|_| Err("connection refused".into()));
        assert!(run_etl(&mock, "SELECT 1").is_err());
    }

    #[test]
    fn test_count_records() {
        let mut mock = MockDataSource::new();
        mock.expect_fetch()
            .returning(|_| Ok(vec!["a".into(); 42]));
        assert_eq!(count_records(&mock, "SELECT *").unwrap(), 42);
    }

    #[test]
    fn test_is_healthy() {
        let mut mock = MockDataSource::new();
        mock.expect_health().return_const(true);
        assert!(is_healthy(&mock));

        let mut mock = MockDataSource::new();
        mock.expect_health().return_const(false);
        assert!(!is_healthy(&mock));
    }

    #[test]
    fn test_get_schema() {
        let mut mock = MockDataSource::new();
        mock.expect_schema()
            .return_const(vec!["id".into(), "name".into(), "email".into()]);
        let schema = get_schema(&mock);
        assert_eq!(schema, vec!["id", "name", "email"]);
    }

    #[test]
    fn test_filter_rows() {
        let mut mock = MockDataSource::new();
        mock.expect_fetch()
            .returning(|_| Ok(vec![
                "apple".into(),
                "apricot".into(),
                "banana".into(),
                "avocado".into(),
            ]));
        let result = filter_rows(&mock, "SELECT *", "ap").unwrap();
        assert_eq!(result, vec!["apple", "apricot", "avocado"]);
    }

    #[test]
    fn test_batch_etl() {
        let mut mock = MockDataSource::new();
        mock.expect_fetch()
            .with(eq("q1"))
            .returning(|_| Ok(vec!["a".into(), "b".into()]));
        mock.expect_fetch()
            .with(eq("q2"))
            .returning(|_| Ok(vec!["c".into()]));
        let results = batch_etl(&mock, &["q1", "q2"]).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].len(), 2);
        assert_eq!(results[1].len(), 1);
    }

    #[test]
    fn test_validate_pipeline_ok() {
        let mut mock = MockDataSource::new();
        mock.expect_health().return_const(true);
        mock.expect_fetch()
            .returning(|_| Ok(vec!["row".into()]));
        assert_eq!(validate_pipeline(&mock).unwrap(), "ok");
    }

    #[test]
    fn test_validate_pipeline_unhealthy() {
        let mut mock = MockDataSource::new();
        mock.expect_health().return_const(false);
        assert!(validate_pipeline(&mock).is_err());
    }

    #[test]
    fn test_validate_pipeline_empty() {
        let mut mock = MockDataSource::new();
        mock.expect_health().return_const(true);
        mock.expect_fetch()
            .returning(|_| Ok(vec![]));
        assert!(validate_pipeline(&mock).is_err());
    }

    #[test]
    fn test_total_rows() {
        let mut mock = MockDataSource::new();
        mock.expect_record_count()
            .with(eq("t1"))
            .returning(|_| Ok(100));
        mock.expect_record_count()
            .with(eq("t2"))
            .returning(|_| Ok(50));
        assert_eq!(total_rows(&mock, &["t1", "t2"]).unwrap(), 150);
    }
}
