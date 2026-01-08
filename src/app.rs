use std::io::Result;
use std::string::String;
use ratatui::backend::Backend;
use ratatui::crossterm::event;
use ratatui::crossterm::event::Event;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::prelude::Line;
use ratatui::Terminal;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::shapes::*;
use crate::ui::{get_instructions_for, ui, Instruction};

#[derive(Copy, Clone)]
pub enum Pages {
    Launching, // c
    AddingBody, // c
    BodyView, // c
    RenamingBody, // c
    ShowingHoleFeatureOptions, // c
    ShowingCornerFeatureOptions, // c
    ShowingCutoutFeatureOptions, // c
    ShowingCircularFeatureOptions, // i
    AddingFeature, // i
    RemovingFeature, // i
    ResettingBody, //i
    FinishingBody, // i
    Quitting, // i
}



pub struct App {
    pub body: Body<>,
    pub current_page: Pages,
    pub is_name_set: bool,
    pub is_width_set: bool,
    pub is_height_set: bool,
    pub new_body_name: String,
    pub new_body_width: String,
    pub new_body_height: String,
    pub current_feature_addition_path: Option<FeatureAdditionPath>,
    pub feature_page_index: usize,
    pub feature_pages: Vec<Vec<String>>,
}

impl App {
    pub fn new() -> App {
        App {
            body: Body::new(),
            current_page: Pages::Launching,
            is_name_set: false,
            is_width_set: false,
            is_height_set: false,
            new_body_name: "".to_string(),
            new_body_width: "".to_string(),
            new_body_height: "".to_string(),
            current_feature_addition_path: None,
            feature_page_index: 0,
            feature_pages: Vec::new(),
        }
    }

    pub fn go_to_next_feature_page(&mut self) {
        if self.feature_pages.is_empty() { return; }

        if self.feature_page_index >= self.feature_pages.len() - 1 { return; }
        self.feature_page_index += 1;
    }

    pub fn go_to_previous_feature_page(&mut self) {
        if self.feature_pages.is_empty() { return; }

        if self.feature_page_index == 0 { return; }
        self.feature_page_index -= 1;
    }

    pub fn current_feature_page(&self) -> Vec<String> {
        if self.feature_pages.is_empty() {
            return Vec::new();
        }

        self.feature_pages[self.feature_page_index]
            .iter()
            .map(|line| line.clone())
            .collect()
    }

    pub fn current_page_name(&self) -> String {
        match self.current_page {
            Pages::Launching => { "Launching".to_string() }
            Pages::AddingBody => { "Adding Body".to_string() }
            Pages::BodyView => { "Body View".to_string() }
            Pages::RenamingBody => { "Renaming Body".to_string() }
            Pages::ShowingHoleFeatureOptions => { "Hole Feature Options".to_string() }
            Pages::ShowingCornerFeatureOptions => { "Corner Feature Options".to_string() }
            Pages::ShowingCutoutFeatureOptions => { "Cutout Feature Options".to_string() }
            Pages::ShowingCircularFeatureOptions => { "Circular Feature Options".to_string() }
            Pages::AddingFeature => { "Adding Feature".to_string() }
            Pages::RemovingFeature => { "Removing Feature".to_string() }
            Pages::ResettingBody => { "Resetting Body".to_string() }
            Pages::FinishingBody => { "Finishing Body".to_string() }
            Pages::Quitting => { "Quitting".to_string() }
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        // running
        loop {
            // pre-render
            let footer_height = get_instructions_for(&self.current_page).len() as u16 + 2;
            let header_height = 4;
            let page_height = terminal.size()?.height - footer_height - header_height;



            // assembling feature pages
            // the pages of features
            let mut new_feature_pages: Vec<Vec<String>> = Vec::new();
            // the current page of features being assembled
            let mut current_page: Vec<String> = Vec::new();
            // how many lines have been used in the current page
            let mut lines_used_in_current_page: u16 = 0;
            // iterating through the features
            for i in 0..self.body.features.len() {
                // checks if a new page is needed
                if lines_used_in_current_page + self.body.features[i].print_height() + 1 > page_height {
                    new_feature_pages.push(current_page);
                    current_page = Vec::new();
                    lines_used_in_current_page = 0;
                }
                // adds the feature to the current page
                let lines_to_add = self.body.features[i].summarize();
                lines_used_in_current_page += lines_to_add.len() as u16 + 1;
                current_page.push("".to_string());
                current_page.extend(lines_to_add);
            }
            // adds the last page if it is not empty
            if !current_page.is_empty() { new_feature_pages.push(current_page); }

            // updates the feature page count in the app
            let cloned_new_feature_pages: Vec<Vec<String>> = new_feature_pages
                .iter()
                .map(|line| line.clone())
                .collect();
            self.feature_pages = new_feature_pages;

            // makes sure that the feature page index is valid
            if !self.feature_pages.is_empty() {
                if self.feature_page_index >= self.feature_pages.len() { self.feature_page_index = self.feature_pages.len() - 1; }
            }



            // rendering
            terminal.draw(|f| ui(f, self))?;

            // getting input
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release { continue; }

                match self.current_page {
                    Pages::Launching => {
                        self.current_page = Pages::AddingBody;
                        continue;
                    }

                    Pages::AddingBody => {
                        // quits
                        if key.code == Instruction::quit_instruction().keybind {
                            self.current_page = Pages::Quitting;
                            continue;
                        }

                        // cancels/resets
                        if key.code == Instruction::cancel_instruction().keybind {
                            self.new_body_name = "".to_string();
                            self.new_body_width = "".to_string();
                            self.new_body_height = "".to_string();
                            self.is_name_set = false;
                            self.is_width_set = false;
                            self.is_height_set = false;
                            continue;
                        }

                        // edits the new body name
                        if !self.is_name_set {
                            self.new_body_name = term_tools::keypad(&self.new_body_name, key);
                            if key.code == Instruction::confirm_instruction().keybind {
                                if self.new_body_name.is_empty() { continue; }
                                self.is_name_set = true;
                                continue;
                            }
                        }

                        // edits the new width before the height
                        if !self.is_width_set && self.is_name_set {
                            self.new_body_width = term_tools::numpad(&self.new_body_width, key);
                            if key.code == Instruction::confirm_instruction().keybind {
                                if self.new_body_width.parse::<f64>().unwrap_or(0.0) <= 0.0 { continue; }
                                self.is_width_set = true;
                                continue;
                            }
                        }

                        // edits the new height if the new width is set
                        if !self.is_height_set && self.is_name_set && self.is_width_set {
                            self.new_body_height = term_tools::numpad(&self.new_body_height, key);
                            if key.code == Instruction::confirm_instruction().keybind {
                                if self.new_body_height.parse::<f64>().unwrap_or(0.0) <= 0.0 { continue; }
                                self.is_height_set = true;
                                //continue;
                            }
                        }

                        // creates a new body if both width and height are set
                        if self.is_width_set && self.is_height_set {
                            self.body = Body::new();
                            self.body.rename(self.new_body_name.clone());
                            self.body.set_width(self.new_body_width.parse::<f64>().unwrap_or(0.0));
                            self.body.set_height(self.new_body_height.parse::<f64>().unwrap_or(0.0));
                            self.current_page = Pages::BodyView;
                            continue;
                        }
                    }

                    Pages::BodyView => {
                        // quits
                        if key.code == Instruction::quit_instruction().keybind {
                            self.current_page = Pages::Quitting;
                            continue;
                        }

                        // goes to the previous feature page
                        else if key.code == Instruction::previous_page().keybind {
                            self.go_to_previous_feature_page();
                            continue;
                        }

                        // goes to the next feature page
                        else if key.code == Instruction::next_page().keybind {
                            self.go_to_next_feature_page();
                            continue;
                        }

                        // shows hole feature options
                        else if key.code == Instruction::add_hole_instruction().keybind {
                            self.current_page = Pages::ShowingHoleFeatureOptions;
                            continue;
                        }

                        // shows hole feature options
                        else if key.code == Instruction::add_corner_instruction().keybind {
                            self.current_page = Pages::ShowingCornerFeatureOptions;
                            continue;
                        }

                        // shows hole feature options
                        else if key.code == Instruction::add_cutout_instruction().keybind {
                            self.current_page = Pages::ShowingCutoutFeatureOptions;
                            continue;
                        }

                        // shows hole feature options
                        else if key.code == Instruction::add_circular_feature_instruction().keybind {
                            self.current_page = Pages::ShowingCircularFeatureOptions;
                            continue;
                        }

                        // adds an undefined feature
                        else if key.code == Instruction::add_other_feature_instruction().keybind {
                            self.start_adding_feature(Features::OtherFeatureFeature);
                            continue;
                        }

                        // renames a body
                        else if key.code == Instruction::rename_instruction().keybind {
                            self.new_body_name = self.body.name.clone();
                            self.current_page = Pages::RenamingBody;
                            continue;
                        }

                        // removes a feature
                        else if key.code == Instruction::remove_feature_instruction().keybind {
                            self.current_page = Pages::RemovingFeature;
                            continue;
                        }

                        // resets the body
                        else if key.code == Instruction::reset_instruction().keybind {
                            self.current_page = Pages::ResettingBody;
                            continue;
                        }

                        // finishes the body
                        else if key.code == Instruction::finish_instruction().keybind {
                            self.current_page = Pages::FinishingBody;
                            continue;
                        }
                    }

                    Pages::RenamingBody => {
                        // edits the new body name
                        self.new_body_name = term_tools::keypad(&self.new_body_name, key);

                        // renames the body
                        if key.code == Instruction::confirm_instruction().keybind {
                            if self.new_body_name.is_empty() { continue; }
                            self.is_name_set = true;
                            self.body.rename(self.new_body_name.clone());
                            self.current_page = Pages::BodyView;
                        }
                    }

                    Pages::ShowingHoleFeatureOptions => {
                        // cancels
                        if key.code == Instruction::cancel_instruction().keybind {
                            self.current_page = Pages::BodyView;
                            continue;
                        }

                        // adds a circular hole
                        else if key.code == Instruction::add_circular_hole_instruction().keybind {
                            self.start_adding_feature(Features::CircularHoleFeature);
                            continue;
                        }

                        // adds a capsule hole
                        else if key.code == Instruction::add_capsular_hole_instruction().keybind {
                            self.start_adding_feature(Features::CapsularHoleFeature);
                            continue;
                        }

                        // adds a rectangular hole
                        else if key.code == Instruction::add_rectangular_hole_instruction().keybind {
                            self.start_adding_feature(Features::RectangularHoleFeature);
                            continue;
                        }
                    }

                    Pages::ShowingCornerFeatureOptions => {
                        // cancels
                        if key.code == Instruction::cancel_instruction().keybind {
                            self.current_page = Pages::BodyView;
                            continue;
                        }

                        // adds a fillet
                        else if key.code == Instruction::add_fillet_instruction().keybind {
                            self.start_adding_feature(Features::FilletFeature);
                            continue;
                        }

                        // adds a chamfer
                        else if key.code == Instruction::add_chamfer_instruction().keybind {
                            self.start_adding_feature(Features::ChamferFeature);
                            continue;
                        }

                        // adds a slope
                        else if key.code == Instruction::add_slope_instruction().keybind {
                            self.start_adding_feature(Features::SlopeFeature);
                            continue;
                        }

                        // adds a cliff
                        else if key.code == Instruction::add_cliff_instruction().keybind {
                            self.start_adding_feature(Features::CliffFeature);
                            continue;
                        }
                    }

                    Pages::ShowingCutoutFeatureOptions => {
                        // cancels
                        if key.code == Instruction::cancel_instruction().keybind {
                            self.current_page = Pages::BodyView;
                            continue;
                        }

                        // adds a notch
                        else if key.code == Instruction::add_notch_instruction().keybind {
                            self.start_adding_feature(Features::NotchFeature);
                            continue;
                        }

                        // adds a sawtooth
                        else if key.code == Instruction::add_sawtooth_instruction().keybind {
                            self.start_adding_feature(Features::SawtoothFeature);
                            continue;
                        }

                        // adds a claw
                        else if key.code == Instruction::add_claw_instruction().keybind {
                            self.start_adding_feature(Features::ClawFeature);
                            continue;
                        }

                        // adds a composite slope
                        else if key.code == Instruction::add_composite_slope_instruction().keybind {
                            self.start_adding_feature(Features::CompositeSlopeFeature);
                            continue;
                        }
                    }

                    Pages::ShowingCircularFeatureOptions => {
                        // cancels
                        if key.code == Instruction::cancel_instruction().keybind {
                            self.current_page = Pages::BodyView;
                            continue;
                        }

                        // adds an arc
                        else if key.code == Instruction::add_arc_instruction().keybind {
                            self.start_adding_feature(Features::ArcFeature);
                            continue;
                        }

                        // adds an ellipse
                        else if key.code == Instruction::add_ellipse_instruction().keybind {
                            self.start_adding_feature(Features::EllipseFeature);
                            continue;
                        }
                    }

                    Pages::AddingFeature => {
                        // cancels
                        if key.code == Instruction::cancel_instruction().keybind {
                            self.current_page = Pages::BodyView;
                            continue;
                        }

                        // updates the current value input and finishes the current step if the confirmation key is pressed
                        if let Some(path) = &mut self.current_feature_addition_path {
                            let new_value_input = term_tools::numpad(&path.current_step_value_input(), key);
                            path.update_current_step_value_input(new_value_input);

                            if key.code == Instruction::confirm_instruction().keybind {
                                let result = path.finish_current_step();
                                if let Some(feature) = result {
                                    self.body.add(feature);
                                    self.current_feature_addition_path = None;
                                    self.current_page = Pages::BodyView;
                                    continue;
                                }
                                continue;
                            }
                        }

                        // cancels if there is no current feature addition path
                        else {
                            self.current_page = Pages::BodyView;
                            continue;
                        }
                    }

                    Pages::RemovingFeature => {
                        // cancels
                        if key.code == Instruction::cancel_instruction().keybind {
                            self.current_page = Pages::BodyView;
                            continue;
                        }
                    }

                    Pages::ResettingBody => {
                        // cancels
                        if key.code == Instruction::cancel_instruction().keybind {
                            self.current_page = Pages::BodyView;
                            continue;
                        }

                        // resets the body
                        else if key.code == Instruction::confirm_instruction().keybind {
                            self.new_body_name = "".to_string();
                            self.new_body_width = "".to_string();
                            self.new_body_height = "".to_string();
                            self.is_name_set = false;
                            self.is_width_set = false;
                            self.is_height_set = false;
                            self.body = Body::new();
                            self.current_page = Pages::AddingBody;
                            continue;
                        }
                    }

                    Pages::FinishingBody => {
                        // cancels
                        if key.code == Instruction::cancel_instruction().keybind {
                            self.current_page = Pages::BodyView;
                            continue;
                        }

                        // finishes the body
                        else if key.code == Instruction::confirm_instruction().keybind {
                            self.new_body_name = "".to_string();
                            self.new_body_width = "".to_string();
                            self.new_body_height = "".to_string();
                            self.is_name_set = false;
                            self.is_width_set = false;
                            self.is_height_set = false;
                            self.body = Body::new();
                            self.current_page = Pages::AddingBody;
                            continue;
                        }
                    }

                    Pages::Quitting => {
                        // cancels
                        if key.code == Instruction::cancel_instruction().keybind {
                            self.current_page = Pages::BodyView;
                            continue;
                        }

                        // quits
                        else if key.code == Instruction::confirm_instruction().keybind {
                            break;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn start_adding_feature(&mut self, feature: Features) {
        self.current_feature_addition_path = Some(feature.path());
        self.current_page = Pages::AddingFeature;
    }
}



pub mod term_tools {
    use ratatui::crossterm::event;
    use ratatui::crossterm::event::{KeyCode, KeyEvent};

    pub fn numpad(field: &str, input: KeyEvent) -> String {
        if input.kind == event::KeyEventKind::Release { return field.to_string(); }

        let mut field = field.to_string();
        match input.code {
            KeyCode::Backspace => {
                if field.is_empty() { return field; }
                field.remove(field.len() - 1);
            }
            KeyCode::Char(char) => {
                match char {
                    '0'..='9' => field.push(char),
                    '.' => { if !field.contains('.') { field.push(char); } }
                    _ => {}
                }
            }
            _ => {}
        }

        field
    }

    pub fn keypad(field: &str, input: KeyEvent) -> String {
        if input.kind == event::KeyEventKind::Release {}

        let mut field = field.to_string();
        match input.code {
            KeyCode::Backspace => {
                if field.is_empty() { return field; }
                field.remove(field.len() - 1);
            }
            KeyCode::Char(char) => { field.push(char); }
            _ => {}
        }
        field
    }
}