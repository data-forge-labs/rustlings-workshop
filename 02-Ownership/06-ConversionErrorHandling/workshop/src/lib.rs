use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
    #[error("missing field: {0}")]
    Missing(&'static str),
    #[error("validation failed: {0}")]
    Validation(String),
}

pub fn unwrap_or_default_when_none(opt: Option<i32>) -> i32 {
    todo!()
}

pub fn map_or_default(opt: Option<i32>, fallback: i32) -> i32 {
    todo!()
}

pub fn ok_or_convert(opt: Option<String>) -> Result<String, AppError> {
    todo!()
}

pub fn map_err_convert(s: &str) -> Result<i32, AppError> {
    todo!()
}

pub fn and_then_chain(s: &str) -> Result<i32, AppError> {
    todo!()
}

pub fn read_and_parse(line: &str) -> Result<i32, AppError> {
    todo!()
}

pub fn multi_step_pipeline(s: &str) -> Result<i32, AppError> {
    todo!()
}

pub fn first_present<'a>(opts: &'a [Option<&'a str>]) -> Result<&'a str, AppError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    mod step_01_option_methods {
        use super::*;

        #[test]
        fn test_unwrap_or_default_when_none() {
            assert_eq!(unwrap_or_default_when_none(None), 0);
            assert_eq!(unwrap_or_default_when_none(Some(42)), 42);
        }

        #[test]
        fn test_map_or_default() {
            assert_eq!(map_or_default(None, 7), 7);
            assert_eq!(map_or_default(Some(3), 7), 3);
        }

        #[test]
        fn test_ok_or_convert() {
            let r: Result<String, AppError> = ok_or_convert(None);
            assert!(matches!(r, Err(AppError::Missing(_))));
            assert_eq!(ok_or_convert(Some("ok".into())).unwrap(), "ok");
        }
    }

    mod step_02_result_methods {
        use super::*;

        #[test]
        fn test_map_err_convert() {
            let r = map_err_convert("not-a-number");
            assert!(matches!(r, Err(AppError::Parse(_))));
            assert_eq!(map_err_convert("123").unwrap(), 123);
        }

        #[test]
        fn test_and_then_chain() {
            assert_eq!(and_then_chain("42").unwrap(), 84);
            assert!(and_then_chain("bad").is_err());
            assert_eq!(and_then_chain("-3").unwrap(), -6);
        }
    }

    mod step_03_from_conversion {
        use super::*;

        #[test]
        fn test_read_and_parse_ok() {
            assert_eq!(read_and_parse("100").unwrap(), 100);
        }

        #[test]
        fn test_read_and_parse_err() {
            let r = read_and_parse("oops");
            assert!(matches!(r, Err(AppError::Parse(_))));
        }
    }

    mod step_04_question_mark {
        use super::*;

        #[test]
        fn test_multi_step_pipeline_ok() {
            assert_eq!(multi_step_pipeline("5").unwrap(), 15);
        }

        #[test]
        fn test_multi_step_pipeline_propagates_parse() {
            assert!(matches!(
                multi_step_pipeline("oops"),
                Err(AppError::Parse(_))
            ));
        }

        #[test]
        fn test_first_present() {
            assert_eq!(first_present(&[None, Some("b"), Some("c")]).unwrap(), "b");
            assert!(first_present(&[None, None]).is_err());
        }
    }
}
