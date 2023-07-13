use crate::{files::{File, FileState}, App};
use super::get_file_from_vec;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Modifier, Style, Color},
    text::{Span, Spans},
    widgets::{Tabs, Block, Borders, Paragraph, Wrap, BorderType},
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
			let main_chunks = Layout::default()
				.direction(Direction::Vertical)
				.margin(2)
				.constraints([
					Constraint::Length(3), Constraint::Min(0)
				].as_ref())
				.split(f.size());
			
			let into_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(65),
                    Constraint::Percentage(35)
                ].as_ref())
                .split(main_chunks[1]);

			let titles = app
				.tabs_titles
				.iter()
				.map(|t| {
					Spans::from(vec![
						Span::styled(t, Style::default().fg(Color::Yellow)),
					])
				})
				.collect();
			
			let tabs = Tabs::new(titles)
				.block(Block::default().borders(Borders::ALL).title("Tabs"))
				.select(app.tabs_indexes)
				.highlight_style(
					Style::default()
						.add_modifier(Modifier::BOLD)
						.bg(Color::Black),
				);
			f.render_widget(tabs, main_chunks[0]);

			let file = get_file_from_vec(app);
			let text: String = file.data.clone().into_iter().collect();
            // let test_file = app.opened_files.get(0).unwrap();
            // let text: Vec<String> = test_file.data.clone();
			// let text: String = text.into_iter().collect();

            let paragraph = Paragraph::new(text.clone())
                .block(create_block("HEX", Alignment::Center))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true })
                .scroll((file.scroll, 0));
            f.render_widget(paragraph, into_chunks[0]);

            let paragraph = Paragraph::new(text.clone())
                .block(create_block("Text", Alignment::Center))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true })
                .scroll((file.scroll, 0));
            f.render_widget(paragraph, into_chunks[1]);
		},
		_ => {},                                 
	}
}

