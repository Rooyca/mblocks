use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs::{self, File};
use std::path::Path;
use battery::{Manager, State};
use notify_rust::Notification;

struct BatteryInfo {
    state: State,
    percentage: f32,
}

impl Display for BatteryInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let percentage = (self.percentage * 100.0).round() as u32;
        let emoji = match percentage {
            90..=100 => "󰂂",
            80..=89 => "󰂁",
            70..=79 => "󰂀",
            60..=69 => "󰁿",
            50..=59 => "󰁾",
            40..=49 => "󰁽",
            30..=39 => "󰁼",
            20..=29 => "󰁻",
            _ => "󰂃",
        };
        let status_emoji = if self.state == State::Discharging {
            emoji
        } else {
            ""
        };
        write!(f, "{} {}%", status_emoji, percentage)
    }
}

fn send_notification(title: &str, message: &str, urgency: notify_rust::Urgency, file_path: &str) {
    if !Path::new(file_path).exists() {
        Notification::new()
            .summary(title)
            .body(message)
            .urgency(urgency)
            .show()
            .expect("Failed to send notification");
        File::create(file_path).expect("Failed to create notification file");
    }
}

fn remove_notification_files() {
    let _ = fs::remove_file("/tmp/battery_warning");
    let _ = fs::remove_file("/tmp/battery_notification");
    let _ = fs::remove_file("/tmp/battery_full");
}

fn handle_notifications(battery_info: &BatteryInfo) {
    let percentage = (battery_info.percentage * 100.0).round() as u32;
    let status = battery_info.state;

    if status == State::Discharging {
        if percentage < 11 {
            send_notification("Battery Low", "Battery below 10%", notify_rust::Urgency::Critical, "/tmp/battery_warning");
        } else if percentage < 21 {
            send_notification("Battery Low", "Battery below 20%", notify_rust::Urgency::Normal, "/tmp/battery_notification");
        } else {
            remove_notification_files();
        }
    } else {
        remove_notification_files();
        if percentage >= 98 {
            send_notification("Battery Full", "Battery at 98%", notify_rust::Urgency::Normal, "/tmp/battery_full");
        } else {
            let _ = fs::remove_file("/tmp/battery_full");
        }
    }
}

pub fn battery_info() -> Result<Box<dyn Display>, Box<dyn Error>> {
    let manager = Manager::new()?;
    let mut batteries = manager.batteries()?;
    let battery = batteries.next().ok_or("No batteries found")??;

    let battery_info = BatteryInfo {
        state: battery.state(),
        percentage: battery.state_of_charge().value,
    };

    Ok(Box::new(battery_info))
}

pub fn battery_info_noty() -> Result<Box<dyn Display>, Box<dyn Error>> {
    let manager = Manager::new()?;
    let mut batteries = manager.batteries()?;
    let battery = batteries.next().ok_or("No batteries found")??;

    let battery_info = BatteryInfo {
        state: battery.state(),
        percentage: battery.state_of_charge().value,
    };

    handle_notifications(&battery_info);

    Ok(Box::new(battery_info))
}
