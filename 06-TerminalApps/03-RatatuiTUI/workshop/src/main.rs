use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use ratatui_tui_workshop::{render_dashboard, LogEvent, PipelineMetric};
use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let metrics = vec![
        PipelineMetric { name: "extract".into(), rows: 1000, duration_ms: 250, status: "success".into() },
        PipelineMetric { name: "transform".into(), rows: 950, duration_ms: 100, status: "success".into() },
        PipelineMetric { name: "load".into(), rows: 0, duration_ms: 50, status: "failed".into() },
    ];
    let events = vec![
        LogEvent { timestamp: "10:00:00".into(), level: "INFO".into(), message: "Pipeline started".into() },
        LogEvent { timestamp: "10:00:01".into(), level: "INFO".into(), message: "Extracted 1000 rows".into() },
        LogEvent { timestamp: "10:00:02".into(), level: "ERROR".into(), message: "Load failed".into() },
    ];

    let result = loop {
        terminal.draw(|f| {
            render_dashboard(f, &metrics, &events);
        })?;
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    break Ok(());
                }
            }
        }
    };

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    result
}
