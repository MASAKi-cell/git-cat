use git2::{Repository, Commit};
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;
use std::io::{self, Write};
use chrono::Local;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};

#[derive(Debug, Serialize, Deserialize)]
struct Cat {
    name: String,
    level: u32,
    happiness: u32,
    hunger: u32,
    last_feed: String,
    last_play: String,
}

impl Cat {
    fn new(name: String) -> Self {
        Cat {
            name,
            level: 1,
            happiness: 50,
            hunger: 50,
            last_feed: Local::now().to_rfc3339(),
            last_play: Local::now().to_rfc3339(),
        }
    }

    fn feed(&mut self) {
        self.hunger = 100;
        self.happiness += 10;
        self.last_feed = Local::now().to_rfc3339();
    }

    fn play(&mut self) {
        self.happiness += 20;
        self.hunger -= 10;
        self.last_play = Local::now().to_rfc3339();
    }

    fn check_level_up(&mut self) {
        if self.happiness >= 100 && self.hunger >= 80 {
            self.level += 1;
            self.happiness = 50;
            self.hunger = 50;
        }
    }

    fn get_cat_art(&self) -> String {
        format!(
            "       /\\_/\\\n      ( o.o )\n      > ^ <"
        )
    }
}

fn save_cat(cat: &Cat) -> io::Result<()> {
    let data = serde_json::to_string_pretty(cat)?;
    fs::write("cat_data.json", data)
}

fn load_cat() -> io::Result<Cat> {
    let data = fs::read_to_string("cat_data.json")?;
    Ok(serde_json::from_str(&data)?)
}

fn ui<B: Backend>(f: &mut Frame<B>, cat: &Cat) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(7),
            Constraint::Length(3),
            Constraint::Length(3),
        ].as_ref())
        .split(f.size());

    let title = Paragraph::new(format!("🐱 {}", cat.name))
        .style(Style::default().fg(Color::Yellow))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let cat_art = Paragraph::new(cat.get_cat_art())
        .style(Style::default().fg(Color::White))
        .alignment(ratatui::layout::Alignment::Center)
        .block(Block::default());
    f.render_widget(cat_art, chunks[1]);

    let happiness = Paragraph::new(format!("❤️ 幸せ度: {}%", cat.happiness))
        .style(Style::default().fg(Color::Red))
        .block(Block::default());
    f.render_widget(happiness, chunks[2]);

    let hunger = Paragraph::new(format!("💤 満腹度: {}%", cat.hunger))
        .style(Style::default().fg(Color::Green))
        .block(Block::default());
    f.render_widget(hunger, chunks[3]);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cat = if Path::new("cat_data.json").exists() {
        load_cat()?
    } else {
        println!("猫の名前を入力してください：");
        let mut name = String::new();
        io::stdin().read_line(&mut name)?;
        Cat::new(name.trim().to_string())
    };

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| ui(f, &cat))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('f') => {
                    cat.feed();
                    save_cat(&cat)?;
                }
                KeyCode::Char('p') => {
                    cat.play();
                    save_cat(&cat)?;
                }
                KeyCode::Char('q') => break,
                _ => {}
            }
        }

        cat.check_level_up();
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}



fn main() {
    let mut a = 1
    let mut b:i32 = 1
    const s:i32 = 1
    println!('s:', {} , a)
}