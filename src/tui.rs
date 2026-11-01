use crate::storage::{format_duration, get_dates, load_for_date, load_range};
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Cell, Row, Table},
};
use std::time::Duration;

pub struct TUIState {
    scroll: usize,
    dates: (String, String, String),
    today_apps: Vec<(String, u64)>,
    week_apps: Vec<(String, u64)>,
    month_apps: Vec<(String, u64)>,
}

impl Default for TUIState {
    fn default() -> Self {
        let (t, w, m) = get_dates();

        Self {
            scroll: 0,
            dates: (t.clone(), w.clone(), m.clone()),
            today_apps: load_range(&w, &t)
                .usage
                .into_iter()
                .map(|(k, v)| (k, v))
                .collect(),
            week_apps: load_range(&w, &t)
                .usage
                .into_iter()
                .map(|(k, v)| (k, v))
                .collect(),
            month_apps: load_range(&m, &t)
                .usage
                .into_iter()
                .map(|(k, v)| (k, v))
                .collect(),
        }
    }
}

impl TUIState {
    fn refresh(&mut self) {
        let (t, w, m) = get_dates();

        self.dates = (t.clone(), w.clone(), m.clone());
        self.today_apps = load_for_date(&t)
            .usage
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect();
        self.week_apps = load_range(&w, &t)
            .usage
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect();
        self.month_apps = load_range(&m, &t)
            .usage
            .into_iter()
            .map(|(k, v)| (k, v))
            .collect();

        let max_len = self
            .today_apps
            .len()
            .max(self.week_apps.len())
            .max(self.month_apps.len());

        self.scroll = self.scroll.min(max_len.saturating_sub(1));
    }
}

pub fn run_tui() {
    enable_raw_mode().unwrap();

    execute!(std::io::stdout(), EnterAlternateScreen).unwrap();

    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend).unwrap();
    let mut state = TUIState::default();

    loop {
        state.refresh();

        let apps0: Vec<_> = state.today_apps.clone();
        let apps1: Vec<_> = state.week_apps.clone();
        let apps2: Vec<_> = state.month_apps.clone();

        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Length(1),
                        Constraint::Min(4),
                        Constraint::Length(1),
                    ])
                    .split(f.area());
                let title = Block::default()
                    .borders(Borders::TOP)
                    .border_style(Style::new().blue())
                    .title(format!("┤ MyAppTime v{} ├", env!("CARGO_PKG_VERSION")))
                    .title_alignment(Alignment::Center);

                f.render_widget(title, chunks[0]);

                let cols = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(33),
                        Constraint::Percentage(34),
                        Constraint::Percentage(33),
                    ])
                    .split(chunks[1]);

                draw_col(f, cols[0], "┤ Today ├", &apps0, state.scroll);
                draw_col(f, cols[1], "┤ Last Week ├", &apps1, state.scroll);
                draw_col(f, cols[2], "┤ Last Month ├", &apps2, state.scroll);

                let hint = Block::default()
                    .borders(Borders::BOTTOM)
                    .border_style(Style::new().blue())
                    .title("┤ [J]/[↓] - Scroll down │ [K]/[↑] - Scroll up │ [Q] - Quit ├")
                    .title_alignment(Alignment::Center);

                f.render_widget(hint, chunks[2]);
            })
            .unwrap();

        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                match key.code {
                    KeyCode::Char('k') | KeyCode::Up => {
                        state.scroll = state.scroll.saturating_sub(1);
                    }
                    KeyCode::Char('j') | KeyCode::Down => {
                        state.scroll += 1;
                    }
                    KeyCode::Char('q') | KeyCode::Esc => {
                        disable_raw_mode().unwrap();

                        execute!(terminal.backend_mut(), LeaveAlternateScreen).unwrap();

                        terminal.show_cursor().unwrap();

                        break;
                    }
                    _ => {}
                }
            }
        }
    }
}

fn draw_col(
    f: &mut ratatui::Frame,
    area: Rect,
    title: &str,
    apps: &Vec<(String, u64)>,
    scroll: usize,
) {
    let mut apps_sorted = apps.clone();

    apps_sorted.sort_by(|a, b| b.1.cmp(&a.1));

    let max_rows = (area.height as usize).saturating_sub(3); // title + header + scrollbar
    let start = scroll.min(apps_sorted.len().saturating_sub(1));
    let end = (start + max_rows).min(apps_sorted.len());
    let header = Row::new([Cell::from("App"), Cell::from("Time")])
        .style(Style::default().bg(Color::Blue).fg(Color::Black));
    let mut rows = Vec::new();

    for (name, sec) in apps_sorted
        .iter()
        .skip(start)
        .take(end.saturating_sub(start))
    {
        rows.push(Row::new(vec![
            Cell::from(name.clone()),
            Cell::from(format_duration(*sec)),
        ]));
    }

    let total: u64 = apps_sorted.iter().map(|(_, s)| *s).sum();

    rows.push(
        Row::new([Cell::from("Total"), Cell::from(format_duration(total))])
            .style(Style::default().reversed()),
    );

    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(Style::new().blue())
        .title(title.to_string())
        .title_style(Style::default().fg(Color::Blue));
    let table = Table::new(
        rows,
        [Constraint::Percentage(50), Constraint::Percentage(50)],
    )
    .header(header)
    .block(block);

    f.render_widget(table, area);
}
