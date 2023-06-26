use crate::{App, InputMode};
use crate::read_files::read_file;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph, Wrap, BorderType},
    Frame,
};


pub fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let create_block = |title, title_alignment| {
        Block::default()
            .borders(Borders::ALL)
            .title(Span::styled(
                title,
                Style::default().add_modifier(Modifier::BOLD),
            ))
            .title_alignment(title_alignment)
            .border_type(BorderType::Rounded)
    };

    match app.input_mode {
        InputMode::Normal => {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(2)
                .constraints([
                    Constraint::Percentage(65),
                    Constraint::Percentage(35)
                ].as_ref())
                .split(f.size());

            app.text = read_file(
                "D:\\Rust Projects\\softhex\\target\\debug\\test.txt".to_string()
            ).unwrap();
            app.hex = app.text.clone();

            let paragraph = Paragraph::new(app.hex.clone())
                .block(create_block("HEX", Alignment::Center))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true })
                .scroll((app.scroll, 0));
            f.render_widget(paragraph, chunks[0]);

            let paragraph = Paragraph::new(app.text.clone())
                .block(create_block("TEXT", Alignment::Center))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true })
                .scroll((app.scroll, 0));
            f.render_widget(paragraph, chunks[1]);
        },
        InputMode::FindInput => todo!("Сделать поиск"),
        _ => todo!("Сделать поиск по элементам и редактирование текста и hex"),
    }
}
