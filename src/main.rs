mod data;
mod todoist;

use crate::{
    data::{init_data, load_data, save_data},
    todoist::fetch_tasks,
};
use std::time;

pub fn format_countdown(delta_millis: i128) -> String {
    let sign = if delta_millis >= 0 { "+" } else { "-" };
    let delta_secs = delta_millis.abs() / 1000;

    const SECS_PER_MIN: i128 = 60;
    const SECS_PER_HOUR: i128 = 3600;
    const SECS_PER_DAY: i128 = 86400;
    const SECS_PER_WEEK: i128 = 604_800;

    let s = delta_secs;

    let formatted = if s >= 2 * SECS_PER_WEEK {
        let weeks = (s + SECS_PER_WEEK - 1) / SECS_PER_WEEK;
        format!("{} {} weeks", sign, weeks)
    } else if s >= 2 * SECS_PER_DAY {
        let days = (s + SECS_PER_DAY - 1) / SECS_PER_DAY;
        format!("{} {} days", sign, days)
    } else if s >= 2 * SECS_PER_HOUR {
        let hours = (s + SECS_PER_HOUR - 1) / SECS_PER_HOUR;
        format!("{} {} hours", sign, hours)
    } else {
        let minutes = (s + SECS_PER_MIN - 1) / SECS_PER_MIN;
        format!("{} {} minutes", sign, minutes)
    };

    formatted
}

fn main() {
    let action = std::env::args().nth(1).unwrap_or("print".to_string());
    let arg = std::env::args().nth(2);

    match action.as_str() {
        "init" => match arg {
            Some(api_key) => init_data(api_key),
            None => panic!("Please provide api key!"),
        },

        "print" => {
            let data = load_data();
            if data.is_none() {
                panic!("Not initialized!")
            }
            let mut data = data.unwrap();
            let now = time::SystemTime::now()
                .duration_since(time::UNIX_EPOCH)
                .unwrap()
                .as_millis();

            if (now - data.last_fetch) > data.fetch_interval {
                data.tasks.clear();

                for (filter_name, filter) in &data.filters {
                    let result = fetch_tasks(&data.api_key, filter);
                    data.tasks.insert(filter_name.clone(), result);
                }

                data.last_fetch = now;
                save_data(&data);
            }

            for (filter_name, _) in &data.filters {
                println!("{}", filter_name);
                for task in data.tasks.get(filter_name).unwrap_or(&vec![]) {
                    let delta_str = match task.time {
                        Some(x) => format_countdown(x as i128 - now as i128),
                        None => "".to_string(),
                    };

                    println!("\t- {} {}", task.text, delta_str);
                }
                println!("");
            }
        }

        _ => {
            panic!("Unknown action: {}", action);
        }
    };

    // println!("{}", resp);
}
