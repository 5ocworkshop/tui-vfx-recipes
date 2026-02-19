// <FILE>examples/demo.rs</FILE> - <DESC>Recipe browser and playback demo</DESC>
// <VERS>VERSION: 0.4.2</VERS>
// <WCTX>Clippy cleanup for demo example</WCTX>
// <CLOG>Use matches! for phase comparison in demo rendering</CLOG>

//! Recipe Demo Application
//!
//! A TUI demo that browses recipe JSON files and plays them back to showcase
//! the tui-vfx-recipes library capabilities.
//!
//! ## Key Bindings
//!
//! ### Global
//! - `Tab`: Switch focus between browser and preview panes
//! - `q`: Quit
//!
//! ### Browser (left pane)
//! - `j/k` or arrows: Navigate files
//! - `PgUp/PgDn`: Page scroll
//! - `g/G`: Go to top/bottom
//! - `a-z`: Jump to first file starting with letter
//! - `Enter` or `Right`: Open file/directory
//! - `Backspace` or `Left`: Parent directory
//! - `.`: Toggle hidden files
//!
//! ### Preview (right pane)
//! - `Space`: Pause/resume animation
//! - `r`: Restart animation
//! - `c`: Close preview
//! - `Esc`: Return focus to browser

use crossterm::{
    ExecutableCommand,
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use fast_fs::nav::{ActionResult, Browser, BrowserConfig, KeyInput};
use ratatui::{
    Frame, Terminal,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};
use std::{
    collections::VecDeque,
    io::stdout,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use tui_vfx_recipes::prelude::*;
use tui_vfx_recipes::recipe::Phase;

// =============================================================================
// FPS Counter
// =============================================================================

/// High-performance FPS counter using a rolling window of frame times.
/// Provides accurate FPS calculation with minimal overhead.
struct FpsCounter {
    /// Ring buffer of frame timestamps (most recent first)
    frame_times: VecDeque<Instant>,
    /// Window size for FPS calculation
    window_size: usize,
    /// Cached FPS value (updated periodically to reduce computation)
    cached_fps: f64,
    /// Last time FPS was recalculated
    last_calc: Instant,
    /// Minimum interval between FPS recalculations
    calc_interval: Duration,
}

impl FpsCounter {
    fn new() -> Self {
        Self {
            frame_times: VecDeque::with_capacity(120),
            window_size: 60, // Use last 60 frames for calculation
            cached_fps: 0.0,
            last_calc: Instant::now(),
            calc_interval: Duration::from_millis(100), // Recalc every 100ms
        }
    }

    /// Record a frame at the given timestamp
    fn record_frame(&mut self, now: Instant) {
        self.frame_times.push_front(now);

        // Keep only window_size + 1 frames (need pairs to calculate deltas)
        while self.frame_times.len() > self.window_size + 1 {
            self.frame_times.pop_back();
        }

        // Recalculate FPS if enough time has passed
        if now.duration_since(self.last_calc) >= self.calc_interval {
            self.cached_fps = self.calculate_fps();
            self.last_calc = now;
        }
    }

    /// Calculate FPS from frame time deltas
    fn calculate_fps(&self) -> f64 {
        if self.frame_times.len() < 2 {
            return 0.0;
        }

        // Calculate average frame time from the window
        let oldest = self.frame_times.back().unwrap();
        let newest = self.frame_times.front().unwrap();
        let elapsed = newest.duration_since(*oldest);
        let frame_count = self.frame_times.len() - 1;

        if elapsed.is_zero() || frame_count == 0 {
            return 0.0;
        }

        frame_count as f64 / elapsed.as_secs_f64()
    }

    /// Get current FPS (uses cached value for performance)
    fn fps(&self) -> f64 {
        self.cached_fps
    }

    /// Get the frame time of the most recent frame in milliseconds
    fn frame_time_ms(&self) -> f64 {
        if self.frame_times.len() < 2 {
            return 0.0;
        }
        let newest = self.frame_times.front().unwrap();
        let previous = self.frame_times.get(1).unwrap();
        newest.duration_since(*previous).as_secs_f64() * 1000.0
    }
}

// =============================================================================
// App State
// =============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FocusedPane {
    Browser,
    Preview,
}

struct App {
    focused: FocusedPane,
    browser: Browser,
    list_state: ListState,
    preview: PreviewManager,
    selected_recipe: Option<Recipe>,
    recipe_path: Option<PathBuf>,
    project_root: PathBuf,
    paused: bool,
    message: Option<String>,
    fps_counter: FpsCounter,
    /// Current frame timestamp - set once per frame for consistent timing
    frame_time: Instant,
}

impl App {
    async fn new(
        recipes_dir: &Path,
        project_root: PathBuf,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let config = BrowserConfig::default();
        let browser = Browser::at_path(recipes_dir, config).await?;

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        let now = Instant::now();
        Ok(Self {
            focused: FocusedPane::Browser,
            browser,
            list_state,
            preview: PreviewManager::new(),
            selected_recipe: None,
            recipe_path: None,
            project_root,
            paused: false,
            message: None,
            fps_counter: FpsCounter::new(),
            frame_time: now,
        })
    }

    fn load_recipe(&mut self, path: &Path) -> Result<(), RecipeError> {
        let recipe = load(path, &self.project_root)?;
        let config = recipe.config();
        let item = preview_from_recipe_config(config);

        self.preview.clear();
        self.preview.add(item, self.frame_time);

        self.selected_recipe = Some(recipe);
        self.recipe_path = Some(path.to_path_buf());
        // Keep focus on browser - user can Tab to preview if needed
        self.paused = false;
        self.message = None;

        Ok(())
    }

    fn has_preview(&self) -> bool {
        self.selected_recipe.is_some()
    }

    fn restart_animation(&mut self) {
        if let Some(ref recipe) = self.selected_recipe {
            let config = recipe.config();
            let item = preview_from_recipe_config(config);

            self.preview.clear();
            self.preview.add(item, self.frame_time);
            self.paused = false;
        }
    }

    fn tick(&mut self) {
        // Always tick animation if we have a preview and not paused
        if self.has_preview() && !self.paused {
            self.preview.tick(self.frame_time);
        }
    }

    /// Update frame timestamp and record FPS
    fn begin_frame(&mut self) {
        self.frame_time = Instant::now();
        self.fps_counter.record_frame(self.frame_time);
    }
}

// =============================================================================
// Event Handling
// =============================================================================

async fn handle_key(app: &mut App, code: KeyCode, viewport_height: usize) -> bool {
    // Global keys
    match code {
        KeyCode::Char('q') => return true,
        KeyCode::Tab => {
            // Toggle focus between panes (only if preview exists)
            if app.has_preview() {
                app.focused = match app.focused {
                    FocusedPane::Browser => FocusedPane::Preview,
                    FocusedPane::Preview => FocusedPane::Browser,
                };
            }
            return false;
        }
        _ => {}
    }

    // Pane-specific keys
    match app.focused {
        FocusedPane::Browser => handle_browser_key(app, code, viewport_height).await,
        FocusedPane::Preview => handle_preview_key(app, code),
    }
}

async fn handle_browser_key(app: &mut App, code: KeyCode, viewport_height: usize) -> bool {
    match code {
        KeyCode::Char('j') | KeyCode::Down => {
            app.browser.move_down();
            let cursor = app.browser.cursor();
            app.list_state.select(Some(cursor));
        }
        KeyCode::Char('k') | KeyCode::Up => {
            app.browser.move_up();
            let cursor = app.browser.cursor();
            app.list_state.select(Some(cursor));
        }
        KeyCode::PageDown | KeyCode::Char('d') => {
            // Page down (Ctrl+D in vim style)
            app.browser.page_down(viewport_height);
            app.list_state.select(Some(app.browser.cursor()));
        }
        KeyCode::PageUp | KeyCode::Char('u') => {
            // Page up (Ctrl+U in vim style)
            app.browser.page_up(viewport_height);
            app.list_state.select(Some(app.browser.cursor()));
        }
        KeyCode::Char('g') => {
            // Go to top (gg in vim, but we just use g for simplicity)
            app.browser.set_cursor(0);
            app.list_state.select(Some(0));
        }
        KeyCode::Char('G') => {
            // Go to bottom
            let count = app.browser.filtered_count();
            if count > 0 {
                app.browser.set_cursor(count - 1);
                app.list_state.select(Some(count - 1));
            }
        }
        KeyCode::Enter => {
            let key_input = KeyInput::Enter;
            match app.browser.handle_key(key_input).await {
                ActionResult::FileSelected(path) => {
                    // Check if it's a JSON file
                    if path.extension().is_some_and(|e| e == "json") {
                        if let Err(e) = app.load_recipe(&path) {
                            app.message = Some(format!("Error: {}", e));
                        }
                    } else {
                        app.message = Some("Not a JSON file".to_string());
                    }
                }
                ActionResult::DirectoryChanged => {
                    app.list_state.select(Some(app.browser.cursor()));
                }
                _ => {}
            }
        }
        KeyCode::Backspace | KeyCode::Left => {
            // Go to parent directory
            let _ = app.browser.handle_key(KeyInput::Backspace).await;
            app.list_state.select(Some(app.browser.cursor()));
        }
        KeyCode::Right => {
            // Enter directory or select file (same as Enter)
            let key_input = KeyInput::Enter;
            match app.browser.handle_key(key_input).await {
                ActionResult::FileSelected(path) => {
                    if path.extension().is_some_and(|e| e == "json") {
                        if let Err(e) = app.load_recipe(&path) {
                            app.message = Some(format!("Error: {}", e));
                        }
                    }
                }
                ActionResult::DirectoryChanged => {
                    app.list_state.select(Some(app.browser.cursor()));
                }
                _ => {}
            }
        }
        KeyCode::Home => {
            app.browser.set_cursor(0);
            app.list_state.select(Some(0));
        }
        KeyCode::End => {
            let count = app.browser.filtered_count();
            if count > 0 {
                app.browser.set_cursor(count - 1);
                app.list_state.select(Some(count - 1));
            }
        }
        KeyCode::Char('.') => {
            // Toggle hidden files
            app.browser.toggle_hidden();
            app.list_state.select(Some(app.browser.cursor()));
        }
        KeyCode::Char('/') => {
            // Start filter mode - for now just clear filter
            app.browser.clear_filter();
            app.list_state.select(Some(app.browser.cursor()));
        }
        KeyCode::Char(c)
            if c.is_alphabetic()
                && c.is_lowercase()
                && c != 'q'
                && c != 'j'
                && c != 'k'
                && c != 'g'
                && c != 'u'
                && c != 'd'
                && c != 'r' =>
        {
            // Jump to first file starting with this character
            if app.browser.jump_to_char(c) {
                app.list_state.select(Some(app.browser.cursor()));
            }
        }
        _ => {}
    }
    false
}

fn handle_preview_key(app: &mut App, code: KeyCode) -> bool {
    match code {
        KeyCode::Esc => {
            // Switch focus back to browser
            app.focused = FocusedPane::Browser;
        }
        KeyCode::Char(' ') => {
            app.paused = !app.paused;
        }
        KeyCode::Char('r') => {
            app.restart_animation();
        }
        KeyCode::Char('c') => {
            // Close/clear the preview
            app.selected_recipe = None;
            app.recipe_path = None;
            app.preview.clear();
            app.focused = FocusedPane::Browser;
        }
        KeyCode::Char('x') => {
            // Trigger exit animation
            let id = app.preview.states().next().map(|s| s.id);
            if let Some(id) = id {
                app.preview.manager_mut().dismiss(id, app.frame_time);
            }
        }
        _ => {}
    }
    false
}

// =============================================================================
// Rendering
// =============================================================================

fn render(app: &mut App, frame: &mut Frame) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(frame.area());

    render_browser(app, frame, chunks[0]);
    render_preview_panel(app, frame, chunks[1]);
}

fn render_browser(app: &App, frame: &mut Frame, area: Rect) {
    let title = format!(" {} ", app.browser.current_path().display());
    let is_focused = app.focused == FocusedPane::Browser;
    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if is_focused {
            Color::White
        } else {
            Color::DarkGray
        }));

    // Build list items from browser files
    // Using WCAG AA compliant high-contrast colors
    let files = app.browser.files();
    let items: Vec<ListItem> = files
        .iter()
        .map(|entry| {
            let name = &entry.name;
            let style = if entry.is_dir {
                // LightBlue for directories - high contrast on dark backgrounds
                Style::default()
                    .fg(Color::LightBlue)
                    .add_modifier(Modifier::BOLD)
            } else if name.ends_with(".json") {
                // LightGreen for JSON files - high contrast
                Style::default().fg(Color::LightGreen)
            } else {
                // White for other files
                Style::default().fg(Color::White)
            };

            let prefix = if entry.is_dir { "📁 " } else { "📄 " };
            ListItem::new(Line::from(vec![
                Span::raw(prefix),
                Span::styled(name.as_str(), style),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(block)
        .highlight_style(
            // High contrast selection: white text on blue background
            Style::default()
                .fg(Color::White)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    frame.render_stateful_widget(list, area, &mut app.list_state.clone());
}

fn render_preview_panel(app: &mut App, frame: &mut Frame, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(40), Constraint::Min(5)])
        .split(area);

    render_recipe_info(app, frame, chunks[0]);
    render_animation(app, frame, chunks[1]);
}

fn render_recipe_info(app: &App, frame: &mut Frame, area: Rect) {
    let is_focused = app.focused == FocusedPane::Preview;
    let block = Block::default()
        .title(" Recipe Info ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if is_focused {
            Color::White
        } else {
            Color::DarkGray
        }));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    if let Some(ref recipe) = app.selected_recipe {
        let mut lines = vec![
            Line::from(vec![
                Span::styled("ID: ", Style::default().fg(Color::Cyan)),
                Span::raw(recipe.id()),
            ]),
            Line::from(vec![
                Span::styled("Title: ", Style::default().fg(Color::Cyan)),
                Span::raw(recipe.title()),
            ]),
            Line::from(vec![
                Span::styled("Desc: ", Style::default().fg(Color::Cyan)),
                Span::raw(recipe.metadata().description.as_ref()),
            ]),
            Line::from(""),
        ];

        // Status
        let status = if app.paused { "PAUSED" } else { "PLAYING" };
        lines.push(Line::from(vec![
            Span::styled("Status: ", Style::default().fg(Color::Green)),
            Span::styled(
                status,
                Style::default().fg(if app.paused {
                    Color::Yellow
                } else {
                    Color::Green
                }),
            ),
        ]));

        let text = Text::from(lines);
        let paragraph = Paragraph::new(text).wrap(Wrap { trim: true });
        frame.render_widget(paragraph, inner);

        // Render detailed effects list in the remaining space of the info block
        // We calculate area for effects list by taking the bottom part of the inner area
        // This is a bit of a hack, but works for this simple layout
        let effects_area = Rect {
            x: inner.x,
            y: inner.y + 6, // Skip the header lines (ID, Title, Desc, Empty, Status) + 1 margin
            width: inner.width,
            height: inner.height.saturating_sub(6),
        };

        if effects_area.height > 0 {
            let mut effects_lines = vec![Line::from(Span::styled(
                "Effects Pipeline:",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ))];

            // Get current animation phase
            let current_phase = app.preview.states().next().map(|s| s.phase);

            for target_phase in [Phase::Enter, Phase::Dwell, Phase::Exit] {
                // Determine if this phase is currently active
                let is_phase_active = matches!(
                    (target_phase, current_phase),
                    (Phase::Enter, Some(AnimationPhase::Entering))
                        | (Phase::Dwell, Some(AnimationPhase::Dwelling))
                        | (Phase::Exit, Some(AnimationPhase::Exiting))
                );

                // Header Style
                let header_style = if is_phase_active {
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::DarkGray)
                };

                // Capitalize phase name for display
                let phase_name = match target_phase {
                    Phase::Enter => "Enter",
                    Phase::Dwell => "Dwell",
                    Phase::Exit => "Exit",
                };

                effects_lines.push(Line::from(Span::styled(
                    format!("{}:", phase_name),
                    header_style,
                )));

                // Effect Item Style (Green/Bold if active, else DarkGray)
                let item_style = if is_phase_active {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default().fg(Color::DarkGray)
                };

                // 1. Style Effects
                for layer in recipe.style_layers() {
                    let style = layer.style;
                    let index = layer.index;

                    let phase_field = match target_phase {
                        Phase::Enter => "enter_effect",
                        Phase::Dwell => "dwell_effect",
                        Phase::Exit => "exit_effect",
                    };

                    if let Some(effect) = style.get(phase_field) {
                        if !effect.is_null() {
                            let type_str = effect
                                .get("type")
                                .and_then(|t| t.as_str())
                                .unwrap_or("unknown");
                            if type_str != "none" {
                                effects_lines.push(Line::from(vec![
                                    Span::styled("  Style: ", item_style),
                                    Span::styled(
                                        format!("styles[{}].{} ({})", index, phase_field, type_str),
                                        item_style,
                                    ),
                                ]));
                            }
                        }
                    }
                }

                // 2. Shaders
                for shader in recipe.shaders() {
                    if shader.phase == target_phase {
                        let type_str = shader
                            .shader
                            .get("type")
                            .and_then(|t| t.as_str())
                            .unwrap_or("unknown");
                        effects_lines.push(Line::from(vec![
                            Span::styled("  Shader: ", item_style),
                            Span::styled(type_str, item_style),
                        ]));
                    }
                }

                // 3. Masks
                for mask in recipe.masks() {
                    if mask.phase == target_phase {
                        let type_str = mask
                            .mask
                            .get("type")
                            .and_then(|t| t.as_str())
                            .unwrap_or("unknown");
                        effects_lines.push(Line::from(vec![
                            Span::styled("  Mask: ", item_style),
                            Span::styled(type_str, item_style),
                        ]));
                    }
                }

                // 4. Samplers
                for sampler in recipe.samplers() {
                    if sampler.phase == target_phase {
                        let type_str = sampler
                            .sampler
                            .get("type")
                            .and_then(|t| t.as_str())
                            .unwrap_or("unknown");
                        effects_lines.push(Line::from(vec![
                            Span::styled("  Sampler: ", item_style),
                            Span::styled(type_str, item_style),
                        ]));
                    }
                }

                // 5. Filters
                for filter in recipe.filters() {
                    if filter.phase == target_phase {
                        let type_str = filter
                            .filter
                            .get("type")
                            .and_then(|t| t.as_str())
                            .unwrap_or("unknown");
                        effects_lines.push(Line::from(vec![
                            Span::styled("  Filter: ", item_style),
                            Span::styled(type_str, item_style),
                        ]));
                    }
                }
            }

            // Add scrollable or truncated view if needed, but for now just render
            let effects_text = Text::from(effects_lines);
            let effects_paragraph = Paragraph::new(effects_text);
            frame.render_widget(effects_paragraph, effects_area);
        }
    } else if let Some(ref msg) = app.message {
        let text = Text::from(vec![Line::from(Span::styled(
            msg.as_str(),
            Style::default().fg(Color::Red),
        ))]);
        let paragraph = Paragraph::new(text);
        frame.render_widget(paragraph, inner);
    } else {
        let help = vec![
            Line::from("Select a .json recipe file and press Enter to preview"),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Navigation:",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(vec![
                Span::styled("j/k", Style::default().fg(Color::Cyan)),
                Span::raw("/"),
                Span::styled("arrows", Style::default().fg(Color::Cyan)),
                Span::raw(" - Move  "),
                Span::styled("PgUp/PgDn", Style::default().fg(Color::Cyan)),
                Span::raw(" - Page  "),
                Span::styled("g/G", Style::default().fg(Color::Cyan)),
                Span::raw(" - Top/bottom"),
            ]),
            Line::from(vec![
                Span::styled("Tab", Style::default().fg(Color::Cyan)),
                Span::raw(" - Switch pane  "),
                Span::styled("Esc", Style::default().fg(Color::Cyan)),
                Span::raw(" - Back to browser  "),
                Span::styled("q", Style::default().fg(Color::Cyan)),
                Span::raw(" - Quit"),
            ]),
            Line::from(vec![
                Span::styled("Space", Style::default().fg(Color::Cyan)),
                Span::raw(" - Pause  "),
                Span::styled("r", Style::default().fg(Color::Cyan)),
                Span::raw(" - Restart  "),
                Span::styled("c", Style::default().fg(Color::Cyan)),
                Span::raw(" - Close  "),
                Span::styled("x", Style::default().fg(Color::Cyan)),
                Span::raw(" - Dismiss (Exit)"),
            ]),
        ];
        let text = Text::from(help);
        let paragraph = Paragraph::new(text);
        frame.render_widget(paragraph, inner);
    }
}

fn render_animation(app: &mut App, frame: &mut Frame, area: Rect) {
    let is_focused = app.focused == FocusedPane::Preview;

    // Build title with FPS counter
    let fps = app.fps_counter.fps();
    let frame_ms = app.fps_counter.frame_time_ms();
    let fps_display = format!(" Animation Preview │ {:.1} FPS ({:.1}ms) ", fps, frame_ms);

    let block = Block::default()
        .title(fps_display)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(if is_focused {
            Color::White
        } else {
            Color::DarkGray
        }));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Render the animation if we have a preview
    if app.has_preview() {
        let buf = frame.buffer_mut();
        // Use consistent frame_time for all rendering
        app.preview.render(inner, buf, app.frame_time);
    }
}

// =============================================================================
// Main
// =============================================================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Determine paths
    let project_root = std::env::current_dir()?;
    let recipes_dir = project_root.join("recipes");

    if !recipes_dir.exists() {
        eprintln!("Error: recipes/ directory not found");
        eprintln!("Run this from the tui-vfx-recipes project root");
        return Ok(());
    }

    // Setup terminal
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(ratatui::backend::CrosstermBackend::new(stdout()))?;

    // Create app
    let mut app = App::new(&recipes_dir, project_root).await?;

    // Main loop - target ~60 FPS
    let target_frame_time = Duration::from_millis(16);

    loop {
        // Begin frame: set consistent timestamp for all operations this frame
        app.begin_frame();

        // Tick animations FIRST - uses frame_time internally
        // This ensures animation state is updated before rendering
        app.tick();

        // Render
        let term_size = terminal.size()?;
        terminal.draw(|frame| render(&mut app, frame))?;

        // Calculate viewport height for page navigation
        let viewport_height = term_size.height.saturating_sub(2) as usize;

        // Calculate remaining time in frame budget for event polling
        let frame_elapsed = app.frame_time.elapsed();
        let poll_timeout = target_frame_time.saturating_sub(frame_elapsed);

        // Handle events with remaining frame time
        if event::poll(poll_timeout)? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && handle_key(&mut app, key.code, viewport_height).await
                {
                    break;
                }
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_focused_pane_equality() {
        assert_eq!(FocusedPane::Browser, FocusedPane::Browser);
        assert_eq!(FocusedPane::Preview, FocusedPane::Preview);
        assert_ne!(FocusedPane::Browser, FocusedPane::Preview);
    }

    #[test]
    fn test_focused_pane_copy() {
        let pane = FocusedPane::Browser;
        let copied = pane;
        assert_eq!(pane, copied);
    }

    #[test]
    fn test_focus_toggle_logic() {
        // Simulate the Tab key logic
        let mut focused = FocusedPane::Browser;

        // Toggle to Preview
        focused = match focused {
            FocusedPane::Browser => FocusedPane::Preview,
            FocusedPane::Preview => FocusedPane::Browser,
        };
        assert_eq!(focused, FocusedPane::Preview);

        // Toggle back to Browser
        focused = match focused {
            FocusedPane::Browser => FocusedPane::Preview,
            FocusedPane::Preview => FocusedPane::Browser,
        };
        assert_eq!(focused, FocusedPane::Browser);
    }

    #[test]
    fn test_preview_manager_default() {
        let manager = PreviewManager::new();
        // PreviewManager should be empty by default
        assert_eq!(manager.states().count(), 0);
    }

    #[test]
    fn test_recipe_load_from_file() {
        // Test loading a real recipe file
        let project_root = std::env::current_dir().unwrap();
        let recipe_path = project_root.join("recipes/border_breathing_glow.json");

        if recipe_path.exists() {
            let result = load(&recipe_path, &project_root);
            assert!(result.is_ok(), "Failed to load recipe: {:?}", result.err());

            let recipe = result.unwrap();
            assert!(!recipe.id().is_empty());
            assert!(!recipe.title().is_empty());
        }
    }

    #[test]
    fn test_preview_from_loaded_recipe() {
        let project_root = std::env::current_dir().unwrap();
        let recipe_path = project_root.join("recipes/border_breathing_glow.json");

        if recipe_path.exists() {
            let recipe = load(&recipe_path, &project_root).unwrap();
            let config = recipe.config();
            let item = preview_from_recipe_config(config);

            // PreviewItem should have been created
            // We can't easily inspect its internals, but creation shouldn't panic
            let _ = item;
        }
    }

    #[test]
    fn test_recipe_effects_iteration() {
        let project_root = std::env::current_dir().unwrap();
        let recipe_path = project_root.join("recipes/border_breathing_glow.json");

        if recipe_path.exists() {
            let recipe = load(&recipe_path, &project_root).unwrap();

            // These should not panic and should return iterators
            let shader_count = recipe.shaders().count();
            let mask_count = recipe.masks().count();
            let sampler_count = recipe.samplers().count();
            let filter_count = recipe.filters().count();

            // Verify iteration works - at least one effect type should exist
            // in a real recipe (or total count is valid)
            let total = shader_count + mask_count + sampler_count + filter_count;
            // Just ensure we can compute and use the counts
            let _ = total;
        }
    }
}

// <FILE>examples/demo.rs</FILE> - <DESC>Recipe browser and playback demo</DESC>
// <VERS>END OF VERSION: 0.4.2</VERS>
