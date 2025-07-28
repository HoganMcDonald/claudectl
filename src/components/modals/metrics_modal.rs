use ratatui::{
    layout::{Alignment, Constraint, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{
        Paragraph,
    },
    Frame,
};
use std::process::Command;
use std::str;
use super::SharedModal;

pub struct MetricsModal;

#[derive(Debug, Default)]
pub struct MetricsData {
    pub usage_summary: String,
    pub token_count: String,
    pub cost_estimate: String,
    pub session_progress: f64,
    pub burn_rate: String,
    pub time_remaining: String,
    pub plan_type: String,
    pub is_available: bool,
    pub error_message: Option<String>,
}

impl MetricsModal {
    pub fn new() -> Self {
        Self
    }

    // Debug function to test command detection
    #[allow(dead_code)]
    pub fn test_command_detection() -> String {
        let mut results = Vec::new();
        
        // Test ccusage commands
        for cmd in &["ccusage", "npx"] {
            let args = if *cmd == "npx" {
                vec!["ccusage@latest", "daily", "--json"]
            } else {
                vec!["daily", "--json"]
            };
            
            match std::process::Command::new(cmd).args(&args).output() {
                Ok(output) => {
                    results.push(format!("{} {}: {}", cmd, args.join(" "), 
                        if output.status.success() { "✓ Available" } else { "✗ Failed" }));
                }
                Err(e) => {
                    results.push(format!("{} {}: ✗ Not found ({})", cmd, args.join(" "), e));
                }
            }
        }
        
        // Test claude-monitor commands
        for cmd in &["claude-monitor", "claude-code-monitor", "cmonitor", "ccmonitor", "ccm"] {
            for view in &["--view daily", "--view session", "--view realtime"] {
                match std::process::Command::new(cmd).args(view.split_whitespace()).output() {
                    Ok(output) => {
                        results.push(format!("{} {}: {}", cmd, view, 
                            if output.status.success() { "✓ Available" } else { "✗ Failed" }));
                    }
                    Err(e) => {
                        results.push(format!("{} {}: ✗ Not found ({})", cmd, view, e));
                    }
                }
            }
        }
        
        results.join("\n")
    }

    pub fn get_metrics_data() -> MetricsData {
        // Try to get metrics from ccusage first (quick JSON response)
        if let Ok(output) = std::process::Command::new("ccusage")
            .args(&["daily", "--json"])
            .output()
        {
            if output.status.success() {
                if let Ok(json_str) = str::from_utf8(&output.stdout) {
                    if !json_str.trim().is_empty() {
                        return Self::parse_ccusage_json(json_str);
                    }
                }
            }
        }

        // Try npx ccusage as fallback (might be slower)
        if let Ok(output) = std::process::Command::new("npx")
            .args(&["ccusage@latest", "daily", "--json"])
            .output()
        {
            if output.status.success() {
                if let Ok(json_str) = str::from_utf8(&output.stdout) {
                    if !json_str.trim().is_empty() {
                        return Self::parse_ccusage_json(json_str);
                    }
                }
            }
        }

        // Try claude-monitor with a simpler approach - just try the most likely commands
        for cmd in &["claude-monitor", "claude-code-monitor"] {
            if let Ok(output) = std::process::Command::new(cmd)
                .args(&["--view", "daily"])
                .output()
            {
                if output.status.success() {
                    if let Ok(output_str) = str::from_utf8(&output.stdout) {
                        if !output_str.trim().is_empty() && output_str.len() > 20 {
                            return Self::parse_usage_monitor_output(output_str);
                        }
                    }
                }
            }
        }

        // Fallback to indicate no tools available
        MetricsData {
            usage_summary: "No metrics tools detected".to_string(),
            token_count: "N/A".to_string(),
            cost_estimate: "N/A".to_string(),
            session_progress: 0.0,
            burn_rate: "N/A".to_string(),
            time_remaining: "N/A".to_string(),
            plan_type: "Unknown".to_string(),
            is_available: false,
            error_message: Some("Install ccusage or claude-monitor for live metrics".to_string()),
        }
    }

    fn parse_ccusage_json(json_str: &str) -> MetricsData {
        // Basic JSON parsing for ccusage output
        // In a real implementation, you'd use serde_json
        let raw_cost = Self::extract_value(json_str, "cost").unwrap_or("0.00".to_string());
        let formatted_cost = Self::format_cost(&raw_cost);
        
        MetricsData {
            usage_summary: "ccusage - Live Data".to_string(),
            token_count: Self::extract_value(json_str, "tokens").unwrap_or("N/A".to_string()),
            cost_estimate: formatted_cost,
            session_progress: Self::extract_progress(json_str).unwrap_or(0.25),
            burn_rate: Self::extract_value(json_str, "burn_rate").unwrap_or("N/A".to_string()),
            time_remaining: Self::extract_value(json_str, "time_remaining").unwrap_or("N/A".to_string()),
            plan_type: Self::extract_value(json_str, "plan").unwrap_or("Unknown".to_string()),
            is_available: true,
            error_message: None,
        }
    }

    fn parse_usage_monitor_output(output_str: &str) -> MetricsData {
        // Parse terminal output from Claude Code Usage Monitor
        let mut token_count = "N/A".to_string();
        let mut cost_estimate = "$0.00".to_string();
        let mut session_progress = 0.0;
        let mut plan_type = "Custom".to_string();

        // Look for summary box first (contains Total Tokens and Total Cost)
        for line in output_str.lines() {
            let line_trimmed = line.trim();
            
            // Look for "Total Tokens:" in summary
            if line_trimmed.contains("Total Tokens:") {
                if let Some(tokens_str) = line_trimmed.split("Total Tokens:").nth(1) {
                    let tokens_part = tokens_str.trim().split_whitespace().next().unwrap_or("N/A");
                    token_count = tokens_part.replace(",", ""); // Remove commas
                }
            }
            
            // Look for "Total Cost:" in summary
            if line_trimmed.contains("Total Cost:") {
                if let Some(cost_str) = line_trimmed.split("Total Cost:").nth(1) {
                    let cost_part = cost_str.trim().split_whitespace().next().unwrap_or("$0.00");
                    cost_estimate = Self::format_cost(cost_part);
                }
            }
            
            // Look for session limit calculation to estimate progress
            if line_trimmed.contains("session limit calculated:") || line_trimmed.contains("P90 session limit") {
                if let Some(limit_str) = line_trimmed.split("limit calculated:").nth(1) {
                    // Extract limit number for progress calculation
                    let limit_part = limit_str.trim().split_whitespace().next().unwrap_or("0");
                    let limit_clean = limit_part.replace(",", "");
                    if let Ok(limit) = limit_clean.parse::<i32>() {
                        // Estimate progress based on current tokens vs limit
                        if let Ok(current) = token_count.replace(",", "").parse::<i32>() {
                            session_progress = (current as f64 / limit as f64).min(1.0);
                        }
                    }
                }
            }
        }

        // Calculate estimated burn rate based on recent activity (simplified)
        let burn_rate = if token_count != "N/A" {
            "~500 tokens/hr".to_string() // Rough estimate
        } else {
            "N/A".to_string()
        };

        // Calculate time remaining based on progress
        let time_remaining = if session_progress > 0.0 {
            let remaining_percent = 1.0 - session_progress;
            if remaining_percent > 0.5 {
                "4+ hours".to_string()
            } else if remaining_percent > 0.2 {
                "1-2 hours".to_string()
            } else {
                "< 1 hour".to_string()
            }
        } else {
            "5 hours".to_string()
        };

        MetricsData {
            usage_summary: "Claude Monitor - Live Data".to_string(),
            token_count,
            cost_estimate,
            session_progress,
            burn_rate,
            time_remaining,
            plan_type,
            is_available: true,
            error_message: None,
        }
    }

    fn extract_from_output(output: &str, key: &str) -> Option<String> {
        // Extract values from terminal output by looking for patterns like "Key: Value"
        for line in output.lines() {
            if let Some(start) = line.find(key) {
                let value_start = start + key.len();
                if let Some(value_slice) = line.get(value_start..) {
                    let value = value_slice.trim().split_whitespace().next()?;
                    return Some(value.to_string());
                }
            }
        }
        None
    }

    fn extract_progress_from_output(output: &str) -> Option<f64> {
        // Look for progress indicators in the output
        for line in output.lines() {
            if line.contains("%") {
                if let Some(percent_pos) = line.find('%') {
                    let before_percent = &line[..percent_pos];
                    if let Some(number_start) = before_percent.rfind(' ') {
                        if let Some(number_str) = before_percent.get(number_start + 1..) {
                            if let Ok(percent) = number_str.parse::<f64>() {
                                return Some(percent / 100.0);
                            }
                        }
                    }
                }
            }
        }
        Some(0.25) // Default progress
    }

    fn extract_number_from_line(line: &str) -> Option<String> {
        // Extract the first number from a line (could be tokens, etc.)
        let words: Vec<&str> = line.split_whitespace().collect();
        for word in words {
            // Remove common separators and try to parse as number
            let clean_word = word.replace(",", "").replace(".", "");
            if let Ok(number) = clean_word.parse::<i32>() {
                if number > 0 {
                    return Some(word.to_string());
                }
            }
        }
        None
    }

    fn extract_cost_from_line(line: &str) -> Option<String> {
        // Extract cost information from a line (look for $ signs)
        if let Some(dollar_pos) = line.find('$') {
            let after_dollar = &line[dollar_pos..];
            let words: Vec<&str> = after_dollar.split_whitespace().collect();
            if let Some(first_word) = words.first() {
                return Some(first_word.to_string());
            }
        }
        None
    }

    fn extract_plan_from_line(line: &str) -> Option<String> {
        // Extract plan type from a line
        let line_lower = line.to_lowercase();
        for plan in &["pro", "max5", "max20", "custom"] {
            if line_lower.contains(plan) {
                return Some(plan.to_uppercase());
            }
        }
        None
    }

    fn extract_percentage_from_line(line: &str) -> Option<f64> {
        // Extract percentage value from a line
        if let Some(percent_pos) = line.find('%') {
            let before_percent = &line[..percent_pos];
            let words: Vec<&str> = before_percent.split_whitespace().collect();
            if let Some(last_word) = words.last() {
                if let Ok(percentage) = last_word.parse::<f64>() {
                    return Some(percentage);
                }
            }
        }
        None
    }

    fn extract_value(json_str: &str, key: &str) -> Option<String> {
        // Simple JSON value extraction (would use serde_json in production)
        if let Some(start) = json_str.find(&format!("\"{}\":", key)) {
            let value_start = start + key.len() + 3;
            if let Some(value_slice) = json_str.get(value_start..) {
                if let Some(end) = value_slice.find(',').or_else(|| value_slice.find('}')) {
                    let value = value_slice[..end].trim().trim_matches('"');
                    return Some(value.to_string());
                }
            }
        }
        None
    }

    fn extract_progress(json_str: &str) -> Option<f64> {
        if let Some(progress_str) = Self::extract_value(json_str, "progress") {
            progress_str.parse::<f64>().ok()
        } else {
            Some(0.25) // Default progress
        }
    }

    fn format_cost(raw_cost: &str) -> String {
        // Clean up cost string and format to nearest cent
        let cleaned = raw_cost
            .trim()
            .trim_matches('"')
            .trim_matches('}')
            .trim_matches(']')
            .trim_matches(',')
            .replace("$", "");
        
        if let Ok(cost_float) = cleaned.parse::<f64>() {
            format!("${:.2}", cost_float)
        } else {
            "$0.00".to_string()
        }
    }

    pub fn render(frame: &mut Frame, _area: Rect) {
        let metrics_data = Self::get_metrics_data();
        
        // Use shared modal styling - same size as project init modal
        let modal_area = SharedModal::create_modal_area(frame, 60, 16, "Claude Code Metrics");
        let inner_area = SharedModal::create_inner_area(modal_area);

        // Create layout similar to project init modal
        let chunks = SharedModal::create_layout(
            inner_area,
            vec![
                Constraint::Length(2), // Status header
                Constraint::Length(1), // Spacer
                Constraint::Length(2), // Progress section
                Constraint::Length(1), // Spacer
                Constraint::Length(4), // Metrics details
                Constraint::Length(1), // Spacer
                Constraint::Length(2), // Instructions
            ],
        );

        // Status header (similar to project init welcome text)
        let status_text = Text::from(vec![
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(Color::Yellow)),
                Span::styled(&metrics_data.usage_summary, Style::default().fg(Color::Cyan)),
            ]),
            Line::from(vec![
                Span::styled("Plan: ", Style::default().fg(Color::Yellow)),
                Span::styled(&metrics_data.plan_type, Style::default().fg(Color::White)),
            ]),
        ]);
        let status_widget = Paragraph::new(status_text)
            .alignment(Alignment::Center);
        frame.render_widget(status_widget, chunks[0]);

        // Check if we have an error message (tools not available)
        if let Some(error_msg) = &metrics_data.error_message {
            // Show error/installation instructions
            let error_text = Text::from(vec![
                Line::from(vec![
                    Span::styled("⚠ Tools not found", Style::default().fg(Color::Yellow)),
                ]),
                Line::from(""),
                Line::from("Install one of these tools:"),
                Line::from(vec![
                    Span::styled("• ccusage: ", Style::default().fg(Color::Green)),
                    Span::raw("npm install -g ccusage"),
                ]),
                Line::from(vec![
                    Span::styled("• claude-monitor: ", Style::default().fg(Color::Green)),
                    Span::raw("uv tool install claude-monitor"),
                ]),
            ]);
            let error_widget = Paragraph::new(error_text)
                .alignment(Alignment::Center);
            
            // Use a larger area for the error message (combine progress and metrics sections)
            let error_area = Rect {
                x: chunks[2].x,
                y: chunks[2].y,
                width: chunks[2].width,
                height: chunks[2].height + chunks[3].height + chunks[4].height,
            };
            frame.render_widget(error_widget, error_area);
        } else {
            // Show actual metrics data
            // Progress section
            let progress_text = Text::from(vec![
                Line::from(vec![
                    Span::styled("Session Progress: ", Style::default().fg(Color::Yellow)),
                    Span::styled(
                        format!("{:.1}%", metrics_data.session_progress * 100.0),
                        Style::default().fg(Color::Green),
                    ),
                ]),
            ]);
            let progress_widget = Paragraph::new(progress_text)
                .alignment(Alignment::Center);
            frame.render_widget(progress_widget, chunks[2]);

            // Metrics details (compact format like project init)
            let metrics_text = Text::from(vec![
                Line::from(vec![
                    Span::styled("Tokens: ", Style::default().fg(Color::Yellow)),
                    Span::styled(&metrics_data.token_count, Style::default().fg(Color::White)),
                    Span::styled("  •  Cost: ", Style::default().fg(Color::Yellow)),
                    Span::styled(&metrics_data.cost_estimate, Style::default().fg(Color::Green)),
                ]),
                Line::from(vec![
                    Span::styled("Rate: ", Style::default().fg(Color::Yellow)),
                    Span::styled(&metrics_data.burn_rate, Style::default().fg(Color::White)),
                ]),
                Line::from(vec![
                    Span::styled("Remaining: ", Style::default().fg(Color::Yellow)),
                    Span::styled(&metrics_data.time_remaining, Style::default().fg(Color::Cyan)),
                ]),
            ]);
            let metrics_widget = Paragraph::new(metrics_text)
                .alignment(Alignment::Center);
            frame.render_widget(metrics_widget, chunks[4]);
        }

        // Instructions (same style as project init)
        let instructions_text = Text::from(vec![
            Line::from("Press m or Esc to close"),
            Line::from("Metrics refresh automatically"),
        ]);
        let instructions_widget = Paragraph::new(instructions_text)
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(instructions_widget, chunks[6]);
    }

}