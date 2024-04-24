use std::{env, io, process::exit};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

use chrklst::{load, tui, ChecklistError};

fn main() {
    if let Err(err) = inner_main() {
        eprintln!("Error: {}", err);
        exit(1);
    }
    exit(0);
}

fn inner_main() -> Result<(), ChecklistError> {
    let cmd = choose_command()?;
    match cmd {
        Command::Help => print_help(),
        Command::Version => print_version(),
        Command::Directory => print_directory()?,
        Command::List => print_checklists()?,
        Command::Execute(name) => present_checklist(&name)?,
    };
    Ok(())
}

fn print_help() {
    println!(
        r#"
{bin} {version}
{authors}

{bin} presents the non-empty lines of a textfile to you one at a time
in a distraction-free TUI.

USAGE:

    {bin}               List available checklists
    {bin} <checklist>   Start presenting the given checklist
    {bin} -l
    {bin} -h            Print this help message
    {bin} --help
    {bin} -d            Print the full path of the directory where checklists are stored
    {bin} --directory
    {bin} -v            Print the version
    {bin} --version
"#,
        bin = env!("CARGO_PKG_NAME"),
        version = env!("CARGO_PKG_VERSION"),
        authors = env!("CARGO_PKG_AUTHORS"),
    );
}

fn print_version() {
    println!("{}", env!("CARGO_PKG_VERSION"));
}

fn print_directory() -> Result<(), ChecklistError> {
    let path = load::checklist_dir()?;
    let pathstr = path.to_string_lossy();
    println!("{}", pathstr);
    Ok(())
}

fn print_checklists() -> Result<(), ChecklistError> {
    let lists = load::get_checklists()?;
    if !lists.is_empty() {
        for list in lists {
            println!("{}", list);
        }
    } else {
        eprint!("No checklists");
    };
    Ok(())
}

fn present_checklist(name: &str) -> Result<(), ChecklistError> {
    let checklist = load::get_checklist(name)?;
    run(&mut ChecklistApp::with_steps(checklist)).map_err(ChecklistError::from)
}

#[derive(Debug)]
enum Command {
    Help,
    Version,
    Directory,
    List,
    Execute(String),
}

fn choose_command() -> Result<Command, ChecklistError> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        return Ok(Command::List);
    }
    if args.len() == 2 {
        match args[1].as_str() {
            "-d" | "--directory" => Ok(Command::Directory),
            "-h" | "--help" => Ok(Command::Help),
            "-v" | "--version" => Ok(Command::Version),
            "-l" | "--list" => Ok(Command::List),
            name => Ok(Command::Execute(name.to_owned())),
        }
    } else {
        Err(ChecklistError::from("Invalid arguments".to_string()))
    }
}

fn run(app: &mut ChecklistApp) -> io::Result<()> {
    let mut terminal = tui::init()?;
    let app_result = app.run(&mut terminal);
    tui::restore()?;
    app_result
}

#[derive(Debug, Default)]
pub struct ChecklistApp {
    steps: Vec<String>,
    exit: bool,
}

impl ChecklistApp {
    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    pub fn with_steps<T>(steps: Vec<T>) -> Self
    where
        T: ToString,
    {
        ChecklistApp {
            exit: false,
            steps: steps.iter().rev().map(|i| i.to_string()).collect(),
        }
    }

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char(' ') => self.next_step(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn next_step(&mut self) {
        self.steps.pop();
    }

    fn message(&self) -> Text {
        match self.steps.last() {
            Some(s) => s.clone().into(),
            None => "All done :)".to_owned().green().into(),
        }
    }
}

impl Widget for &ChecklistApp {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Title::from(" Checklist ".bold());
        let instructions = Title::from(Line::from(vec![
            " Next ".into(),
            "<Space/Enter>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));
        let block = Block::default()
            .title(title.alignment(Alignment::Center))
            .title(
                instructions
                    .alignment(Alignment::Center)
                    .position(Position::Bottom),
            )
            .borders(Borders::ALL)
            .border_set(border::THICK);

        let counter_text = self.message();

        Paragraph::new(counter_text)
            .wrap(Wrap { trim: true })
            .centered()
            .block(block.padding(Padding::uniform(3)))
            .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handle_key_event() -> io::Result<()> {
        {
            let mut app = ChecklistApp::default();
            assert_eq!(app.message(), String::from("All done :)").green().into());
            app.handle_key_event(KeyCode::Char(' ').into());
            assert_eq!(app.message(), String::from("All done :)").green().into());
            app.handle_key_event(KeyCode::Enter.into());
            assert_eq!(app.message(), String::from("All done :)").green().into());
        }
        {
            let mut app = ChecklistApp::with_steps(vec!["One", "Two"]);
            assert_eq!(app.message(), String::from("One").into());
            app.next_step();
            assert_eq!(app.message(), String::from("Two").into());
            app.next_step();
            assert_eq!(app.message(), String::from("All done :)").green().into());
        }
        Ok(())
    }
}
