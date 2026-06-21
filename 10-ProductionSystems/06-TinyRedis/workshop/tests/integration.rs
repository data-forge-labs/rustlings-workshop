use std::time::Duration;
use tiny_redis::command::{execute, Command};
use tiny_redis::storage::new_store;

#[tokio::test]
async fn set_and_get() {
    let store = new_store();
    execute(
        Command::Set {
            key: "name".to_string(),
            value: "Alice".to_string(),
            ttl: None,
        },
        &store,
    )
    .await;
    let response = execute(Command::Get { key: "name".to_string() }, &store).await;
    assert_eq!(response, "+Alice\n");
}

#[tokio::test]
async fn get_missing_key_returns_nil() {
    let store = new_store();
    let response = execute(Command::Get { key: "ghost".to_string() }, &store).await;
    assert_eq!(response, "$-1\n");
}

#[tokio::test]
async fn del_removes_key() {
    let store = new_store();
    execute(
        Command::Set {
            key: "temp".to_string(),
            value: "value".to_string(),
            ttl: None,
        },
        &store,
    )
    .await;
    let del_response = execute(Command::Del { key: "temp".to_string() }, &store).await;
    assert_eq!(del_response, ":1\n");
    let get_response = execute(Command::Get { key: "temp".to_string() }, &store).await;
    assert_eq!(get_response, "$-1\n");
}

#[tokio::test]
async fn del_nonexistent_key_returns_zero() {
    let store = new_store();
    let response = execute(Command::Del { key: "nope".to_string() }, &store).await;
    assert_eq!(response, ":0\n");
}

#[tokio::test]
async fn exists_returns_one_for_existing_key() {
    let store = new_store();
    execute(
        Command::Set {
            key: "mykey".to_string(),
            value: "val".to_string(),
            ttl: None,
        },
        &store,
    )
    .await;
    let response = execute(Command::Exists { key: "mykey".to_string() }, &store).await;
    assert_eq!(response, ":1\n");
}

#[tokio::test]
async fn exists_returns_zero_for_missing_key() {
    let store = new_store();
    let response = execute(Command::Exists { key: "nope".to_string() }, &store).await;
    assert_eq!(response, ":0\n");
}

#[tokio::test]
async fn expired_key_returns_nil() {
    let store = new_store();
    execute(
        Command::Set {
            key: "short".to_string(),
            value: "lived".to_string(),
            ttl: Some(Duration::from_millis(1)),
        },
        &store,
    )
    .await;
    tokio::time::sleep(Duration::from_millis(10)).await;
    let response = execute(Command::Get { key: "short".to_string() }, &store).await;
    assert_eq!(response, "$-1\n");
}

#[tokio::test]
async fn dbsize_counts_active_keys() {
    let store = new_store();
    execute(
        Command::Set {
            key: "a".to_string(),
            value: "1".to_string(),
            ttl: None,
        },
        &store,
    )
    .await;
    execute(
        Command::Set {
            key: "b".to_string(),
            value: "2".to_string(),
            ttl: None,
        },
        &store,
    )
    .await;
    execute(
        Command::Set {
            key: "c".to_string(),
            value: "3".to_string(),
            ttl: None,
        },
        &store,
    )
    .await;
    let response = execute(Command::DbSize, &store).await;
    assert_eq!(response, ":3\n");
}

#[tokio::test]
async fn ping_returns_pong() {
    let store = new_store();
    let response = execute(Command::Ping, &store).await;
    assert_eq!(response, "+PONG\n");
}

#[tokio::test]
async fn overwrite_existing_key() {
    let store = new_store();
    execute(
        Command::Set {
            key: "count".to_string(),
            value: "1".to_string(),
            ttl: None,
        },
        &store,
    )
    .await;
    execute(
        Command::Set {
            key: "count".to_string(),
            value: "99".to_string(),
            ttl: None,
        },
        &store,
    )
    .await;
    let response = execute(Command::Get { key: "count".to_string() }, &store).await;
    assert_eq!(response, "+99\n");
}

#[tokio::test]
async fn ttl_returns_negative_one_for_no_expiry() {
    let store = new_store();
    execute(
        Command::Set {
            key: "k".to_string(),
            value: "v".to_string(),
            ttl: None,
        },
        &store,
    )
    .await;
    let response = execute(Command::Ttl { key: "k".to_string() }, &store).await;
    assert_eq!(response, ":-1\n");
}

#[tokio::test]
async fn ttl_returns_negative_two_for_missing_key() {
    let store = new_store();
    let response = execute(Command::Ttl { key: "nope".to_string() }, &store).await;
    assert_eq!(response, ":-2\n");
}

#[tokio::test]
async fn expire_sets_expiry_on_existing_key() {
    let store = new_store();
    execute(
        Command::Set {
            key: "k".to_string(),
            value: "v".to_string(),
            ttl: None,
        },
        &store,
    )
    .await;
    let response = execute(
        Command::Expire {
            key: "k".to_string(),
            seconds: 10,
        },
        &store,
    )
    .await;
    assert_eq!(response, ":1\n");
    let ttl = execute(Command::Ttl { key: "k".to_string() }, &store).await;
    assert!(ttl.starts_with(':'));
    let seconds: i64 = ttl.trim_start_matches(':').trim().parse().unwrap();
    assert!(seconds <= 10 && seconds > 0);
}

#[tokio::test]
async fn expire_returns_zero_for_missing_key() {
    let store = new_store();
    let response = execute(
        Command::Expire {
            key: "nope".to_string(),
            seconds: 10,
        },
        &store,
    )
    .await;
    assert_eq!(response, ":0\n");
}
