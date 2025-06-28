use log::{debug, warn};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub exclusions: Exclusions,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Exclusions {
    #[serde(default)]
    pub page_patterns: Vec<String>,
}

impl Default for Exclusions {
    fn default() -> Self {
        Self {
            page_patterns: Vec::new(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            exclusions: Exclusions::default(),
        }
    }
}

impl Config {
    /// Loads configuration from navi.toml file
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = "navi.toml";

        if !Path::new(config_path).exists() {
            debug!("navi.toml not found, using default configuration");
            return Ok(Config::default());
        }

        let config_content = fs::read_to_string(config_path)?;
        let config: Config = toml::from_str(&config_content)?;

        debug!("Loaded configuration: {:?}", config);
        Ok(config)
    }

    /// Checks if a page should be excluded based on the configured regex patterns
    pub fn should_exclude_page(&self, page_title: &str, page_url: &str) -> bool {
        for pattern in &self.exclusions.page_patterns {
            match Regex::new(pattern) {
                Ok(regex) => {
                    if regex.is_match(page_title) || regex.is_match(page_url) {
                        debug!(
                            "Page excluded by pattern '{}': title='{}', url='{}'",
                            pattern, page_title, page_url
                        );
                        return true;
                    }
                }
                Err(e) => {
                    warn!("Invalid regex pattern '{}': {}", pattern, e);
                }
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.exclusions.page_patterns.is_empty());
        assert!(!config.should_exclude_page("Test Page", "https://example.com/test"));
    }

    #[test]
    fn test_exclude_page_with_pattern() {
        let mut config = Config::default();
        config
            .exclusions
            .page_patterns
            .push("(?i).*temp.*".to_string());

        // Should exclude pages with "temp" in the title (case insensitive)
        assert!(config.should_exclude_page("My temp notes", "https://example.com/notes"));
        assert!(config.should_exclude_page("Temporary", "https://example.com/notes"));

        // Should not exclude pages without "temp"
        assert!(!config.should_exclude_page("My notes", "https://example.com/notes"));
    }

    #[test]
    fn test_exclude_page_with_url_pattern() {
        let mut config = Config::default();
        config
            .exclusions
            .page_patterns
            .push("https://.*/draft-.*".to_string());

        // Should exclude pages with "draft-" in the URL
        assert!(config.should_exclude_page("My notes", "https://example.com/draft-123"));
        assert!(config.should_exclude_page("Draft", "https://notion.so/draft-abc"));

        // Should not exclude pages without "draft-" in URL
        assert!(!config.should_exclude_page("My notes", "https://example.com/notes"));
    }

    #[test]
    fn test_invalid_regex_pattern() {
        let mut config = Config::default();
        config.exclusions.page_patterns.push("[invalid".to_string());

        // Should not panic and should not exclude any pages
        assert!(!config.should_exclude_page("Test", "https://example.com/test"));
    }

    #[test]
    fn test_exclude_page_title_exact_match() {
        let mut config = Config::default();
        config
            .exclusions
            .page_patterns
            .push("^My Special Page$".to_string());
        // Should exclude the page with the exact title
        assert!(config.should_exclude_page("My Special Page", "https://example.com/anything"));
        // Should not exclude other titles
        assert!(!config.should_exclude_page("My Special Page 2", "https://example.com/anything"));
    }
}
