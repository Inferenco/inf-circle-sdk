use crate::contract::dto::{CreateNotificationSubscriptionBody, NotificationType};

/// Builder for CreateNotificationSubscriptionRequest
pub struct CreateNotificationSubscriptionBodyBuilder {
    request: CreateNotificationSubscriptionBody,
}

impl CreateNotificationSubscriptionBodyBuilder {
    /// Create a new builder with required endpoint
    pub fn new(endpoint: String) -> Self {
        Self {
            request: CreateNotificationSubscriptionBody {
                endpoint,
                notification_types: None,
            },
        }
    }

    /// Set notification types to subscribe to
    /// If not provided, the webhook will be unrestricted and receive all notification types
    pub fn notification_types(mut self, types: Vec<NotificationType>) -> Self {
        self.request.notification_types = Some(types);
        self
    }

    /// Add a single notification type
    pub fn add_notification_type(mut self, notification_type: NotificationType) -> Self {
        match &mut self.request.notification_types {
            Some(types) => types.push(notification_type),
            None => self.request.notification_types = Some(vec![notification_type]),
        }
        self
    }

    /// Build the request
    pub fn build(self) -> CreateNotificationSubscriptionBody {
        self.request
    }
}
