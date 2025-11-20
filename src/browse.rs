use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub title: String,
    pub author: String,
    pub description: String,
    pub feed_url: String,
    pub artwork_url: Option<String>,
    pub subscribers: u64,
}

pub struct BrowseState {
    pub search_query: String,
    pub search_results: Vec<SearchResult>,
    pub selected_index: usize,
    pub is_searching: bool,
    pub showing_defaults: bool,
}

impl BrowseState {
    pub fn new() -> Self {
        Self {
            search_query: String::new(),
            search_results: get_default_podcasts(),
            selected_index: 0,
            is_searching: false,
            showing_defaults: true,
        }
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_index < self.search_results.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    pub fn selected_result(&self) -> Option<&SearchResult> {
        self.search_results.get(self.selected_index)
    }
}

/// Fetches popular podcasts from gpodder.net toplist
pub fn get_default_podcasts() -> Vec<SearchResult> {
    // Fetch top podcasts from gpodder.net
    // Fallback to empty list if fetch fails
    fetch_top_podcasts(10).unwrap_or_default()
}

/// Fetch top podcasts from gpodder.net toplist API
pub fn fetch_top_podcasts(count: u32) -> Result<Vec<SearchResult>, String> {
    let count = count.clamp(1, 100); // API accepts 1-100
    let url = format!("https://gpodder.net/toplist/{}.json", count);

    let response = reqwest::blocking::get(&url)
        .map_err(|e| format!("Failed to fetch toplist: {}", e))?;

    let json: serde_json::Value = response.json()
        .map_err(|e| format!("Failed to parse toplist: {}", e))?;

    let results_array = json.as_array()
        .ok_or("Invalid toplist format")?;

    let results: Vec<SearchResult> = results_array.iter()
        .filter_map(|item| {
            Some(SearchResult {
                title: item["title"].as_str()?.to_string(),
                author: item["author"].as_str().unwrap_or("Unknown").to_string(),
                description: item["description"].as_str().unwrap_or("").to_string(),
                feed_url: item["url"].as_str()?.to_string(),
                artwork_url: item["logo_url"].as_str().map(String::from),
                subscribers: item["subscribers"].as_u64().unwrap_or(0),
            })
        })
        .collect();

    Ok(results)
}

/// Normalize title for deduplication (lowercase, remove extra chars)
fn normalize_title(title: &str) -> String {
    title.to_lowercase()
        .replace("podcasts", "")
        .replace("podcast", "")
        .replace("(", "")
        .replace(")", "")
        .trim()
        .to_string()
}

/// Extract hostname from URL
fn get_hostname(url: &str) -> Option<String> {
    url.split("://")
        .nth(1)?
        .split('/')
        .next()
        .map(|s| s.to_string())
}

/// Search podcasts using gpodder.net API (free, open, no auth required)
pub fn search_podcasts(query: &str) -> Result<Vec<SearchResult>, String> {
    if query.is_empty() {
        return Ok(Vec::new());
    }

    // gpodder.net search API - completely free and open
    let url = format!("https://gpodder.net/search.json?q={}",
        urlencoding::encode(query));

    let response = reqwest::blocking::get(&url)
        .map_err(|e| format!("Search failed: {}", e))?;

    let json: serde_json::Value = response.json()
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    let results_array = json.as_array()
        .ok_or("Invalid response format")?;

    let all_results: Vec<SearchResult> = results_array.iter()
        .filter_map(|item| {
            Some(SearchResult {
                title: item["title"].as_str()?.to_string(),
                author: item["author"].as_str().unwrap_or("Unknown").to_string(),
                description: item["description"].as_str().unwrap_or("").to_string(),
                feed_url: item["url"].as_str()?.to_string(),
                artwork_url: item["logo_url"].as_str().map(String::from),
                subscribers: item["subscribers"].as_u64().unwrap_or(0),
            })
        })
        .collect();

    // Deduplicate by normalized title + hostname, keeping the one with most subscribers
    let mut deduped: HashMap<(String, String), SearchResult> = HashMap::new();

    for result in all_results {
        if let Some(hostname) = get_hostname(&result.feed_url) {
            let key = (normalize_title(&result.title), hostname);

            deduped.entry(key)
                .and_modify(|existing| {
                    // Keep the one with more subscribers
                    if result.subscribers > existing.subscribers {
                        *existing = result.clone();
                    }
                })
                .or_insert(result);
        }
    }

    // Convert back to Vec and sort by subscriber count (descending)
    let mut results: Vec<SearchResult> = deduped.into_values().collect();
    results.sort_by(|a, b| b.subscribers.cmp(&a.subscribers));
    results.truncate(20); // Limit to 20 results

    Ok(results)
}
