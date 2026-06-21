use crate::error::RedisError;
use crate::storage::Store;
use std::time::{Duration, Instant};

#[derive(Debug, PartialEq)]
pub enum Command {
    Ping,
    Set {
        key: String,
        value: String,
        ttl: Option<Duration>,
    },
    Get {
        key: String,
    },
    Del {
        key: String,
    },
    Exists {
        key: String,
    },
    Expire {
        key: String,
        seconds: u64,
    },
    Ttl {
        key: String,
    },
    DbSize,
    Quit,
}

impl Command {
    pub fn parse(input: &str) -> Result<Command, RedisError> {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            return Err(RedisError::EmptyCommand);
        }
        match parts[0].to_uppercase().as_str() {
            "PING" => Ok(Command::Ping),
            "DBSIZE" => Ok(Command::DbSize),
            "QUIT" => Ok(Command::Quit),
            "GET" if parts.len() == 2 => Ok(Command::Get {
                key: parts[1].to_string(),
            }),
            "DEL" if parts.len() == 2 => Ok(Command::Del {
                key: parts[1].to_string(),
            }),
            "EXISTS" if parts.len() == 2 => Ok(Command::Exists {
                key: parts[1].to_string(),
            }),
            "TTL" if parts.len() == 2 => Ok(Command::Ttl {
                key: parts[1].to_string(),
            }),
            "SET" if parts.len() == 3 => Ok(Command::Set {
                key: parts[1].to_string(),
                value: parts[2].to_string(),
                ttl: None,
            }),
            "SET" if parts.len() == 5 && parts[3].to_uppercase() == "EX" => {
                let secs = parts[4].parse::<u64>().map_err(|_| {
                    RedisError::InvalidArgument("EX requires a positive integer".into())
                })?;
                Ok(Command::Set {
                    key: parts[1].to_string(),
                    value: parts[2].to_string(),
                    ttl: Some(Duration::from_secs(secs)),
                })
            }
            "EXPIRE" if parts.len() == 3 => {
                let secs = parts[2].parse::<u64>().map_err(|_| {
                    RedisError::InvalidArgument("EXPIRE requires a positive integer".into())
                })?;
                Ok(Command::Expire {
                    key: parts[1].to_string(),
                    seconds: secs,
                })
            }
            cmd => Err(RedisError::UnknownCommand(cmd.to_string())),
        }
    }
}

pub async fn execute(cmd: Command, store: &Store) -> String {
    match cmd {
        Command::Ping => "+PONG\n".to_string(),
        Command::Set { key, value, ttl } => {
            let entry = match ttl {
                Some(duration) => crate::storage::Entry::with_expiry(value, duration),
                None => crate::storage::Entry::new(value),
            };
            store.lock().unwrap().insert(key, entry);
            "+OK\n".to_string()
        }
        Command::Get { key } => {
            let mut store = store.lock().unwrap();
            match store.get(&key) {
                Some(entry) if entry.is_expired() => {
                    store.remove(&key);
                    "$-1\n".to_string()
                }
                Some(entry) => format!("+{}\n", entry.value),
                None => "$-1\n".to_string(),
            }
        }
        Command::Del { key } => {
            let removed = store.lock().unwrap().remove(&key).is_some();
            if removed {
                ":1\n".to_string()
            } else {
                ":0\n".to_string()
            }
        }
        Command::Exists { key } => {
            let store = store.lock().unwrap();
            let exists = store.get(&key).map_or(false, |e| !e.is_expired());
            if exists {
                ":1\n".to_string()
            } else {
                ":0\n".to_string()
            }
        }
        Command::Expire { key, seconds } => {
            let mut store = store.lock().unwrap();
            if let Some(entry) = store.get_mut(&key) {
                entry.expires_at = Some(Instant::now() + Duration::from_secs(seconds));
                ":1\n".to_string()
            } else {
                ":0\n".to_string()
            }
        }
        Command::Ttl { key } => {
            let store = store.lock().unwrap();
            match store.get(&key) {
                None => ":-2\n".to_string(),
                Some(entry) => match entry.expires_at {
                    None => ":-1\n".to_string(),
                    Some(exp) => {
                        let now = Instant::now();
                        if now > exp {
                            ":-2\n".to_string()
                        } else {
                            format!(":{}\n", (exp - now).as_secs())
                        }
                    }
                },
            }
        }
        Command::DbSize => {
            let count = store.lock().unwrap().len();
            format!(":{}\n", count)
        }
        Command::Quit => "+OK\n".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ping() {
        assert_eq!(Command::parse("PING").unwrap(), Command::Ping);
        assert_eq!(Command::parse("ping").unwrap(), Command::Ping);
        assert_eq!(Command::parse("  PING  ").unwrap(), Command::Ping);
    }

    #[test]
    fn parse_set() {
        let cmd = Command::parse("SET mykey myvalue").unwrap();
        assert_eq!(
            cmd,
            Command::Set {
                key: "mykey".to_string(),
                value: "myvalue".to_string(),
                ttl: None,
            }
        );
    }

    #[test]
    fn parse_set_with_expiry() {
        let cmd = Command::parse("SET session abc123 EX 3600").unwrap();
        assert_eq!(
            cmd,
            Command::Set {
                key: "session".to_string(),
                value: "abc123".to_string(),
                ttl: Some(Duration::from_secs(3600)),
            }
        );
    }

    #[test]
    fn parse_get() {
        let cmd = Command::parse("GET mykey").unwrap();
        assert_eq!(
            cmd,
            Command::Get {
                key: "mykey".to_string()
            }
        );
    }

    #[test]
    fn parse_del() {
        let cmd = Command::parse("DEL mykey").unwrap();
        assert_eq!(
            cmd,
            Command::Del {
                key: "mykey".to_string()
            }
        );
    }

    #[test]
    fn parse_exists() {
        let cmd = Command::parse("EXISTS mykey").unwrap();
        assert_eq!(
            cmd,
            Command::Exists {
                key: "mykey".to_string()
            }
        );
    }

    #[test]
    fn parse_expire() {
        let cmd = Command::parse("EXPIRE mykey 60").unwrap();
        assert_eq!(
            cmd,
            Command::Expire {
                key: "mykey".to_string(),
                seconds: 60
            }
        );
    }

    #[test]
    fn parse_ttl() {
        let cmd = Command::parse("TTL mykey").unwrap();
        assert_eq!(
            cmd,
            Command::Ttl {
                key: "mykey".to_string()
            }
        );
    }

    #[test]
    fn parse_dbsize() {
        assert_eq!(Command::parse("DBSIZE").unwrap(), Command::DbSize);
    }

    #[test]
    fn parse_quit() {
        assert_eq!(Command::parse("QUIT").unwrap(), Command::Quit);
    }

    #[test]
    fn empty_command_returns_error() {
        assert!(Command::parse("").is_err());
        assert!(Command::parse("   ").is_err());
    }

    #[test]
    fn unknown_command_returns_error() {
        assert!(Command::parse("HGET key field").is_err());
    }

    #[test]
    fn wrong_arg_count_returns_error() {
        assert!(Command::parse("GET").is_err());
        assert!(Command::parse("SET key").is_err());
        assert!(Command::parse("SET key value EX notanumber").is_err());
    }
}
