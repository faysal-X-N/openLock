use anyhow::{Result, Context};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Terminal,
};
use secrecy::{SecretBox, ExposeSecret};
use shield_core::{Vault, model::Entry};
use std::{io::{self, Write}, path::PathBuf};

struct App {
    vault: Vault,
    entries: Vec<Entry>,
    state: ListState,
    should_quit: bool,
}

impl App {
    fn new(vault: Vault) -> Result<Self> {
        let entries = vault.list_entries()?;
        let mut state = ListState::default();
        if !entries.is_empty() {
            state.select(Some(0));
        }
        Ok(Self {
            vault,
            entries,
            state,
            should_quit: false,
        })
    }

    fn on_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') | KeyCode::Esc => self.should_quit = true,
            KeyCode::Down | KeyCode::Char('j') => self.next(),
            KeyCode::Up | KeyCode::Char('k') => self.previous(),
            _ => {}
        }
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.entries.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.entries.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

fn main() -> Result<()> {
    // Setup vault path
    let db_path = dirs::data_local_dir()
        .unwrap_or(PathBuf::from("."))
        .join("shield.db");

    if !db_path.exists() {
        println!("Vault not found at {:?}. Please run 'shield-cli init' first.", db_path);
        return Ok(());
    }

    // Password prompt
    let password = if let Ok(p) = std::env::var("SHIELD_PASSWORD") {
        SecretBox::new(Box::new(p))
    } else {
        print!("Enter Master Password: ");
        std::io::stdout().flush()?;
        let p = rpassword::read_password()?;
        SecretBox::new(Box::new(p))
    };

    let vault = Vault::open(&db_path, &password).context("Failed to open vault")?;
    let mut app = App::new(vault)?;

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run app loop
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err);
    }

    Ok(())
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>, app: &mut App) -> Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                app.on_key(key.code);
            }
        }

        if app.should_quit {
            return Ok(());
        }
    }
}

fn ui(f: &mut ratatui::Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
        .split(f.size());

    let items: Vec<ListItem> = app
        .entries
        .iter()
        .map(|entry| {
            let content = Line::from(Span::raw(&entry.name));
            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Entries"))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD).fg(Color::Yellow))
        .highlight_symbol("> ");

    f.render_stateful_widget(list, chunks[0], &mut app.state);

    let selected_entry = app.state.selected().and_then(|i| app.entries.get(i));

    if let Some(entry) = selected_entry {
        let text = vec![
            Line::from(vec![Span::raw("Name: "), Span::styled(&entry.name, Style::default().fg(Color::Green))]),
            Line::from(vec![Span::raw("Username: "), Span::raw(entry.username.as_deref().unwrap_or("-"))]),
            Line::from(vec![Span::raw("URL: "), Span::raw(entry.url.as_deref().unwrap_or("-"))]),
            Line::from(vec![Span::raw("Password: "), Span::styled("********", Style::default().fg(Color::Red))]),
            Line::from(vec![Span::raw("Notes: "), Span::raw(entry.notes.as_deref().unwrap_or("-"))]),
            Line::from(vec![Span::raw("Updated: "), Span::raw(entry.updated_at.to_string())]),
        ];
        
        let paragraph = Paragraph::new(text)
            .block(Block::default().borders(Borders::ALL).title("Details"));
        f.render_widget(paragraph, chunks[1]);
    } else {
        let paragraph = Paragraph::new("No entry selected")
            .block(Block::default().borders(Borders::ALL).title("Details"));
        f.render_widget(paragraph, chunks[1]);
    }
}
