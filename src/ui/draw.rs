use crate::{App, files::FileState};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Alignment},
    style::{Modifier, Style, Color},
    text::{Span, Spans},
    widgets::{Tabs, Block, Borders, Paragraph, Wrap, BorderType},
    Frame,
};
use unicode_width::UnicodeWidthStr;


type ColumnCounter = u16;


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
					Constraint::Length(3), Constraint::Min(0),
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
				.select(app.current_index)
				.highlight_style(
					Style::default()
						.add_modifier(Modifier::BOLD)
						.bg(Color::Black),
				);
			f.render_widget(tabs, main_chunks[0]);

			let file = app.get_current_file();

            let paragraph = Paragraph::new(file.hex_data)
                .block(create_block("HEX", Alignment::Center))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true })
                .scroll((file.scroll, 0));
<<<<<<< HEAD
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
=======
            f.render_widget(paragraph, main_chunks[1]);
		},
		FileState::FindTextInput => {
			let main_chunks = Layout::default()
				.direction(Direction::Vertical)
				.margin(2)
				.constraints([
					Constraint::Length(3), Constraint::Min(0), Constraint::Length(3),
				].as_ref())
				.split(f.size());

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
				.select(app.current_index)
				.highlight_style(
					Style::default()
						.add_modifier(Modifier::BOLD)
						.bg(Color::Black),
				);
			f.render_widget(tabs, main_chunks[0]);

			let file = app.get_current_file();

            let paragraph = Paragraph::new(file.hex_data)
                .block(create_block("HEX", Alignment::Center))
                .alignment(Alignment::Left)
                .wrap(Wrap { trim: true })
                .scroll((file.scroll, 0));
            f.render_widget(paragraph, main_chunks[1]);

			let file = app.get_current_file();

			let input_line = Paragraph::new(file.find_text.as_ref())
				.block(create_block("Find strings", Alignment::Right));
			f.render_widget(input_line, main_chunks[2]);

			f.set_cursor(
				main_chunks[2].x + file.find_text.width() as ColumnCounter + 1,
				main_chunks[2].y + 1,
			);
		},
		FileState::OpenFile => {
			
>>>>>>> dev
		},
		_ => {},                                 
	}
}

