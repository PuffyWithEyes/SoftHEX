use crate::{files::File, App, AppState};
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

    match app.app_mode {
        AppState::Normal => {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(2)
                .constraints([
                    Constraint::Percentage(65),
                    Constraint::Percentage(35)
                ].as_ref())
                .split(f.size());

            let test_file = File::new(
                &"D:\\Rust Projects\\softhex\\target\\debug\\test.txt".to_string()
            );
            app.opened_files.push(test_file);

            let test_file = app.opened_files.get(0).unwrap();
            let text: String = test_file.data.clone();

            let paragraph = Paragraph::new(text.clone())
                .block(create_block("HEX", Alignment::Center))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true })
                .scroll((test_file.scroll, 0));
            f.render_widget(paragraph, chunks[0]);

            let paragraph = Paragraph::new(text.clone())
                .block(create_block("TEXT", Alignment::Center))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true })
                .scroll((test_file.scroll, 0));
            f.render_widget(paragraph, chunks[1]);
        },
        AppState::FindInput => todo!("Сделать поиск"),
        _ => todo!("Сделать поиск по элементам и редактирование текста и hex"),
    }
}
