use std::io::{self, stdout, Write};
use std::time::{Duration, Instant};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    style::{Attribute, Color, ResetColor, SetAttribute, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};

mod exercises;
use exercises::{Difficulty, Exercise, Phase, PhaseKind};

const BAR_WIDTH: usize = 44;

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    execute!(stdout(), cursor::Hide)?;

    let result = run_app();

    let _ = terminal::disable_raw_mode();
    let _ = execute!(
        stdout(),
        cursor::Show,
        SetAttribute(Attribute::Reset),
        Clear(ClearType::All),
        cursor::MoveTo(0, 0)
    );

    result?;
    println!("Breathe well.");
    Ok(())
}

fn run_app() -> io::Result<()> {
    let exercises = exercises::all();
    loop {
        match show_menu(&exercises)? {
            None => return Ok(()),
            Some(idx) => {
                if show_intro(&exercises[idx])? {
                    run_session(&exercises[idx])?;
                    show_complete(&exercises[idx])?;
                }
            }
        }
    }
}

fn show_menu(exercises: &[Exercise]) -> io::Result<Option<usize>> {
    let mut input = String::new();
    loop {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

        println!("\r");
        println!("  BREATH  ·  James Nestor\r");
        println!("  {}\r", "─".repeat(52));
        println!("\r");

        for (i, ex) in exercises.iter().enumerate() {
            let diff_color = match ex.difficulty {
                Difficulty::Beginner => Color::Green,
                Difficulty::Intermediate => Color::Yellow,
                Difficulty::Advanced => Color::Red,
            };
            print!("  {:<3} {:<36}", i + 1, ex.name);
            execute!(stdout, SetForegroundColor(diff_color))?;
            print!("{}", ex.difficulty.label());
            execute!(stdout, ResetColor)?;
            println!("\r");
        }

        println!("\r");
        println!("  {}\r", "─".repeat(52));
        println!("  Number + Enter to begin  ·  q to quit\r");
        print!("\r\n  > {}", input);
        stdout.flush()?;

        if let Event::Key(key) = event::read()? {
            if is_quit(&key) {
                return Ok(None);
            }
            match key.code {
                KeyCode::Enter => {
                    if let Ok(n) = input.trim().parse::<usize>() {
                        if n >= 1 && n <= exercises.len() {
                            return Ok(Some(n - 1));
                        }
                    }
                    input.clear();
                }
                KeyCode::Char(c) if c.is_ascii_digit() => {
                    input.push(c);
                }
                KeyCode::Backspace => {
                    input.pop();
                }
                _ => {}
            }
        }
    }
}

fn show_intro(ex: &Exercise) -> io::Result<bool> {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

    println!("\r");
    println!("  {}\r", ex.name);
    println!("  {}\r", "─".repeat(52));
    println!("\r");

    execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
    println!("  {}\r", ex.purpose);
    execute!(stdout, ResetColor)?;
    println!("\r");

    for line in ex.intro.lines() {
        println!("  {}\r", line);
    }

    if let Some(w) = ex.warning {
        println!("\r");
        execute!(stdout, SetForegroundColor(Color::Red))?;
        print!("  ! ");
        execute!(stdout, ResetColor)?;
        println!("{}\r", w);
    }

    println!("\r");
    println!("  {}\r", "─".repeat(52));
    println!("  Enter to begin  ·  q to go back\r");
    stdout.flush()?;

    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Enter => return Ok(true),
                _ if is_quit(&key) => return Ok(false),
                _ => {}
            }
        }
    }
}

fn run_session(ex: &Exercise) -> io::Result<()> {
    for round in 1..=ex.rounds {
        for phase in &ex.phases {
            if !run_phase(ex.name, phase, round, ex.rounds)? {
                return Ok(());
            }
        }
    }
    Ok(())
}

fn run_phase(name: &str, phase: &Phase, round: u32, total: u32) -> io::Result<bool> {
    if phase.kind == PhaseKind::Interactive {
        run_interactive(name, phase, round, total)
    } else {
        run_timed(name, phase, round, total)
    }
}

fn run_timed(name: &str, phase: &Phase, round: u32, total: u32) -> io::Result<bool> {
    let duration = Duration::from_secs_f32(phase.duration_secs);
    let start = Instant::now();

    loop {
        let elapsed = start.elapsed();
        if elapsed >= duration {
            break;
        }
        let progress = elapsed.as_secs_f32() / phase.duration_secs;
        let remaining = duration - elapsed;
        render_timed(name, phase, progress, remaining, round, total)?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if is_quit(&key) {
                    return Ok(false);
                }
            }
        }
    }
    Ok(true)
}

fn run_interactive(name: &str, phase: &Phase, round: u32, total: u32) -> io::Result<bool> {
    let start = Instant::now();
    loop {
        render_interactive(name, phase, start.elapsed(), round, total)?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter => return Ok(true),
                    _ if is_quit(&key) => return Ok(false),
                    _ => {}
                }
            }
        }
    }
}

fn render_timed(
    name: &str,
    phase: &Phase,
    progress: f32,
    remaining: Duration,
    round: u32,
    total: u32,
) -> io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

    let bar_fill = match phase.kind {
        PhaseKind::Inhale => progress,
        PhaseKind::HoldFull => 1.0,
        PhaseKind::Exhale => 1.0 - progress,
        PhaseKind::HoldEmpty => 0.0,
        PhaseKind::Interactive => 0.0,
    };

    let (label_color, bar_color) = phase_colors(phase.kind);

    println!("\r");
    println!("  {:<40}Round {}/{}\r", name, round, total);
    println!("  {}\r", "─".repeat(52));
    println!("\r");
    println!("\r");

    execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(label_color))?;
    println!("         {}\r", phase.label);
    execute!(stdout, SetAttribute(Attribute::Reset))?;

    match phase.hint {
        Some(h) => {
            execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
            println!("         {}\r", h);
            execute!(stdout, ResetColor)?;
        }
        None => {
            println!("\r");
        }
    }

    println!("\r");

    let filled = (bar_fill.clamp(0.0, 1.0) * BAR_WIDTH as f32) as usize;
    let empty = BAR_WIDTH - filled;
    execute!(stdout, SetForegroundColor(bar_color))?;
    print!("  [");
    print!("{}", "█".repeat(filled));
    execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
    print!("{}", "░".repeat(empty));
    execute!(stdout, ResetColor)?;
    println!("]  {:.1}s\r", remaining.as_secs_f32());

    println!("\r");
    println!("\r");
    execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
    println!("  q to stop\r");
    execute!(stdout, ResetColor)?;

    stdout.flush()
}

fn render_interactive(
    name: &str,
    phase: &Phase,
    elapsed: Duration,
    round: u32,
    total: u32,
) -> io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

    println!("\r");
    println!("  {:<40}Round {}/{}\r", name, round, total);
    println!("  {}\r", "─".repeat(52));
    println!("\r");
    println!("\r");

    execute!(stdout, SetAttribute(Attribute::Bold), SetForegroundColor(Color::Yellow))?;
    println!("         {}\r", phase.label);
    execute!(stdout, SetAttribute(Attribute::Reset))?;

    if let Some(h) = phase.hint {
        println!("\r");
        println!("         {}\r", h);
    }

    println!("\r");
    execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
    println!("         {}s elapsed\r", elapsed.as_secs());
    execute!(stdout, ResetColor)?;

    println!("\r");
    println!("\r");
    execute!(stdout, SetForegroundColor(Color::DarkGrey))?;
    println!("  Enter to continue  ·  q to stop\r");
    execute!(stdout, ResetColor)?;

    stdout.flush()
}

fn show_complete(ex: &Exercise) -> io::Result<()> {
    let mut stdout = stdout();
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

    println!("\r");
    println!("  {}\r", ex.name);
    println!("  {}\r", "─".repeat(52));
    println!("\r");

    execute!(stdout, SetForegroundColor(Color::Green))?;
    println!("  Complete.\r");
    execute!(stdout, ResetColor)?;

    if let Some(note) = ex.completion_note.as_deref() {
        println!("\r");
        for line in note.lines() {
            println!("  {}\r", line);
        }
    }

    println!("\r");
    println!("  Enter to return to menu\r");
    stdout.flush()?;

    loop {
        if let Event::Key(key) = event::read()? {
            if matches!(key.code, KeyCode::Enter) || is_quit(&key) {
                break;
            }
        }
    }
    Ok(())
}

fn phase_colors(kind: PhaseKind) -> (Color, Color) {
    match kind {
        PhaseKind::Inhale => (Color::Cyan, Color::Cyan),
        PhaseKind::HoldFull => (Color::White, Color::White),
        PhaseKind::Exhale => (Color::DarkCyan, Color::DarkCyan),
        PhaseKind::HoldEmpty => (Color::DarkGrey, Color::DarkGrey),
        PhaseKind::Interactive => (Color::Yellow, Color::Yellow),
    }
}

fn is_quit(key: &KeyEvent) -> bool {
    matches!(key.code, KeyCode::Char('q') | KeyCode::Esc)
        || (key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c'))
}
