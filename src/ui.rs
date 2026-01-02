use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
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
    // The main sections of the screen.
    let leaflets = Layout::new(Direction::Vertical, [
        Constraint::Length(4), // header
        Constraint::Min(1), // body
        Constraint::Max(5), // footer
    ]).split(frame.area());

    // header
    let header_block = Block::new().borders(Borders::ALL);
    let header = Paragraph::new(vec![
        Line::raw("PERI"),
        Line::raw(app.current_page_name()),
    ]).block(header_block);
    frame.render_widget(header, leaflets[0]);

    // footer
    let footer_block = Block::new().borders(Borders::ALL);
    let instructions = get_instructions_for(&app.current_page);
    let footer = Paragraph::new(instructions).wrap(Wrap { trim: true }).block(footer_block);
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
            let body = Paragraph::new(vec![
                Line::raw(&app.body.name),
                Line::raw(format!("Width : {}", app.body.width)),
                Line::raw(format!("Height: {}", app.body.height)),
                Line::raw(format!("Perimeter : {}", app.body.perimeter())),
            ]);

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
                body = Paragraph::new(format!("Adding {}", path.feature.name()));
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



struct Instruction {
    key: String,
    label: String,
}
impl Instruction {
    pub fn new(key: String, label: String) -> Instruction { Instruction {key, label } }

    fn printed(&mut self) -> String {
        let mut print = "".to_string();
        print += &format!("[ {} ] : {}", &self.key, &self.label);
        print
    }

    // instructions
    //      navigation
    fn confirm_instruction() -> Instruction { Instruction::new("ENTER".to_string(), "confirm".to_string()) }
    fn cancel_instruction() -> Instruction { Instruction::new("X".to_string(), "cancel".to_string()) }
    fn quit_instruction() -> Instruction { Instruction::new("Q".to_string(), "quit".to_string()) }
    //      body/feature management
    fn rename_instruction() -> Instruction { Instruction::new("N".to_string(), "rename body".to_string()) }
    fn finish_instruction() -> Instruction { Instruction::new("F".to_string(), "finish".to_string()) }
    fn remove_feature_instruction() -> Instruction { Instruction::new("R".to_string(), "remove feature".to_string()) }
    //      holes
    fn add_hole_instruction() -> Instruction { Instruction::new("0".to_string(), "add hole".to_string()) }
    fn add_circular_hole_instruction() -> Instruction { Instruction::new("1".to_string(), "add circular hole".to_string()) }
    fn add_capsular_hole_instruction() -> Instruction { Instruction::new("2".to_string(), "add capsular hole".to_string()) }
    fn add_rectangular_hole_instruction() -> Instruction { Instruction::new("3".to_string(), "add rectangular hole".to_string()) }
    //      corners
    fn add_corner_instruction() -> Instruction { Instruction::new("1".to_string(), "add corner".to_string()) }
    fn add_fillet_instruction() -> Instruction { Instruction::new("1".to_string(), "add fillet".to_string()) }
    fn add_chamfer_instruction() -> Instruction { Instruction::new("2".to_string(), "add chamfer".to_string()) }
    fn add_slope_instruction() -> Instruction { Instruction::new("4".to_string(), "add slope".to_string()) }
    fn add_cliff_instruction() -> Instruction { Instruction::new("5".to_string(), "add cliff".to_string()) }
    //      cutout
    fn add_cutout_instruction() -> Instruction { Instruction::new("2".to_string(), "add cutout".to_string()) }
    fn add_notch_instruction() -> Instruction { Instruction::new("1".to_string(), "add notch".to_string()) }
    fn add_sawtooth_instruction() -> Instruction { Instruction::new("2".to_string(), "add sawtooth".to_string()) }
    fn add_claw_instruction() -> Instruction { Instruction::new("3".to_string(), "add claw".to_string()) }
    fn add_composite_slope_instruction() -> Instruction { Instruction::new("4".to_string(), "add composite slope".to_string()) }
    //      circular
    fn add_circular_feature_instruction() -> Instruction { Instruction::new("3".to_string(), "add circular feature".to_string()) }
    fn add_arc_instruction() -> Instruction { Instruction::new("1".to_string(), "add arc".to_string()) }
    fn add_ellipse_instruction() -> Instruction { Instruction::new("2".to_string(), "add ellipse".to_string()) }

    //      other
    fn add_other_feature_instruction() -> Instruction { Instruction::new("4".to_string(), "add other feature".to_string()) }
}



pub fn get_instructions_for(page: &Pages) -> Vec<Line> {
    let mut instructions = vec![];

    match page {
        Pages::Launching => {
            instructions.push(Line::raw(Instruction::quit_instruction().printed()));
        }

        Pages::AddingBody => {
            instructions.push(Line::raw(Instruction::confirm_instruction().printed()));
            instructions.push(Line::raw(Instruction::cancel_instruction().printed()));
            instructions.push(Line::raw(Instruction::quit_instruction().printed()));
        }

        Pages::BodyView => {
            instructions.push(Line::raw(Instruction::add_hole_instruction().printed()));
            instructions.push(Line::raw(Instruction::add_corner_instruction().printed()));
            instructions.push(Line::raw(Instruction::add_cutout_instruction().printed()));
            instructions.push(Line::raw(Instruction::add_other_feature_instruction().printed()));
            instructions.push(Line::raw(Instruction::remove_feature_instruction().printed()));
            instructions.push(Line::raw(Instruction::finish_instruction().printed()));
            instructions.push(Line::raw(Instruction::quit_instruction().printed()));
        }

        Pages::RenamingBody => {
            instructions.push(Line::raw(Instruction::confirm_instruction().printed()));
            instructions.push(Line::raw(Instruction::cancel_instruction().printed()));
        }


        Pages::ShowingHoleFeatureOptions => {
            instructions.push(Line::raw(Instruction::add_circular_hole_instruction().printed()));
            instructions.push(Line::raw(Instruction::add_capsular_hole_instruction().printed()));
            instructions.push(Line::raw(Instruction::add_rectangular_hole_instruction().printed()));
            instructions.push(Line::raw(Instruction::cancel_instruction().printed()));
        }

        Pages::ShowingCornerFeatureOptions => {
            instructions.push(Line::raw(Instruction::add_fillet_instruction().printed()));
            instructions.push(Line::raw(Instruction::add_chamfer_instruction().printed()));
            instructions.push(Line::raw(Instruction::add_slope_instruction().printed()));
            instructions.push(Line::raw(Instruction::add_cliff_instruction().printed()));
            instructions.push(Line::raw(Instruction::cancel_instruction().printed()));
        }

        Pages::ShowingCutoutFeatureOptions => {
            instructions.push(Line::raw(Instruction::add_notch_instruction().printed()));
            instructions.push(Line::raw(Instruction::add_sawtooth_instruction().printed()));
            instructions.push(Line::raw(Instruction::add_claw_instruction().printed()));
            instructions.push(Line::raw(Instruction::add_composite_slope_instruction().printed()));
            instructions.push(Line::raw(Instruction::add_arc_instruction().printed()));
            instructions.push(Line::raw(Instruction::add_ellipse_instruction().printed()));
            instructions.push(Line::raw(Instruction::cancel_instruction().printed()));
        }

        Pages::ShowingCircularFeatureOptions => {
            instructions.push(Line::raw(Instruction::add_arc_instruction().printed()));
            instructions.push(Line::raw(Instruction::add_ellipse_instruction().printed()));
        }

        Pages::AddingFeature => {
            instructions.push(Line::raw(Instruction::confirm_instruction().printed()));
            instructions.push(Line::raw(Instruction::cancel_instruction().printed()));
        }

        Pages::RemovingFeature => {
            instructions.push(Line::raw(Instruction::confirm_instruction().printed()));
            instructions.push(Line::raw(Instruction::cancel_instruction().printed()));
        }

        Pages::FinishingBody => {
            instructions.push(Line::raw(Instruction::confirm_instruction().printed()));
            instructions.push(Line::raw(Instruction::cancel_instruction().printed()));
        }

        Pages::Quitting => {
            instructions.push(Line::raw(Instruction::confirm_instruction().printed()));
            instructions.push(Line::raw(Instruction::cancel_instruction().printed()));
        }
    }

    instructions
}