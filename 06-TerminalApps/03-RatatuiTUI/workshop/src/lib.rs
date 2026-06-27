use ratatui::backend::Backend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Bar, BarChart, BarGroup, Block, Borders, List, ListItem, Paragraph, Row, Table};
use ratatui::{Frame, Terminal};

#[derive(Debug, Clone, PartialEq)]
pub struct PipelineMetric {
    pub name: String,
    pub rows: u64,
    pub duration_ms: u64,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LogEvent {
    pub timestamp: String,
    pub level: String,
    pub message: String,
}

pub fn build_metric_rows(metrics: &[PipelineMetric]) -> Vec<Row<'_>> {
    metrics
        .iter()
        .map(|m| {
            Row::new(vec![
                m.name.clone(),
                m.rows.to_string(),
                m.duration_ms.to_string(),
                m.status.clone(),
            ])
        })
        .collect()
}

pub fn build_bar_data(metrics: &[PipelineMetric]) -> Vec<(&'static str, u64)> {
    metrics
        .iter()
        .map(|m| {
            let name: &'static str = Box::leak(m.name.clone().into_boxed_str());
            (name, m.rows)
        })
        .collect()
}

pub fn build_log_items(events: &[LogEvent]) -> Vec<ListItem<'_>> {
    events
        .iter()
        .map(|e| {
            let line = Line::from(vec![
                Span::styled(format!("[{}] ", e.timestamp), Style::default().fg(Color::Gray)),
                Span::styled(format!("{}: ", e.level), Style::default().fg(color_for_status(&e.level.to_lowercase()))),
                Span::raw(&e.message),
            ]);
            ListItem::new(line)
        })
        .collect()
}

pub fn build_summary_paragraph(metrics: &[PipelineMetric]) -> Paragraph<'_> {
    let total_rows: u64 = metrics.iter().map(|m| m.rows).sum();
    let total_duration: u64 = metrics.iter().map(|m| m.duration_ms).sum();
    let successes = metrics.iter().filter(|m| m.status == "success").count();
    let failures = metrics.iter().filter(|m| m.status == "failed").count();
    let text = format!(
        "Total rows: {} | Duration: {}ms | Success: {} | Failed: {}",
        total_rows, total_duration, successes, failures
    );
    Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Summary"))
}

pub fn color_for_status(status: &str) -> Color {
    match status {
        "success" => Color::Green,
        "failed" => Color::Red,
        "skipped" => Color::Yellow,
        _ => Color::Gray,
    }
}

pub fn build_layout_rects(area: ratatui::layout::Rect) -> std::vec::Vec<ratatui::layout::Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(40),
            Constraint::Percentage(30),
            Constraint::Percentage(30),
        ])
        .split(area)
        .to_vec()
}

pub fn render_table(f: &mut Frame, metrics: &[PipelineMetric]) {
    let rows = build_metric_rows(metrics);
    let table = Table::new(
        rows,
        [
            Constraint::Percentage(30),
            Constraint::Percentage(20),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ],
    )
    .header(Row::new(vec!["Name", "Rows", "Duration (ms)", "Status"]))
    .block(Block::default().borders(Borders::ALL).title("Pipeline Metrics"));
    f.render_widget(table, f.area());
}

pub fn render_barchart(f: &mut Frame, metrics: &[PipelineMetric]) {
    let data = build_bar_data(metrics);
    let bars: Vec<Bar> = data
        .iter()
        .map(|(name, value)| {
            Bar::default()
                .label(Line::from(*name))
                .value(*value)
                .style(Style::default().fg(Color::Cyan))
        })
        .collect();
    let barchart = BarChart::default()
        .block(Block::default().borders(Borders::ALL).title("Rows Processed"))
        .data(BarGroup::default().bars(&bars));
    f.render_widget(barchart, f.area());
}

pub fn render_log_list(f: &mut Frame, events: &[LogEvent]) {
    let items = build_log_items(events);
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Log Events"));
    f.render_widget(list, f.area());
}

pub fn render_dashboard(f: &mut Frame, metrics: &[PipelineMetric], events: &[LogEvent]) {
    let rects = build_layout_rects(f.area());

    let rows = build_metric_rows(metrics);
    let table = Table::new(
        rows,
        [
            Constraint::Percentage(30),
            Constraint::Percentage(20),
            Constraint::Percentage(25),
            Constraint::Percentage(25),
        ],
    )
    .header(Row::new(vec!["Name", "Rows", "Duration (ms)", "Status"]))
    .block(Block::default().borders(Borders::ALL).title("Pipeline Metrics"));
    f.render_widget(table, rects[0]);

    let data = build_bar_data(metrics);
    let bars: Vec<Bar> = data
        .iter()
        .map(|(name, value)| {
            Bar::default()
                .label(Line::from(*name))
                .value(*value)
                .style(Style::default().fg(Color::Cyan))
        })
        .collect();
    let barchart = BarChart::default()
        .block(Block::default().borders(Borders::ALL).title("Rows Processed"))
        .data(BarGroup::default().bars(&bars));
    f.render_widget(barchart, rects[1]);

    let items = build_log_items(events);
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Log Events"));
    f.render_widget(list, rects[2]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::backend::TestBackend;

    fn sample_metrics() -> Vec<PipelineMetric> {
        vec![
            PipelineMetric { name: "extract".into(), rows: 1000, duration_ms: 250, status: "success".into() },
            PipelineMetric { name: "transform".into(), rows: 950, duration_ms: 100, status: "success".into() },
            PipelineMetric { name: "load".into(), rows: 0, duration_ms: 50, status: "failed".into() },
        ]
    }

    fn sample_events() -> Vec<LogEvent> {
        vec![
            LogEvent { timestamp: "10:00:00".into(), level: "INFO".into(), message: "Pipeline started".into() },
            LogEvent { timestamp: "10:00:01".into(), level: "INFO".into(), message: "Extracted 1000 rows".into() },
            LogEvent { timestamp: "10:00:02".into(), level: "ERROR".into(), message: "Load failed: connection refused".into() },
        ]
    }

    mod step_01_widgets {
        use super::*;

        #[test]
        fn test_build_metric_rows() {
            let m = sample_metrics();
            let rows = build_metric_rows(&m);
            assert_eq!(rows.len(), 3);
        }

        #[test]
        fn test_build_bar_data() {
            let m = sample_metrics();
            let data = build_bar_data(&m);
            assert_eq!(data, vec![("extract", 1000), ("transform", 950), ("load", 0)]);
        }

        #[test]
        fn test_build_log_items() {
            let e = sample_events();
            let items = build_log_items(&e);
            assert_eq!(items.len(), 3);
        }

        #[test]
        fn test_build_summary_paragraph() {
            let m = sample_metrics();
            let p = build_summary_paragraph(&m);
            let _ = p;
        }
    }

    mod step_02_styling {
        use super::*;

        #[test]
        fn test_color_for_status_success() {
            assert_eq!(color_for_status("success"), Color::Green);
        }

        #[test]
        fn test_color_for_status_failed() {
            assert_eq!(color_for_status("failed"), Color::Red);
        }

        #[test]
        fn test_color_for_status_skipped() {
            assert_eq!(color_for_status("skipped"), Color::Yellow);
        }

        #[test]
        fn test_color_for_status_unknown_is_gray() {
            assert_eq!(color_for_status("weird"), Color::Gray);
        }
    }

    mod step_03_layout {
        use super::*;

        #[test]
        fn test_build_layout_rects_splits_three_ways() {
            let area = ratatui::layout::Rect::new(0, 0, 100, 30);
            let rects = build_layout_rects(area);
            assert_eq!(rects.len(), 3);
        }
    }

    mod step_04_rendering {
        use super::*;

        #[test]
        fn test_render_table() {
            let backend = TestBackend::new(80, 20);
            let mut terminal = Terminal::new(backend).unwrap();
            let m = sample_metrics();
            terminal.draw(|f| render_table(f, &m)).unwrap();
        }

        #[test]
        fn test_render_barchart() {
            let backend = TestBackend::new(80, 20);
            let mut terminal = Terminal::new(backend).unwrap();
            let m = sample_metrics();
            terminal.draw(|f| render_barchart(f, &m)).unwrap();
        }

        #[test]
        fn test_render_log_list() {
            let backend = TestBackend::new(80, 20);
            let mut terminal = Terminal::new(backend).unwrap();
            let e = sample_events();
            terminal.draw(|f| render_log_list(f, &e)).unwrap();
        }

        #[test]
        fn test_render_dashboard_combines_three_panels() {
            let backend = TestBackend::new(120, 30);
            let mut terminal = Terminal::new(backend).unwrap();
            let m = sample_metrics();
            let e = sample_events();
            terminal.draw(|f| render_dashboard(f, &m, &e)).unwrap();
        }
    }
}
