use crate::contract::dto::UpdateEventMonitorRequest;

/// Builder for updating an event monitor request
pub struct UpdateEventMonitorBodyBuilder {
    monitor_id: String,
    is_enabled: bool,
}

impl UpdateEventMonitorBodyBuilder {
    /// Create a new builder with required parameters
    ///
    /// # Arguments
    /// * `monitor_id` - The ID of the event monitor to update
    /// * `is_enabled` - Whether the event monitor should be active (true) or inactive (false)
    pub fn new(monitor_id: String, is_enabled: bool) -> Self {
        Self {
            monitor_id,
            is_enabled,
        }
    }

    /// Build the request, returning the monitor ID and request body
    pub fn build(self) -> (String, UpdateEventMonitorRequest) {
        (
            self.monitor_id,
            UpdateEventMonitorRequest {
                is_enabled: self.is_enabled,
            },
        )
    }
}
