use crate::{App, files::FileState};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Modifier, Style, Color},
    text::{Span, Spans},
    widgets::{Tabs, Block, Borders, Paragraph, Wrap, BorderType},
    Frame,
};


const RED_FOR_PINK: u8 = 255_u8;
const GREEN_FOR_PINK: u8 = 192_u8;
const BLUE_FOR_PINK: u8 = 203_u8;


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

	match app.get_current_file().file_mode {
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
						Span::styled(t, Style::default().fg(Color::Rgb(
							RED_FOR_PINK,
							GREEN_FOR_PINK,
							BLUE_FOR_PINK,
						))),
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

			let file = app.get_current_file();
			let text: String = file.data.clone().into_iter().collect();

            let paragraph = Paragraph::new(text.clone())
                .block(create_block("HEX", Alignment::Center))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true })
                .scroll((file.scroll, 0));
            f.render_widget(paragraph, into_chunks[0]);
                .scroll((file.scroll, 0));
            f.render_widget(paragraph, into_chunks[0]);

            let paragraph = Paragraph::new(text.clone())
                .block(create_block("Text", Alignment::Center))
                .block(create_block("Text", Alignment::Center))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true })
                .scroll((file.scroll, 0));
            f.render_widget(paragraph, into_chunks[1]);
                .scroll((file.scroll, 0));
            f.render_widget(paragraph, into_chunks[1]);
		},
		_ => {},                                 
	}
}

