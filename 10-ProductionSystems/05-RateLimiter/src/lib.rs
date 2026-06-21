use std::time::{Duration, SystemTime};
use thiserror::Error;

// =============================================================================
// Error types
// =============================================================================

#[derive(Error, Debug)]
pub enum RateLimitError {
    #[error("Rate limit exceeded. Reset after {0:?}")]
    RateLimitExceeded(SystemTime),

    #[error("Redis error: {0}")]
    RedisError(String),

    #[error("Timeout")]
    Timeout,
}

// =============================================================================
// Clock abstraction (for testability)
// =============================================================================

pub trait Clock {
    fn now(&self) -> SystemTime;
}

pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> SystemTime {
        SystemTime::now()
    }
}

// =============================================================================
// Rate limit attempt structure
// =============================================================================

pub struct AcquireAttempt {
    pub tokens_to_acquire: u32,
    pub max_tokens_per_window: u32,
    pub window_duration: Duration,
    pub previous_window_requests: u32,
    pub current_window_requests: u32,
}

// =============================================================================
// Algorithm trait (pluggable)
// =============================================================================

pub trait RateLimitAlgorithm {
    fn try_acquire(
        &self,
        attempt: &AcquireAttempt,
    ) -> Result<(u32, SystemTime), RateLimitError>;
}

// =============================================================================
// Sliding window implementation
// =============================================================================

#[derive(Debug, Clone)]
pub struct SlidingWindow<C: Clock> {
    clock: C,
}

impl SlidingWindow<SystemClock> {
    pub fn new() -> Self {
        Self::with_clock(SystemClock)
    }
}

impl<C: Clock> SlidingWindow<C> {
    pub fn with_clock(clock: C) -> SlidingWindow<C> {
        SlidingWindow { clock }
    }
}

fn to_unix_millis(t: SystemTime) -> u128 {
    t.duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

impl<C: Clock> RateLimitAlgorithm for SlidingWindow<C> {
    fn try_acquire(
        &self,
        attempt: &AcquireAttempt,
    ) -> Result<(u32, SystemTime), RateLimitError> {
        let window_ms = attempt.window_duration.as_millis();

        let now = self.clock.now();
        let unix_now = to_unix_millis(now);

        let time_in_current_window = unix_now % window_ms;
        let remaining_window_time = window_ms - time_in_current_window;

        let reset_after = now + Duration::from_millis(remaining_window_time as u64);

        // Fast path: current window alone exceeds limit
        if attempt
            .current_window_requests
            .saturating_add(attempt.tokens_to_acquire)
            > attempt.max_tokens_per_window
        {
            return Err(RateLimitError::RateLimitExceeded(reset_after));
        }

        // Sliding window calculation
        let previous_window_weight = remaining_window_time as f64 / window_ms as f64;
        let used = attempt.current_window_requests.saturating_add(
            (attempt.previous_window_requests as f64 * previous_window_weight).round() as u32,
        );

        let possible_used = used.saturating_add(attempt.tokens_to_acquire);
        if possible_used > attempt.max_tokens_per_window {
            Err(RateLimitError::RateLimitExceeded(reset_after))
        } else {
            Ok((
                attempt.max_tokens_per_window.saturating_sub(possible_used),
                reset_after,
            ))
        }
    }
}

// =============================================================================
// Policy system
// =============================================================================

#[derive(Debug, Clone)]
pub enum PatternType {
    Exact,
    Prefix,
}

#[derive(Debug, Clone)]
pub struct PolicyDefinition {
    pub pattern: String,
    pub pattern_type: PatternType,
    pub max_tokens: u32,
    pub window_secs: u64,
    pub priority: u32,
}

pub struct PolicyEngine {
    pub policies: Vec<PolicyDefinition>,
    pub default_policy: PolicyDefinition,
}

impl PolicyEngine {
    pub fn find_policy(&self, key: &str) -> &PolicyDefinition {
        self.policies
            .iter()
            .filter(|rule| match rule.pattern_type {
                PatternType::Exact => rule.pattern == key,
                PatternType::Prefix => key.starts_with(&rule.pattern),
            })
            .max_by_key(|rule| rule.priority)
            .unwrap_or(&self.default_policy)
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{Duration, SystemTime};
    use proptest::prelude::*;

    // Mock clock for deterministic testing
    struct MockClock {
        now: SystemTime,
    }

    impl MockClock {
        fn new(now: SystemTime) -> Self {
            MockClock { now }
        }
    }

    impl Clock for MockClock {
        fn now(&self) -> SystemTime {
            self.now
        }
    }

    // =========================================================================
    // Step 1: Basic sliding window
    // =========================================================================

    #[test]
    fn test_sliding_window_allowed() {
        let base = SystemTime::UNIX_EPOCH + Duration::from_secs(1761948300);
        let clock = MockClock::new(base);
        let algo = SlidingWindow::with_clock(clock);

        let attempt = AcquireAttempt {
            tokens_to_acquire: 1,
            max_tokens_per_window: 100,
            window_duration: Duration::from_secs(60),
            previous_window_requests: 50,
            current_window_requests: 30,
        };

        let (remaining, _) = algo.try_acquire(&attempt).unwrap();
        assert!(remaining > 0);
    }

    #[test]
    fn test_sliding_window_exceeded() {
        let base = SystemTime::UNIX_EPOCH + Duration::from_secs(1761948300);
        let clock = MockClock::new(base);
        let algo = SlidingWindow::with_clock(clock);

        let attempt = AcquireAttempt {
            tokens_to_acquire: 1,
            max_tokens_per_window: 100,
            window_duration: Duration::from_secs(60),
            previous_window_requests: 80,
            current_window_requests: 30,
        };

        assert!(algo.try_acquire(&attempt).is_err());
    }

    // =========================================================================
    // Step 2: Saturating arithmetic safety
    // =========================================================================

    #[test]
    fn test_saturating_add_overflow() {
        let base = SystemTime::UNIX_EPOCH + Duration::from_secs(1761948300);
        let clock = MockClock::new(base);
        let algo = SlidingWindow::with_clock(clock);

        let attempt = AcquireAttempt {
            tokens_to_acquire: 1,
            max_tokens_per_window: u32::MAX,
            window_duration: Duration::from_secs(60),
            previous_window_requests: u32::MAX,
            current_window_requests: u32::MAX,
        };

        // Should not panic — saturating arithmetic
        let _ = algo.try_acquire(&attempt);
    }

    // =========================================================================
    // Step 3: Edge cases
    // =========================================================================

    #[test]
    fn test_sliding_window_at_boundary() {
        let base = SystemTime::UNIX_EPOCH + Duration::from_secs(1761948360);
        let clock = MockClock::new(base);
        let algo = SlidingWindow::with_clock(clock);

        let attempt = AcquireAttempt {
            tokens_to_acquire: 100,
            max_tokens_per_window: 100,
            window_duration: Duration::from_secs(60),
            previous_window_requests: 0,
            current_window_requests: 0,
        };

        let (remaining, _) = algo.try_acquire(&attempt).unwrap();
        assert_eq!(remaining, 0);
    }

    #[test]
    fn test_sliding_window_single_token() {
        let base = SystemTime::UNIX_EPOCH + Duration::from_secs(1761948300);
        let clock = MockClock::new(base);
        let algo = SlidingWindow::with_clock(clock);

        let attempt = AcquireAttempt {
            tokens_to_acquire: 1,
            max_tokens_per_window: 1,
            window_duration: Duration::from_secs(60),
            previous_window_requests: 0,
            current_window_requests: 0,
        };

        let (remaining, _) = algo.try_acquire(&attempt).unwrap();
        assert_eq!(remaining, 0);
    }

    // =========================================================================
    // Step 4: Policy engine
    // =========================================================================

    #[test]
    fn test_policy_exact_match() {
        let engine = PolicyEngine {
            policies: vec![PolicyDefinition {
                pattern: "user.login".to_string(),
                pattern_type: PatternType::Exact,
                max_tokens: 5,
                window_secs: 60,
                priority: 100,
            }],
            default_policy: PolicyDefinition {
                pattern: String::new(),
                pattern_type: PatternType::Exact,
                max_tokens: 100,
                window_secs: 60,
                priority: 0,
            },
        };

        let policy = engine.find_policy("user.login");
        assert_eq!(policy.max_tokens, 5);
    }

    #[test]
    fn test_policy_prefix_match() {
        let engine = PolicyEngine {
            policies: vec![PolicyDefinition {
                pattern: "user.".to_string(),
                pattern_type: PatternType::Prefix,
                max_tokens: 100,
                window_secs: 60,
                priority: 50,
            }],
            default_policy: PolicyDefinition {
                pattern: String::new(),
                pattern_type: PatternType::Exact,
                max_tokens: 10,
                window_secs: 60,
                priority: 0,
            },
        };

        let policy = engine.find_policy("user.profile.update");
        assert_eq!(policy.max_tokens, 100);
    }

    #[test]
    fn test_policy_fallback_to_default() {
        let engine = PolicyEngine {
            policies: vec![],
            default_policy: PolicyDefinition {
                pattern: String::new(),
                pattern_type: PatternType::Exact,
                max_tokens: 10,
                window_secs: 60,
                priority: 0,
            },
        };

        let policy = engine.find_policy("unknown.resource");
        assert_eq!(policy.max_tokens, 10);
    }

    #[test]
    fn test_policy_priority() {
        let engine = PolicyEngine {
            policies: vec![
                PolicyDefinition {
                    pattern: "user.".to_string(),
                    pattern_type: PatternType::Prefix,
                    max_tokens: 100,
                    window_secs: 60,
                    priority: 50,
                },
                PolicyDefinition {
                    pattern: "user.login".to_string(),
                    pattern_type: PatternType::Exact,
                    max_tokens: 5,
                    window_secs: 60,
                    priority: 100,
                },
            ],
            default_policy: PolicyDefinition {
                pattern: String::new(),
                pattern_type: PatternType::Exact,
                max_tokens: 10,
                window_secs: 60,
                priority: 0,
            },
        };

        // Exact match with higher priority wins
        let policy = engine.find_policy("user.login");
        assert_eq!(policy.max_tokens, 5);

        // No exact match — falls through to prefix
        let policy = engine.find_policy("user.profile");
        assert_eq!(policy.max_tokens, 100);
    }

    // =========================================================================
    // Step 5: Property-based testing
    // =========================================================================

    proptest! {
        #[test]
        fn test_sliding_window_no_panic(
            max_requests in 1u32..1000u32,
            previous_requests in 0u32..1000u32,
            current_requests in 0u32..1000u32,
            tokens in 1u32..100u32,
            window_secs in 1u64..3600,
        ) {
            let base = SystemTime::UNIX_EPOCH + Duration::from_secs(1761948300);
            let clock = MockClock::new(base);
            let algo = SlidingWindow::with_clock(clock);

            let attempt = AcquireAttempt {
                tokens_to_acquire: tokens,
                max_tokens_per_window: max_requests,
                window_duration: Duration::from_secs(window_secs),
                previous_window_requests: previous_requests,
                current_window_requests: current_requests,
            };

            // Should never panic, regardless of input
            let _ = algo.try_acquire(&attempt);
        }
    }
}
