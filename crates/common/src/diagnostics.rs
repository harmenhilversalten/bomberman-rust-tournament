use std::collections::HashMap;

use serde::Serialize;

pub struct HealthChecker {
    components: HashMap<String, Box<dyn HealthCheck>>,
}

impl Default for HealthChecker {
    fn default() -> Self {
        Self::new()
    }
}

pub trait HealthCheck: Send + Sync {
    fn name(&self) -> &str;
    fn check_health(&self) -> HealthStatus;
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub struct HealthStatus {
    pub component: String,
    pub status: Status,
    pub details: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub enum Status {
    Healthy,
    Degraded,
    Unhealthy,
}

impl HealthChecker {
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
        }
    }

    pub fn register_check(&mut self, name: String, check: Box<dyn HealthCheck>) {
        self.components.insert(name, check);
    }

    pub fn check_all(&self) -> Vec<HealthStatus> {
        self.components.values().map(|c| c.check_health()).collect()
    }

    pub fn overall_health(&self) -> Status {
        let statuses = self.check_all();
        if statuses
            .iter()
            .any(|s| matches!(s.status, Status::Unhealthy))
        {
            Status::Unhealthy
        } else if statuses
            .iter()
            .any(|s| matches!(s.status, Status::Degraded))
        {
            Status::Degraded
        } else {
            Status::Healthy
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct DummyCheck(Status);

    impl HealthCheck for DummyCheck {
        fn name(&self) -> &str {
            "dummy"
        }

        fn check_health(&self) -> HealthStatus {
            HealthStatus {
                component: self.name().to_string(),
                status: self.0.clone(),
                details: None,
                timestamp: chrono::Utc::now(),
            }
        }
    }

    #[test]
    fn overall_health_reports_worst_status() {
        let mut checker = HealthChecker::new();
        checker.register_check("a".into(), Box::new(DummyCheck(Status::Healthy)));
        checker.register_check("b".into(), Box::new(DummyCheck(Status::Degraded)));
        assert_eq!(checker.overall_health(), Status::Degraded);
        checker.register_check("c".into(), Box::new(DummyCheck(Status::Unhealthy)));
        assert_eq!(checker.overall_health(), Status::Unhealthy);
    }
}
