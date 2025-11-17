use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use std::io;
use crate::app::App;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

/// 运行应用程序主循环
pub fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| crate::ui::draw(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('q') => {
                        if app.in_save_mode {
                            app.in_save_mode = false;
                            app.in_edit_mode = false;
                            app.edit_buffer.clear();
                        } else if app.in_edit_mode {
                            app.edit_buffer.push('q');
                        } else if app.in_detail_page {
                            app.in_detail_page = false;
                        } else if app.in_detail_view {
                            app.in_detail_view = false;
                            app.in_detail_page = false;
                            app.current_group_id = None;
                        } else {
                            return Ok(());
                        }
                    }
                    _ => {
                        app.handle_key_event(key.code);
                    }
                }
            }
        }
    }
}
