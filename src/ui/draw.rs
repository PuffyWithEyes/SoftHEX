use crate::{files::{File, FileState}, App};
use super::get_file_from_vec;
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

	match get_file_from_vec(app).file_mode {
		FileState::Normal => {
			let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(2)
                .constraints([
                    Constraint::Percentage(65),
                    Constraint::Percentage(35)
                ].as_ref())
                .split(f.size());

            let test_file = File::new(
                &"/home/puffy/dev/SoftHEX/test/test.txt".to_string()
            );
            app.opened_files.push(test_file);

            let test_file = app.opened_files.get(0).unwrap();
            let text: Vec<String> = test_file.data.clone()
				;
			let text: String = text.into_iter().collect();

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
		_ => {},                                 
	}
}

