# Workshop: Ratatui TUI

**Goal**: Implement all functions in `src/lib.rs` to pass all 13 tests.

## Functions to Implement

### Step 1 — Widgets

#### `build_metric_rows`
- **Signature**: `pub fn build_metric_rows(metrics: &[PipelineMetric]) -> Vec<Row<'_>>`
- **Task**: Convert each `PipelineMetric` to a `Row` with 4 cells (name, rows, duration_ms, status).

#### `build_bar_data`
- **Signature**: `pub fn build_bar_data(metrics: &[PipelineMetric]) -> Vec<(&'static str, u64)>`
- **Task**: Return `Vec<(name, rows)>` for `BarChart`.

#### `build_log_items`
- **Signature**: `pub fn build_log_items(events: &[LogEvent]) -> Vec<ListItem<'_>>`
- **Task**: Convert each `LogEvent` to a `ListItem` (use `Line::from` with two spans).

#### `build_summary_paragraph`
- **Signature**: `pub fn build_summary_paragraph(metrics: &[PipelineMetric]) -> Paragraph<'_>`
- **Task**: A summary line: total rows, total duration, success/failed count.

### Step 2 — Styling

#### `color_for_status`
- **Signature**: `pub fn color_for_status(status: &str) -> Color`
- **Task**: match `"success"` → `Color::Green`, `"failed"` → `Color::Red`, `"skipped"` → `Color::Yellow`, _ → `Color::Gray`.

### Step 3 — Layout

#### `build_layout_rects`
- **Signature**: `pub fn build_layout_rects(area: ratatui::layout::Rect) -> std::vec::Vec<ratatui::layout::Rect>`
- **Task**: Split `area` into 3 vertical sections: header (10%), body (60%), footer (30%).

### Step 4 — Rendering

#### `render_table` / `render_barchart` / `render_log_list` / `render_dashboard`
- **Task**: Use `terminal.draw(|f| ...)` to render the corresponding widget inside a bordered `Block`.

## Test Modules

| Module | Tests | What It Tests |
|--------|-------|---------------|
| step_01_widgets | 4 | Build rows, bar data, log items, summary paragraph |
| step_02_styling | 4 | Status → Color mapping |
| step_03_layout | 1 | Layout splits area into 3 sections |
| step_04_rendering | 4 | Render widgets to a TestBackend |

## How to Run Tests
```bash
cargo test
```
