use std::collections::HashMap;
use std::error::Error;
use std::io::Result;
use std::string::String;
use ratatui::backend::Backend;
use ratatui::crossterm::event;
use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::Terminal;
use crate::shapes::*;
use crate::ui::ui;

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
    FinishingBody, // i
    Quitting, // i
}



pub struct App {
    pub body: Body,
    pub current_page: Pages,
    pub is_name_set: bool,
    pub is_width_set: bool,
    pub is_height_set: bool,
    pub new_body_name: String,
    pub new_body_width: String,
    pub new_body_height: String,
    
    pub current_feature_addition_path: Option<FeatureAdditionPath>,
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
        }
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
            Pages::FinishingBody => { "Finishing Body".to_string() }
            Pages::Quitting => { "Quitting".to_string() }
        }
    }

    pub fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<()> {
        println!("{}", &self.current_page_name());

        loop {
            terminal.draw(|f| ui(f, self))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Release { continue; }

                match self.current_page {
                    Pages::Launching => {
                        self.current_page = Pages::AddingBody;
                        continue;
                    }

                    Pages::AddingBody => {
                        // quits
                        self.try_quit(key);

                        // cancels/resets
                        if key.code == keybinds::CANCEL {
                            self.new_body_name = "".to_string();
                            self.new_body_width = "".to_string();
                            self.new_body_height = "".to_string();
                            self.is_width_set = false;
                            self.is_height_set = false;
                            continue;
                        }

                        // edits the new body name
                        if !self.is_name_set {
                            self.new_body_name = term_tools::keypad(&self.new_body_name, key);
                            if key.code == keybinds::CONFIRM {
                                if self.new_body_name.is_empty() { continue; }
                                self.is_name_set = true;
                                continue;
                            }
                        }

                        // edits the new width before the height
                        if !self.is_width_set && self.is_name_set {
                            self.new_body_width = term_tools::numpad(&self.new_body_width, key);
                            if key.code == keybinds::CONFIRM {
                                if self.new_body_width.parse::<f64>().unwrap_or(0.0) <= 0.0 { continue; }
                                self.is_width_set = true;
                                continue;
                            }
                        }

                        // edits the new height if the new width is set
                        if !self.is_height_set && self.is_name_set && self.is_width_set {
                            self.new_body_height = term_tools::numpad(&self.new_body_height, key);
                            if key.code == keybinds::CONFIRM {
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
                        self.try_quit(key);

                        match key.code {
                            keybinds::SHOW_HOLE_FEATURES => {
                                self.current_page = Pages::ShowingHoleFeatureOptions;
                                continue;
                            }
                            keybinds::SHOW_CORNER_FEATURES => {
                                self.current_page = Pages::ShowingCornerFeatureOptions;
                                continue;
                            }
                            keybinds::SHOW_CUTOUT_FEATURES => {
                                self.current_page = Pages::ShowingCutoutFeatureOptions;
                                continue;
                            }
                            keybinds::SHOW_CIRCULAR_FEATURES => {
                                self.current_page = Pages::ShowingCircularFeatureOptions;
                                continue;
                            }
                            keybinds::CREATE_OTHER_FEATURE => {
                                self.start_adding_feature(Features::OtherFeatureFeature);
                                continue;
                            }
                            keybinds::START_RENAMING_BODY => {
                                self.is_name_set = false;
                                self.current_page = Pages::RenamingBody;
                                continue;
                            }
                            keybinds::START_REMOVING_FEATURE => {
                                self.current_page = Pages::RemovingFeature;
                                continue;
                            }
                            keybinds::START_FINISHING_BODY => {
                                self.current_page = Pages::FinishingBody;
                                continue;
                            }
                            _ => {}
                        }
                    }

                    Pages::RenamingBody => {
                        // quits
                        self.try_quit(key);

                        // cancels/resets
                        if key.code == keybinds::CANCEL {
                            self.is_name_set = true;
                            self.new_body_name = "".to_string();
                            continue;
                        }

                        // edits the new body name
                        self.new_body_name = term_tools::keypad(&self.new_body_name, key);

                        // renames the body
                        if key.code == keybinds::CONFIRM {
                            if self.new_body_name.is_empty() { continue; }
                            self.is_name_set = true;
                            self.body.rename(self.new_body_name.clone());
                            self.current_page = Pages::BodyView;
                        }
                    }

                    Pages::ShowingHoleFeatureOptions => {
                        // cancels
                        if key.code == keybinds::CANCEL {
                            self.current_page = Pages::BodyView;
                            continue;
                        }

                        match key.code {
                            keybinds::CREATE_CIRCULAR_HOLE_FEATURE => {
                                self.start_adding_feature(Features::CircularHoleFeature);
                                continue;
                            }
                            keybinds::CREATE_CAPSULAR_HOLE_FEATURE => {
                                self.start_adding_feature(Features::CapsularHoleFeature);
                                continue;
                            }
                            keybinds::CREATE_RECTANGULAR_HOLE_FEATURE => {
                                self.start_adding_feature(Features::RectangularHoleFeature);
                                continue;
                            }
                            _ => {}
                        }
                    }

                    Pages::ShowingCornerFeatureOptions => {
                        // cancels
                        if key.code == keybinds::CANCEL {
                            self.current_page = Pages::BodyView;
                            continue;
                        }

                        match key.code {
                            keybinds::CREATE_FILLET_FEATURE => {
                                self.start_adding_feature(Features::FilletFeature);
                                continue;
                            }
                            keybinds::CREATE_CHAMFER_FEATURE => {
                                self.start_adding_feature(Features::ChamferFeature);
                                continue;
                            }
                            keybinds::CREATE_SLOPE_FEATURE => {
                                self.start_adding_feature(Features::SlopeFeature);
                                continue;
                            }
                            keybinds::CREATE_CLIFF_FEATURE => {
                                self.start_adding_feature(Features::CliffFeature);
                                continue;
                            }
                            _ => {}
                        }
                    }

                    Pages::ShowingCutoutFeatureOptions => {
                        // cancels
                        if key.code == keybinds::CANCEL {
                            self.current_page = Pages::BodyView;
                            continue;
                        }

                        match key.code {
                            keybinds::CREATE_NOTCH_FEATURE => {
                                self.start_adding_feature(Features::NotchFeature);
                                continue;
                            }
                            keybinds::CREATE_SAWTOOTH_FEATURE => {
                                self.start_adding_feature(Features::SawtoothFeature);
                                continue;
                            }
                            keybinds::CREATE_CLAW_FEATURE => {
                                self.start_adding_feature(Features::ClawFeature);
                                continue;
                            }
                            keybinds::CREATE_COMPOSITE_SLOPE_FEATURE => {
                                self.start_adding_feature(Features::CompositeSlopeFeature);
                                continue;
                            }
                            _ => {}
                        }
                    }

                    Pages::ShowingCircularFeatureOptions => {
                        // cancels
                        if key.code == keybinds::CANCEL {
                            self.current_page = Pages::BodyView;
                            continue;
                        }

                        match key.code {
                            keybinds::CREATE_ARC_FEATURE => {
                                self.start_adding_feature(Features::ArcFeature);
                                continue;
                            }
                            keybinds::CREATE_ELLIPSE_FEATURE => {
                                self.start_adding_feature(Features::EllipseFeature);
                                continue;
                            }
                            _ => {}
                        }
                    }

                    Pages::AddingFeature => {
                        // cancels
                        if key.code == keybinds::CANCEL {
                            self.current_page = Pages::BodyView;
                            continue;
                        }

                        // updates the current value input and finishes the current step if the confirmation key is pressed
                        if let Some(path) = &mut self.current_feature_addition_path {
                            let new_value_input = term_tools::keypad(&path.current_step_value(), key);
                            path.update_current_step_value_input(new_value_input);

                            if key.code == keybinds::CONFIRM {
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
                        if key.code == keybinds::CANCEL {
                            self.current_page = Pages::BodyView;
                            continue;
                        }
                    }

                    Pages::FinishingBody => {
                        self.new_body_name = "".to_string();
                        self.new_body_width = "".to_string();
                        self.new_body_height = "".to_string();
                        self.is_width_set = false;
                        self.is_height_set = false;
                        self.current_page = Pages::AddingBody;
                        continue;
                    }

                    Pages::Quitting => {
                        break;
                    }
                }
            }
        }
        Ok(())
    }
    
    // general actions
    pub fn try_quit(&mut self, input: KeyEvent) {
        if input.code == keybinds::QUIT {
            self.current_page = Pages::Quitting;
        }
    }
    
    pub fn start_adding_feature(&mut self, feature: Features) {
        self.current_feature_addition_path = Some(feature.path());
        self.current_page = Pages::AddingFeature;
    }

    // actions for Adding Body

    pub fn cancel_adding_body(&mut self) {
        self.new_body_width = "".to_string();
        self.new_body_height = "".to_string();
    }

    pub fn finish_adding_body(&mut self) {
        self.body.set_width(self.new_body_width.parse::<f64>().unwrap_or(0.0));
        self.body.set_height(self.new_body_height.parse::<f64>().unwrap_or(0.0));
        self.current_page = Pages::BodyView;
    }

    // actions for Renaming Body
    pub fn rename_body(&mut self) {
        self.body.rename(self.new_body_name.clone());
        self.new_body_name = "".to_string();
        self.current_page = Pages::BodyView;
    }
    
    // actions for Body View
    pub fn show_hole_feature_options(&mut self) { self.current_page = Pages::ShowingHoleFeatureOptions; }
    
    pub fn show_corner_feature_options(&mut self) { self.current_page = Pages::ShowingCornerFeatureOptions; }
    
    pub fn show_cutout_feature_options(&mut self) { self.current_page = Pages::ShowingCutoutFeatureOptions; }

    pub fn start_renaming_body(&mut self) { self.current_page = Pages::RenamingBody; }

    pub fn start_removing_feature(&mut self) { self.current_page = Pages::RemovingFeature; }
    
    pub fn start_finishing_body(&mut self) { self.current_page = Pages::FinishingBody; }
    
    pub fn reset_body(&mut self) {
        self.body = Body::new();
        self.current_page = Pages::BodyView;
    }
    
    // actions for Adding Feature
    pub fn cancel_adding_feature(&mut self) { self.current_page = Pages::BodyView; }
    
    pub fn update_current_field_value(&mut self, value: &str) {
        if let Some(path) = &mut self.current_feature_addition_path {
            path.update_current_step_value_input(value.to_string());
        }
    }
    
    pub fn finish_step(&mut self) {
        if let Some(path) = &mut self.current_feature_addition_path {
            path.finish_current_step();
        }
    }
    
    // actions for Finishing Body
    pub fn finish_body(&mut self) {
        self.new_body_name = "".to_string();
        self.new_body_width = "".to_string();
        self.new_body_height = "".to_string();
        self.is_width_set = false;
        self.is_height_set = false;
        self.current_page = Pages::AddingBody;
    }
}

pub mod keybinds {
    use ratatui::crossterm::event::KeyCode;

    // general
    pub const CANCEL: KeyCode = KeyCode::Char('x');
    pub const QUIT: KeyCode = KeyCode::Char('q');
    pub const CONFIRM: KeyCode = KeyCode::Enter;
    pub const BACKSPACE: KeyCode = KeyCode::Backspace;
    // body view
    pub const START_RENAMING_BODY: KeyCode = KeyCode::Char('n');
    pub const START_REMOVING_FEATURE: KeyCode = KeyCode::Char('r');
    pub const START_FINISHING_BODY: KeyCode = KeyCode::Char('f');
    // feature selection
    //      holes
    pub const SHOW_HOLE_FEATURES: KeyCode = KeyCode::Char('0');
    pub const CREATE_CIRCULAR_HOLE_FEATURE: KeyCode = KeyCode::Char('1');
    pub const CREATE_CAPSULAR_HOLE_FEATURE: KeyCode = KeyCode::Char('2');
    pub const CREATE_RECTANGULAR_HOLE_FEATURE: KeyCode = KeyCode::Char('3');
    //      corners
    pub const SHOW_CORNER_FEATURES: KeyCode = KeyCode::Char('1');
    pub const CREATE_FILLET_FEATURE: KeyCode = KeyCode::Char('1');
    pub const CREATE_CHAMFER_FEATURE: KeyCode = KeyCode::Char('2');
    pub const CREATE_SLOPE_FEATURE: KeyCode = KeyCode::Char('4');
    pub const CREATE_CLIFF_FEATURE: KeyCode = KeyCode::Char('5');
    //      cutouts
    pub const SHOW_CUTOUT_FEATURES: KeyCode = KeyCode::Char('2');
    pub const CREATE_NOTCH_FEATURE: KeyCode = KeyCode::Char('1');
    pub const CREATE_SAWTOOTH_FEATURE: KeyCode = KeyCode::Char('2');
    pub const CREATE_CLAW_FEATURE: KeyCode = KeyCode::Char('3');
    pub const CREATE_COMPOSITE_SLOPE_FEATURE: KeyCode = KeyCode::Char('4');
    //      circular features
    pub const SHOW_CIRCULAR_FEATURES: KeyCode = KeyCode::Char('3');
    pub const CREATE_ARC_FEATURE: KeyCode = KeyCode::Char('1');
    pub const CREATE_ELLIPSE_FEATURE: KeyCode = KeyCode::Char('2');
    //      other features
    pub const CREATE_OTHER_FEATURE: KeyCode = KeyCode::Char('4');
}

pub mod term_tools {
    use ratatui::crossterm::event;
    use ratatui::crossterm::event::{Event, KeyCode, KeyEvent};
    use crate::app::keybinds;

    pub fn numpad(field: &str, input: KeyEvent) -> String {
        if input.kind == event::KeyEventKind::Release { return field.to_string(); }

        let mut field = field.to_string();
        match input.code {
            keybinds::BACKSPACE => {
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
            keybinds::BACKSPACE => {
                if field.is_empty() { return field; }
                field.remove(field.len() - 1);
            }
            KeyCode::Char(char) => { field.push(char); }
            _ => {}
        }
        field
    }
}