# 🦀 Ratatui Terminal Dashboards — Python to Rust Workshop

*Subtitle: Build interactive terminal UIs for monitoring data pipelines.*

> **Test-driven approach**: This project includes a Cargo project with progressive
> unit tests. Each function in `src/lib.rs` starts as a `todo!()` stub. As you
> follow each section, replace `todo!()` with real code and run `cargo test` to
> watch the pass count grow. Your goal: **all 13 tests pass**.

---

## Why Ratatui for Data Pipeline UIs?

**Python pain:** You want a live dashboard for a data pipeline (rows processed per second, error log, status by stage), but `curses` is hostile, `rich` is great for static output but weak for live redraws, and `textual` is heavyweight. You end up writing to stdout with `print` and praying.

**Rust fix:** `ratatui` is a Rust port of the original Go `tview` library. It uses an immediate-mode rendering model — every frame is a complete redraw — which is perfect for live dashboards. Build a tree of widgets, render to a `Frame`, repeat on every event:

```rust
terminal.draw(|f| {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
        .split(f.area());
    f.render_widget(header, chunks[0]);
    f.render_widget(table, chunks[1]);
})?;
```

This project builds a 3-panel dashboard: header (summary paragraph), body (table of stage metrics), footer (barchart of rows processed per stage).

## At a Glance

| # | Concept | Rust | Python | Why it matters |
|---|---------|------|--------|----------------|
| 1 | TUI framework | `ratatui` | `rich`, `textual` | Immediate-mode terminal UI |
| 2 | Cross-platform backend | `crossterm` | `prompt_toolkit` | Windows/Linux/macOS terminals |
| 3 | Layouts | `Layout::split` | CSS flexbox-ish | Responsive panel arrangement |
| 4 | Widgets | `Table`, `BarChart`, `List`, `Paragraph` | `rich.table`, `rich.bar` | Composable, pre-built UI parts |
| 5 | Styling | `Style`, `Color`, `Modifier` | `rich.style` | Colors, bold, italic, underline |
| 6 | Test backend | `TestBackend` | n/a (textual has `Pilot`) | Unit-test the rendering |
| 7 | Event loop | `crossterm::event::poll` | `textual.on_event` | Async, non-blocking key input |
| 8 | Raw mode | `enable_raw_mode` | terminal-only | Take over the terminal |

---

## Table of Contents
1. [Introduction](#1-introduction)
2. [Prerequisites](#2-prerequisites)
3. [Concept: Immediate-Mode TUI and `Frame`](#3-concept-immediate-mode-tui-and-frame)
4. [Concept: Widgets (Table, BarChart, List, Paragraph)](#4-concept-widgets-table-barchart-list-paragraph)
5. [Concept: Layout with Constraints](#5-concept-layout-with-constraints)
6. [Concept: Styling and Colors](#6-concept-styling-and-colors)
7. [Concept: TestBackend for Unit Tests](#7-concept-testbackend-for-unit-tests)
8. [Concept: The Event Loop](#8-concept-the-event-loop)
9. [Putting It All Together](#9-putting-it-all-together)
10. [Complete Code Reference](#10-complete-code-reference)
11. [Summary](#11-summary)

## 1. Introduction

`ratatui` (formerly `tui-rs`) is the standard for terminal UIs in Rust. Used in production at:
- **GitHub** (the `gh` CLI's interactive mode)
- **Spotify** (backstage tooling)
- **Bevy** (engine debugging tools)
- **Helix editor** (TUI text editor)
- **Lazygit** (the famous Git TUI)

**Python to Rust:** Python has `rich`, `textual`, and `urwid`, all of which work fine. The Rust ecosystem's advantage is **type-safe widget composition** — a `Table` knows its columns at compile time, so a header/row mismatch is a build error.

**Data-engineering motivation:** When you run a long pipeline (hours, days), a live TUI dashboard is more useful than a log file. It shows status at a glance, scrolls the error log, and doesn't require a web server.

## 2. Prerequisites

- Completed [04-FileIO/04-Arrow](../04-FileIO/04-Arrow/README.md) — comfortable with progressive workshops.
- Familiar with `Result` and `Box<dyn Error>`.

## 3. Concept: Immediate-Mode TUI and `Frame`

`ratatui` uses **immediate-mode** rendering: you don't keep a tree of widgets in memory and update them. Instead, every frame you describe **what you want the terminal to look like**, and the framework handles the diff against the previous frame.

The entry point is `terminal.draw(|f| ...)`:

```rust
use ratatui::{Terminal, backend::CrosstermBackend};
let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
terminal.draw(|f| {
    // `f: &mut Frame` is your canvas
    f.render_widget(my_widget, f.area());
})?;
```

The closure receives a `&mut Frame`. The `Frame` has:
- `f.area()` — the rectangle of the entire terminal
- `f.render_widget(widget, area)` — render a widget into a sub-rectangle
- `f.set_cursor(x, y)` — position the cursor (for editable text)

**In Python (`textual`):**

```python
from textual.app import App

class MyApp(App):
    def compose(self):
        yield Header()
        yield Table()
        yield Footer()

    def on_mount(self):
        # Textual is retained-mode; widgets persist between frames
        ...
```

The retained mode is conceptually different but produces the same UI.

## 4. Concept: Widgets (Table, BarChart, List, Paragraph)

Each widget is a struct that implements `Widget`. You build them with builder methods, then call `f.render_widget(widget, area)`.

**Table** — rows and columns:

```rust
let rows = vec![
    Row::new(vec!["extract", "1000", "250ms", "success"]),
    Row::new(vec!["transform", "950", "100ms", "success"]),
];
let table = Table::new(rows, &[Constraint::Length(15), Constraint::Length(10), ...])
    .header(Row::new(vec!["Stage", "Rows", "Duration", "Status"]))
    .block(Block::default().title("Metrics").borders(Borders::ALL));
```

**BarChart** — vertical bars:

```rust
let data = vec![("extract", 1000), ("transform", 950), ("load", 0)];
let bars: Vec<Bar> = data.iter().map(|(n, v)| Bar::default().value(*v).label(*n)).collect();
let barchart = BarChart::default()
    .data(BarGroup::default().bars(&bars))
    .block(Block::default().title("Rows by Stage").borders(Borders::ALL));
```

**List** — scrollable items:

```rust
let items: Vec<ListItem> = events.iter().map(|e| ListItem::new(e.message.clone())).collect();
let list = List::new(items).block(Block::default().title("Log").borders(Borders::ALL));
```

**Paragraph** — text block:

```rust
let summary = Paragraph::new(format!("Total: {} rows in {}ms", total_rows, total_ms))
    .block(Block::default().title("Summary").borders(Borders::ALL));
```

## 5. Concept: Layout with Constraints

Layouts split a rectangle into sub-rectangles. The `Constraint` enum is the most flexible part:

```rust
use ratatui::layout::{Layout, Direction, Constraint};

let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Percentage(10),  // 10% of the height
        Constraint::Percentage(60),  // 60% of the height
        Constraint::Percentage(30),  // 30% of the height
    ])
    .split(f.area());
```

Constraint types:
- `Constraint::Percentage(u16)` — percent of the available space
- `Constraint::Length(u16)` — exact number of rows/cols
- `Constraint::Ratio(u32, u32)` — 1:2:1 style
- `Constraint::Min(u16)` — at least this much
- `Constraint::Max(u16)` — at most this much

**In CSS:** `display: flex; flex-direction: column; flex: 1 1 10%`. Same idea, different syntax.

## 6. Concept: Styling and Colors

Every widget accepts a `Style` for colors, bold, italic, etc:

```rust
use ratatui::style::{Color, Modifier, Style};

let success_style = Style::default().fg(Color::Green);
let failed_style = Style::default().fg(Color::Red).add_modifier(Modifier::BOLD);
let unknown_style = Style::default().fg(Color::Gray);
```

You apply styles to:
- Cells in a `Table` row
- Spans in a `Line` (for `Paragraph` and `ListItem`)
- Block borders
- Bar colors in `BarChart`

The `Color` enum has named variants (`Color::Red`, `Color::Green`) and RGB/Indexed constructors for fine control.

## 7. Concept: TestBackend for Unit Tests

The killer feature for test-driven development: `ratatui::backend::TestBackend`. It captures what would have been drawn to the terminal into a buffer, which you can inspect:

```rust
use ratatui::backend::TestBackend;
use ratatui::Terminal;

let backend = TestBackend::new(80, 20);
let mut terminal = Terminal::new(backend)?;
terminal.draw(|f| render_table(f, &metrics)).unwrap();
```

After the draw, you can read `terminal.backend().buffer` to verify the cells contain what you expect (e.g., a particular cell contains "extract" or has `Color::Red`).

**In Python:** `textual` has `Pilot` for testing, but it's async and harder to set up. `rich` has no built-in test backend.

## 8. Concept: The Event Loop

A TUI app has three phases:
1. **Setup:** enable raw mode, enter alternate screen
2. **Loop:** draw + handle events
3. **Teardown:** disable raw mode, leave alternate screen

```rust
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::execute;
use std::io;

enable_raw_mode()?;
execute!(io::stdout(), EnterAlternateScreen)?;

let result = loop {
    terminal.draw(|f| render_dashboard(f, &metrics, &events))?;
    if event::poll(Duration::from_millis(100))? {
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') { break Ok(()); }
        }
    }
};

disable_raw_mode()?;
execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
```

The `?` propagates any terminal error to the caller. The teardown is in a `result` so it runs even on error (use a `Drop` guard in production).

## 9. Putting It All Together

`lib.rs` is organized in four progressive steps:

1. **Step 1 (`step_01_widgets`)** — build Table rows, BarChart data, log items, summary paragraph.
2. **Step 2 (`step_02_styling`)** — `color_for_status` mapping.
3. **Step 3 (`step_03_layout`)** — split a rectangle into 3 vertical sections.
4. **Step 4 (`step_04_rendering`)** — render to a `TestBackend`.

`main.rs` ties it together: enable raw mode, loop drawing the dashboard, exit on `q`.

## 10. Complete Code Reference

See [`workshop/src/lib.rs`](workshop/src/lib.rs) and [`workshop/src/main.rs`](workshop/src/main.rs).

## 11. Summary

| Concept | Used In |
|---------|---------|
| `Table` widget | `render_table` |
| `BarChart` widget | `render_barchart` |
| `List` widget | `render_log_list` |
| `Paragraph` widget | `build_summary_paragraph` |
| `Layout` with `Constraint` | `build_layout_rects` |
| `Color` / `Style` | `color_for_status`, all render functions |
| `TestBackend` | All step_04 tests |
| `crossterm` event loop | `main.rs` |

## Further Reading

- [ratatui documentation](https://ratatui.rs/)
- [ratatui GitHub](https://github.com/ratatui/ratatui)
- [Ratatui tutorial](https://ratatui.rs/tutorials/)
- dasroot.net, "Building a data pipeline dashboard with Ratatui" (Medium, Feb 2026)
- brightcoding.dev, "TUI applications in Rust" (Sep 2025)

## Exercises

1. **Easy**: Add `color_for_level(level: &str) -> Color` that maps `"INFO" → Blue`, `"WARN" → Yellow`, `"ERROR" → Red`, and 1 test.
2. **Medium**: Add a `clear_log(events: &mut Vec<LogEvent>)` function that empties the log, and a test that asserts it works.
3. **Hard**: Add a fourth panel to the dashboard — a `Sparkline` widget that shows the last 10 row counts as a line. Hint: `ratatui::widgets::Sparkline`.
