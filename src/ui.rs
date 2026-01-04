use ratatui::crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::app::{App, Pages};

pub mod standard {
    use ratatui::prelude::*;
    use ratatui::layout::*;

    /// Creates a centered rect using up a certain percentage of the available rect.
    fn centered_rect(percent_x: u16, percent_y: u16, rect: Rect) -> Rect {
        // Cut the given rectangle into three vertical pieces
        let vertical_chunks = Layout::new(Direction::Vertical, [
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ]).split(rect);

        // Cuts the middle chunk of the vertical chunks into three horizontal pieces
        let horizontal_chunks = Layout::new(Direction::Horizontal, [
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ]).split(vertical_chunks[1]);

        // Returns the middle of the horizontal chunks
        horizontal_chunks[1]
    }
}



pub fn ui(frame: &mut Frame, app: &mut App) {
    // header
    let header_block = Block::new().borders(Borders::ALL);
    let header = Paragraph::new(vec![
        Line::raw("PERI"),
        Line::raw(app.current_page_name()),
    ]).block(header_block);

    // footer
    let footer_block = Block::new().borders(Borders::ALL);
    let instructions = get_instructions_for(&app.current_page);
    let instructions_height = instructions.len() as u16 + 2;
    let footer = Paragraph::new(instructions).block(footer_block);

    // The sections of the screen.
    let leaflets = Layout::new(Direction::Vertical, [
        Constraint::Length(4), // header
        Constraint::Fill(1), // body
        Constraint::Length(instructions_height), // footer
    ]).split(frame.area());

    // rendering the header and footer
    frame.render_widget(header, leaflets[0]);
    frame.render_widget(footer, leaflets[2]);

    // body
    match app.current_page {
        Pages::Launching => {
            let body = Paragraph::new("PERI");
            frame.render_widget(body, leaflets[1]);
        }

        Pages::AddingBody => {
            let body = Paragraph::new(vec![
                Line::raw(format!("Name  : {}", &app.new_body_name)),
                Line::raw(format!("Width : {}", &app.new_body_width)),
                Line::raw(format!("Height: {}", &app.new_body_height)),
            ]);

            frame.render_widget(body, leaflets[1]);
        }

        Pages::BodyView => {
            let body = Paragraph::new(app.body.summarize());
            frame.render_widget(body, leaflets[1]);
        }

        Pages::RenamingBody => {
            let body = Paragraph::new(format!("New Name: {}", &app.new_body_name));
            frame.render_widget(body, leaflets[1]);
        }

        Pages::ShowingHoleFeatureOptions => {
            let body = Paragraph::new("Select hole type...");
            frame.render_widget(body, leaflets[1]);
        }

        Pages::ShowingCornerFeatureOptions => {
            let body = Paragraph::new("Select corner type...");
            frame.render_widget(body, leaflets[1]);
        }

        Pages::ShowingCutoutFeatureOptions => {
            let body = Paragraph::new("Select cutout type...");
            frame.render_widget(body, leaflets[1]);
        }

        Pages::ShowingCircularFeatureOptions => {
            let body = Paragraph::new("Select circular feature type...");
            frame.render_widget(body, leaflets[1]);
        }

        Pages::AddingFeature => {
            let body;
            if let Some(path) = &app.current_feature_addition_path {
                body = Paragraph::new(vec![
                    Line::from(format!("Adding {}", path.feature.name())),
                    Line::from(format!("{}: {}", path.current_step_value(), path.current_step_value_input())),
                ]);
            }
            else {
                body = Paragraph::new("Cannot display feature type");
            }

            frame.render_widget(body, leaflets[1]);
        }

        Pages::RemovingFeature => {
            let body = Paragraph::new("Removing feature...");
            frame.render_widget(body, leaflets[1]);
        }

        Pages::FinishingBody => {
            let body = Paragraph::new("Complete body?");
            frame.render_widget(body, leaflets[1]);
        }

        Pages::Quitting => {
            let body = Paragraph::new("Quitting...");
            frame.render_widget(body, leaflets[1]);
        }
    }
}



pub struct Instruction {
    key: String,
    label: String,
    pub keybind: KeyCode
}
impl Instruction {
    pub fn new(key: String, label: String, keybind: KeyCode) -> Instruction { Instruction {key, label, keybind } }

    fn printed(&mut self) -> String {
        let mut print = "".to_string();
        print += &format!("[{}] {}", &self.key, &self.label);
        print
    }

    pub fn in_groups(instructions: Vec<Instruction>, group_limit: usize) -> Vec<Line<'static>> {
        // the lines of instructions to be returned
        let mut lines = Vec::new();

        // the current line of instructions being assembled
        let mut current_group: String = "".to_string();
        let mut amount_in_group: usize = 0;

        // adds the current line to the list of lines and creates a new blank line in its place if it reaches the group limit
        for mut instruction in instructions {
            if amount_in_group >= group_limit {
                lines.push(Line::from(current_group));
                current_group = "".to_string();
                amount_in_group = 0;
            }

            // adds the current instruction to the current line
            amount_in_group += 1;
            if current_group != "" { current_group += " | "; }
            current_group += instruction.printed().as_str();
        }

        // adds the last line to the list of lines if it isn't empty
        if current_group != "" { lines.push(Line::from(current_group)); }

        // returns the list of lines
        lines
    }

    // instructions
    //      navigation
    pub fn confirm_instruction() -> Instruction { Instruction::new("ENTER".to_string(), "confirm".to_string(), KeyCode::Enter) }
    pub fn cancel_instruction() -> Instruction { Instruction::new("X".to_string(), "cancel".to_string(), KeyCode::Char('x')) }
    pub fn quit_instruction() -> Instruction { Instruction::new("Q".to_string(), "quit".to_string(), KeyCode::Char('q')) }
    //      body/feature management
    pub fn rename_instruction() -> Instruction { Instruction::new("N".to_string(), "rename body".to_string(), KeyCode::Char('n')) }
    pub fn finish_instruction() -> Instruction { Instruction::new("F".to_string(), "finish".to_string(), KeyCode::Char('f')) }
    pub fn reset_instruction() -> Instruction { Instruction::new("ESC".to_string(), "reset".to_string(), KeyCode::Esc) }
    pub fn remove_feature_instruction() -> Instruction { Instruction::new("R".to_string(), "remove feature".to_string(), KeyCode::Char('r')) }
    //      holes
    pub fn add_hole_instruction() -> Instruction { Instruction::new("0".to_string(), "add hole".to_string(), KeyCode::Char('0')) }
    pub fn add_circular_hole_instruction() -> Instruction { Instruction::new("1".to_string(), "add circular hole".to_string(), KeyCode::Char('1')) }
    pub fn add_capsular_hole_instruction() -> Instruction { Instruction::new("2".to_string(), "add capsular hole".to_string(), KeyCode::Char('2')) }
    pub fn add_rectangular_hole_instruction() -> Instruction { Instruction::new("3".to_string(), "add rectangular hole".to_string(), KeyCode::Char('3')) }
    //      corners
    pub fn add_corner_instruction() -> Instruction { Instruction::new("1".to_string(), "add corner".to_string(), KeyCode::Char('1')) }
    pub fn add_fillet_instruction() -> Instruction { Instruction::new("1".to_string(), "add fillet".to_string(), KeyCode::Char('1')) }
    pub fn add_chamfer_instruction() -> Instruction { Instruction::new("2".to_string(), "add chamfer".to_string(), KeyCode::Char('2')) }
    pub fn add_slope_instruction() -> Instruction { Instruction::new("4".to_string(), "add slope".to_string(), KeyCode::Char('4')) }
    pub fn add_cliff_instruction() -> Instruction { Instruction::new("5".to_string(), "add cliff".to_string(), KeyCode::Char('5')) }
    //      cutout
    pub fn add_cutout_instruction() -> Instruction { Instruction::new("2".to_string(), "add cutout".to_string(), KeyCode::Char('2')) }
    pub fn add_notch_instruction() -> Instruction { Instruction::new("1".to_string(), "add notch".to_string(), KeyCode::Char('1')) }
    pub fn add_sawtooth_instruction() -> Instruction { Instruction::new("2".to_string(), "add sawtooth".to_string(), KeyCode::Char('2')) }
    pub fn add_claw_instruction() -> Instruction { Instruction::new("3".to_string(), "add claw".to_string(), KeyCode::Char('3')) }
    pub fn add_composite_slope_instruction() -> Instruction { Instruction::new("4".to_string(), "add composite slope".to_string(), KeyCode::Char('4')) }
    //      circular
    pub fn add_circular_feature_instruction() -> Instruction { Instruction::new("3".to_string(), "add circular feature".to_string(), KeyCode::Char('3')) }
    pub fn add_arc_instruction() -> Instruction { Instruction::new("1".to_string(), "add arc".to_string(), KeyCode::Char('1')) }
    pub fn add_ellipse_instruction() -> Instruction { Instruction::new("2".to_string(), "add ellipse".to_string(), KeyCode::Char('2')) }

    //      other
    pub fn add_other_feature_instruction() -> Instruction { Instruction::new("4".to_string(), "add other feature".to_string(), KeyCode::Char('4')) }
}



pub fn get_instructions_for(page: &Pages) -> Vec<Line> {
    return match page {
        Pages::Launching => {
            Instruction::in_groups(vec![
                Instruction::quit_instruction(),
            ], 3)
        }

        Pages::AddingBody => {
            Instruction::in_groups(vec![
                Instruction::confirm_instruction(),
                Instruction::cancel_instruction(),
                Instruction::quit_instruction(),
            ], 3)
        }

        Pages::BodyView => {
            Instruction::in_groups(vec![
                Instruction::add_hole_instruction(),
                Instruction::add_corner_instruction(),
                Instruction::add_cutout_instruction(),
                Instruction::add_circular_feature_instruction(),
                Instruction::add_other_feature_instruction(),
                Instruction::rename_instruction(),
                Instruction::remove_feature_instruction(),
                Instruction::finish_instruction(),
                Instruction::reset_instruction(),
                Instruction::quit_instruction(),
            ], 3)
        }

        Pages::RenamingBody => {
            Instruction::in_groups(vec![
                Instruction::confirm_instruction(),
                Instruction::cancel_instruction(),
            ], 3)
        }

        Pages::ShowingHoleFeatureOptions => {
            Instruction::in_groups(vec![
                Instruction::add_circular_hole_instruction(),
                Instruction::add_capsular_hole_instruction(),
                Instruction::add_rectangular_hole_instruction(),
                Instruction::cancel_instruction(),
            ], 3)
        }

        Pages::ShowingCornerFeatureOptions => {
            Instruction::in_groups(vec![
                Instruction::add_fillet_instruction(),
                Instruction::add_chamfer_instruction(),
                Instruction::add_slope_instruction(),
                Instruction::add_cliff_instruction(),
                Instruction::cancel_instruction(),
            ], 3)
        }

        Pages::ShowingCutoutFeatureOptions => {
            Instruction::in_groups(vec![
                Instruction::add_notch_instruction(),
                Instruction::add_sawtooth_instruction(),
                Instruction::add_claw_instruction(),
                Instruction::add_composite_slope_instruction(),
                Instruction::cancel_instruction(),
            ], 3)
        }

        Pages::ShowingCircularFeatureOptions => {
            Instruction::in_groups(vec![
                Instruction::add_arc_instruction(),
                Instruction::add_ellipse_instruction(),
                Instruction::cancel_instruction(),
            ], 3)
        }

        Pages::AddingFeature => {
            Instruction::in_groups(vec![
                Instruction::confirm_instruction(),
                Instruction::cancel_instruction(),
            ], 3)
        }

        Pages::RemovingFeature => {
            Instruction::in_groups(vec![
                Instruction::cancel_instruction(),
            ], 3)
        }

        Pages::FinishingBody => {
            Instruction::in_groups(vec![
                Instruction::confirm_instruction(),
                Instruction::cancel_instruction(),
            ], 3)
        }

        Pages::Quitting => {
            Instruction::in_groups(vec![
                Instruction::confirm_instruction(),
                Instruction::cancel_instruction(),
            ], 3)
        }
    }
}