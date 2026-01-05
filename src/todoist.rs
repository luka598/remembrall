use chrono::{DateTime, NaiveDate, Utc};
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TodoistTask {
    pub project_id: String,
    pub section_id: Option<String>,
    pub labels: Vec<String>,
    pub due: Option<TodoistTaskDue>,
    pub content: String,
}

#[derive(Debug, Deserialize)]
pub struct TodoistTaskDue {
    pub date: String,
    pub string: String,
    pub is_recurring: bool,
}

#[derive(Debug, Deserialize)]
struct TodoistTasksResponse {
    results: Vec<TodoistTask>,
}

fn parse_date_to_millis(date_str: &str) -> Option<u128> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(date_str) {
        return Some(dt.timestamp_millis() as u128);
    }
    if let Ok(naive) = NaiveDate::parse_from_str(date_str, "%Y-%m-%d") {
        let dt = naive.and_hms(0, 0, 0);
        let dt_utc = DateTime::<Utc>::from_utc(dt, Utc);
        return Some(dt_utc.timestamp_millis() as u128);
    }
    None
}

pub fn fetch_tasks(api_key: &str, filter: &str) -> Vec<crate::data::Task> {
    let client = Client::new();

    let resp = client
        .get("https://api.todoist.com/api/v1/tasks/filter")
        .query(&[("query", filter)])
        .bearer_auth(api_key)
        .send()
        .unwrap()
        .text()
        .unwrap();

    let todoist_tasks: TodoistTasksResponse = serde_json::from_str(&resp).unwrap();

    let mut tasks: Vec<crate::data::Task> = todoist_tasks
        .results
        .into_iter()
        .map(|t| {
            let base_text = match &t.due {
                Some(due) => format!("[] {} @ {} ", t.content.replace("\\.", "."), due.string),
                None => format!("[] {} ", t.content),
            };
            crate::data::Task {
                text: format!("{:<50}|", base_text),
                time: t.due.and_then(|d| parse_date_to_millis(&d.date)),
            }
        })
        .collect();

    tasks.sort_by_key(|t| t.time.unwrap_or(u128::MAX));

    tasks
}
