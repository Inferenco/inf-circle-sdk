use crate::contract::dto::UpdateNotificationSubscriptionBody;

/// Builder for UpdateNotificationSubscriptionRequest
pub struct UpdateNotificationSubscriptionBodyBuilder {
    notification_id: String,
    request: UpdateNotificationSubscriptionBody,
}

impl UpdateNotificationSubscriptionBodyBuilder {
    /// Create a new builder with required parameters
    ///
    /// # Arguments
    /// * `notification_id` - ID of the notification subscription to update
    pub fn new(notification_id: String) -> Self {
        Self {
            notification_id,
            request: UpdateNotificationSubscriptionBody {
                enabled: false,
                name: String::new(),
            },
        }
    }

    /// Set whether the subscription is enabled
    /// true indicates the subscription is active
    pub fn enabled(mut self, enabled: bool) -> Self {
        self.request.enabled = enabled;
        self
    }

    /// Set the name of the subscription
    pub fn name(mut self, name: String) -> Self {
        self.request.name = name;
        self
    }

    /// Get the notification ID
    pub fn get_notification_id(&self) -> &str {
        &self.notification_id
    }

    /// Build the request
    pub fn build(self) -> (String, UpdateNotificationSubscriptionBody) {
        (self.notification_id, self.request)
    }
}
