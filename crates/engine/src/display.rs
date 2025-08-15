//! Terminal display module for visualizing the Bomberman game.

use std::io::{self, Write};
use std::sync::{Arc, RwLock};

use state::{GameGrid, Tile, AgentState, Bomb};
use crossterm::{
    cursor,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, ClearType},
    ExecutableCommand, QueueableCommand,
};

/// Terminal-based game display.
pub struct GameDisplay {
    width: usize,
    height: usize,
}

impl GameDisplay {
    /// Create a new game display.
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    /// Initialize the terminal for game display.
    pub fn init_terminal(&self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        io::stdout()
            .execute(terminal::Clear(ClearType::All))?
            .execute(cursor::Hide)?;
        Ok(())
    }

    /// Restore the terminal to normal mode.
    pub fn restore_terminal(&self) -> io::Result<()> {
        io::stdout()
            .execute(cursor::Show)?
            .execute(ResetColor)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    /// Render the game state to the terminal.
    pub fn render(&self, grid: &Arc<RwLock<GameGrid>>) -> io::Result<()> {
        let mut stdout = io::stdout();
        
        // Clear screen and move to top
        stdout
            .queue(terminal::Clear(ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?;

        // Get game state
        let grid_lock = grid.read().unwrap();
        let snapshot = grid_lock.snapshot();
        
        // Render title
        stdout
            .queue(SetForegroundColor(Color::Yellow))?
            .queue(Print("🎮 BOMBERMAN TOURNAMENT 🎮\n"))?
            .queue(ResetColor)?
            .queue(Print("═".repeat(self.width * 2 + 4)))?
            .queue(Print("\n"))?;

        // Render top border
        stdout
            .queue(SetForegroundColor(Color::White))?
            .queue(SetBackgroundColor(Color::DarkGrey))?
            .queue(Print("  "))?;
        for _ in 0..self.width {
            stdout.queue(Print("██"))?;
        }
        stdout
            .queue(Print("  "))?
            .queue(ResetColor)?
            .queue(Print("\n"))?;

        // Render game grid
        for y in 0..self.height {
            // Left border
            stdout
                .queue(SetForegroundColor(Color::White))?
                .queue(SetBackgroundColor(Color::DarkGrey))?
                .queue(Print("██"))?
                .queue(ResetColor)?;

            // Render row
            for x in 0..self.width {
                let index = y * self.width + x;
                let tile = snapshot.tiles().get(index).copied().unwrap_or(Tile::Empty);
                
                // Check for agents at this position
                let agent_here = snapshot.agents().iter()
                    .find(|agent| agent.position.0 as usize == x && agent.position.1 as usize == y);
                
                // Check for bombs at this position
                let bomb_here = snapshot.bombs().iter()
                    .find(|bomb| bomb.position.0 as usize == x && bomb.position.1 as usize == y);

                // Render based on priority: agent > bomb > tile
                if let Some(agent) = agent_here {
                    self.render_agent(&mut stdout, agent)?;
                } else if let Some(bomb) = bomb_here {
                    self.render_bomb(&mut stdout, bomb)?;
                } else {
                    self.render_tile(&mut stdout, &tile)?;
                }
            }

            // Right border
            stdout
                .queue(SetForegroundColor(Color::White))?
                .queue(SetBackgroundColor(Color::DarkGrey))?
                .queue(Print("██"))?
                .queue(ResetColor)?
                .queue(Print("\n"))?;
        }

        // Render bottom border
        stdout
            .queue(SetForegroundColor(Color::White))?
            .queue(SetBackgroundColor(Color::DarkGrey))?
            .queue(Print("  "))?;
        for _ in 0..self.width {
            stdout.queue(Print("██"))?;
        }
        stdout
            .queue(Print("  "))?
            .queue(ResetColor)?
            .queue(Print("\n"))?;

        // Render game info
        self.render_game_info(&mut stdout, &snapshot)?;

        stdout.flush()?;
        Ok(())
    }

    /// Render a single tile.
    fn render_tile(&self, stdout: &mut io::Stdout, tile: &Tile) -> io::Result<()> {
        match tile {
            Tile::Empty => {
                stdout
                    .queue(SetBackgroundColor(Color::Green))?
                    .queue(Print("  "))?
                    .queue(ResetColor)?;
            }
            Tile::Wall => {
                stdout
                    .queue(SetBackgroundColor(Color::DarkGrey))?
                    .queue(SetForegroundColor(Color::Black))?
                    .queue(Print("██"))?
                    .queue(ResetColor)?;
            }
            Tile::SoftCrate => {
                stdout
                    .queue(SetBackgroundColor(Color::Rgb { r: 139, g: 69, b: 19 }))? // Brown color
                    .queue(SetForegroundColor(Color::Rgb { r: 160, g: 82, b: 45 }))? // Darker brown
                    .queue(Print("▓▓"))?
                    .queue(ResetColor)?;
            }
            Tile::PowerUp => {
                stdout
                    .queue(SetBackgroundColor(Color::Magenta))?
                    .queue(SetForegroundColor(Color::White))?
                    .queue(Print("⭐"))?
                    .queue(ResetColor)?;
            }
            Tile::Explosion => {
                stdout
                    .queue(SetBackgroundColor(Color::Red))?
                    .queue(SetForegroundColor(Color::Yellow))?
                    .queue(Print("💥"))?
                    .queue(ResetColor)?;
            }
        }
        Ok(())
    }

    /// Render an agent.
    fn render_agent(&self, stdout: &mut io::Stdout, agent: &AgentState) -> io::Result<()> {
        let (bg_color, fg_color, symbol) = self.get_player_style(agent.id);

        stdout
            .queue(SetBackgroundColor(bg_color))?
            .queue(SetForegroundColor(fg_color))?
            .queue(Print(symbol))?
            .queue(ResetColor)?;
        Ok(())
    }

    /// Get player style (color and symbol) for up to 100 players.
    fn get_player_style(&self, player_id: usize) -> (Color, Color, String) {
        let symbol = format!("{:02}", player_id + 1); // 01, 02, 03, ..., 99, 100
        
        let (bg_color, fg_color) = match player_id {
            // First 16 players with distinct colors
            0 => (Color::Blue, Color::White),
            1 => (Color::Red, Color::White),
            2 => (Color::Cyan, Color::Black),
            3 => (Color::Magenta, Color::White),
            4 => (Color::Green, Color::Black),
            5 => (Color::Yellow, Color::Black),
            6 => (Color::DarkBlue, Color::White),
            7 => (Color::DarkRed, Color::White),
            8 => (Color::DarkCyan, Color::White),
            9 => (Color::DarkMagenta, Color::White),
            10 => (Color::DarkGreen, Color::White),
            11 => (Color::DarkYellow, Color::Black),
            12 => (Color::Grey, Color::Black),
            13 => (Color::DarkGrey, Color::White),
            14 => (Color::White, Color::Black),
            15 => (Color::Black, Color::White),
            
            // Players 16-99 with RGB colors for variety
            _ => {
                let hue = (player_id * 137) % 360; // Golden angle for good distribution
                let r = ((hue * 255) / 360) as u8;
                let g = (((hue + 120) % 360) * 255 / 360) as u8;
                let b = (((hue + 240) % 360) * 255 / 360) as u8;
                
                // Ensure good contrast by checking brightness
                let brightness = (r as u32 + g as u32 + b as u32) / 3;
                let fg = if brightness > 128 { Color::Black } else { Color::White };
                
                (Color::Rgb { r, g, b }, fg)
            }
        };
        
        (bg_color, fg_color, symbol)
    }

    /// Render a bomb.
    fn render_bomb(&self, stdout: &mut io::Stdout, _bomb: &Bomb) -> io::Result<()> {
        stdout
            .queue(SetBackgroundColor(Color::Black))?
            .queue(SetForegroundColor(Color::Yellow))?
            .queue(Print("💣"))?
            .queue(ResetColor)?;
        Ok(())
    }

    /// Render game information.
    fn render_game_info(&self, stdout: &mut io::Stdout, snapshot: &state::SnapshotView) -> io::Result<()> {
        stdout
            .queue(Print("\n"))?
            .queue(SetForegroundColor(Color::Cyan))?
            .queue(Print("📊 GAME STATUS\n"))?
            .queue(ResetColor)?
            .queue(Print("─".repeat(30)))?
            .queue(Print("\n"))?;

        // Player info
        for (i, agent) in snapshot.agents().iter().enumerate() {
            let (bg_color, fg_color, symbol) = self.get_player_style(i);

            stdout
                .queue(SetBackgroundColor(bg_color))?
                .queue(SetForegroundColor(fg_color))?
                .queue(Print(format!(" {} ", symbol)))?
                .queue(ResetColor)?
                .queue(Print(format!("- Position: ({}, {}) ", agent.position.0, agent.position.1)))?
                .queue(Print(format!("Bombs: {} Power: {}", agent.bombs_left, agent.power)))?;
            stdout
                .queue(ResetColor)?
                .queue(Print("\n"))?;
        }

        // Bomb info
        stdout
            .queue(Print(format!("💣 Active Bombs: {}\n", snapshot.bombs().len())))?;

        // Controls
        stdout
            .queue(Print("\n"))?
            .queue(SetForegroundColor(Color::Yellow))?
            .queue(Print("🎮 CONTROLS\n"))?
            .queue(ResetColor)?
            .queue(Print("─".repeat(30)))?
            .queue(Print("\n"))?
            .queue(Print("Press 'q' to quit\n"))?
            .queue(Print("Press 'r' to restart\n"))?
            .queue(Print("Press SPACE to pause/resume\n"))?;

        Ok(())
    }
}

