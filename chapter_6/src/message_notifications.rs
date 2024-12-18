// Define different types of messages with their specific data requirements
#[derive(Debug)]
enum MessageType {
    Email { subject: String, from: String }, // Email requires subject and sender
    SMS { from: String },                    // SMS just needs sender
    Push { app: String, title: String },     // Push needs app name and title
}

// Define priority levels for messages
#[derive(Debug)]
enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

// Combine message type and content into a complete notification
#[derive(Debug)]
struct Notification {
    message_type: MessageType,
    content: String,
    priority: Priority,
}

fn handle_notification(notification: &Notification) {
    // First determine visual priority marker based on importance
    let priority_marker = match notification.priority {
        Priority::Urgent => "🚨 URGENT: ", // Emergency/critical
        Priority::High => "❗ ",           // Important
        Priority::Medium => "● ",          // Normal
        Priority::Low => "○ ",             // Non-urgent
    };

    // Handle different message types with their specific formatting needs
    match &notification.message_type {
        // Email notifications show subject and sender
        MessageType::Email { subject, from } => {
            println!("{}New Email", priority_marker);
            println!("From: {}", from);
            println!("Subject: {}", subject);
            println!("Message: {}", notification.content);
        }
        // SMS notifications emphasize the sender
        MessageType::SMS { from } => {
            println!("{}New SMS from {}", priority_marker, from);
            println!("Message: {}", notification.content);
        }
        // Push notifications highlight the sending app
        MessageType::Push { app, title } => {
            println!("{}New Push Notification from {}", priority_marker, app);
            println!("Title: {}", title);
            println!("Message: {}", notification.content);
        }
    }
}

fn main() {
    // Create test cases for different types of notifications
    let notifications = vec![
        // High priority email example
        Notification {
            message_type: MessageType::Email {
                subject: String::from("Meeting Reminder"),
                from: String::from("boss@company.com"),
            },
            content: String::from("Team meeting in 15 minutes"),
            priority: Priority::High,
        },
        // Low priority SMS example
        Notification {
            message_type: MessageType::SMS {
                from: String::from("+1234567890"),
            },
            content: String::from("Running 10 mins late"),
            priority: Priority::Low,
        },
        // Medium priority push notification example
        Notification {
            message_type: MessageType::Push {
                app: String::from("Calendar"),
                title: String::from("Upcoming Event"),
            },
            content: String::from("Dentist appointment tomorrow at 2 PM"),
            priority: Priority::Medium,
        },
    ];

    // Process each notification
    for notification in notifications {
        handle_notification(&notification);
        println!("------------------------"); // Visual separator
    }
}
