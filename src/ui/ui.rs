use crate::app::{App};
use std::io;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle};
use tui::widgets::{
    Axis, BarChart, Block, Borders, Chart, Dataset, Gauge, List, Marker, Paragraph, Row,
    SelectableList, Sparkline, Table, Tabs, Text, Widget,
};
use tui::{Frame, Terminal};

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &App) -> Result<(), io::Error> {
    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(f.size());
        Tabs::default()
            .block(Block::default().borders(Borders::ALL).title(app.title))
            .titles(&app.tabs.titles)
            .style(Style::default().fg(Color::Green))
            .highlight_style(Style::default().fg(Color::Yellow))
            .select(app.tabs.index)
            .render(&mut f, chunks[0]);
        match app.tabs.index {
            0 => draw_first_tab(&mut f, &app, chunks[1]),
            1 => draw_second_tab(&mut f, &app, chunks[1]),
            _ => {}
        };
    })
}

fn draw_first_tab<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(7)].as_ref())
        .split(area);

    draw_ssm_text(f, app, chunks[0]);
}

fn draw_ssm_text<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let text = [
        Text::styled("TODO:\n", Style::default().fg(Color::Green)),
        Text::raw("- Select a set of clients and query multiple account given a SSM path\n"),
        Text::raw("- Select a number of git-secret folders and have all updated"),
    ];
    Paragraph::new(text.iter())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Roadmap for parameters store integration")
                .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::BOLD)),
        )
        .wrap(true)
        .render(f, area);
}

fn draw_second_tab<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let chunks = Layout::default()
        .constraints([Constraint::Length(7)].as_ref())
        .split(area);

    draw_ecs_text(f, app, chunks[0]);
}

fn draw_ecs_text<B>(f: &mut Frame<B>, app: &App, area: Rect)
where
    B: Backend,
{
    let text = [
        Text::styled("TODO:\n", Style::default().fg(Color::Green)),
        Text::raw("- Call describe-service on all accounts to monitor launches\n"),
        Text::raw("- Poll describe-service to have updated information"),
    ];
    Paragraph::new(text.iter())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Roadmap for ecs integration")
                .title_style(Style::default().fg(Color::Magenta).modifier(Modifier::BOLD)),
        )
        .wrap(true)
        .render(f, area);
}
