use std::any::Any;
use std::cmp::PartialEq;
use ratatui::text::Line;

/// The list of possible features.
#[derive(Clone)]
pub enum Features {
    CircularHoleFeature,
    CapsularHoleFeature,
    RectangularHoleFeature,

    FilletFeature,
    ChamferFeature,
    SlopeFeature,
    CliffFeature,

    NotchFeature,
    SawtoothFeature,
    ClawFeature,
    CompositeSlopeFeature,
    
    ArcFeature,
    EllipseFeature,

    OtherFeatureFeature,
}
impl Features {
    /// Returns the name of the feature.
    pub fn name(&self) -> String {
        match self {
            Features::CircularHoleFeature => { "circular hole".to_string() }
            Features::CapsularHoleFeature => { "capsular hole".to_string() }
            Features::RectangularHoleFeature => { "rectangular hole".to_string() }
            Features::FilletFeature => { "fillet".to_string() }
            Features::ChamferFeature => { "chamfer".to_string() }
            Features::SlopeFeature => { "slope".to_string() }
            Features::CliffFeature => { "cliff".to_string() }
            Features::NotchFeature => { "notch".to_string() }
            Features::SawtoothFeature => { "sawtooth".to_string() }
            Features::ClawFeature => { "claw".to_string() }
            Features::CompositeSlopeFeature => { "composite slope".to_string() }
            Features::ArcFeature => { "arc".to_string() }
            Features::EllipseFeature => { "ellipse".to_string() }
            Features::OtherFeatureFeature => { "other feature".to_string() }
        }
    }

    /// Returns the list of steps needed to add a feature for a given type.
    pub fn steps(&self) -> Vec<FeatureAdditionStep> {
        let mut steps = Vec::new();

        match self {
            Features::CircularHoleFeature => {
                steps.push(FeatureAdditionStep::new("diameter".to_string()));
                steps.push(FeatureAdditionStep::new("count".to_string()));
            }

            Features::CapsularHoleFeature => {
                steps.push(FeatureAdditionStep::new("diameter".to_string()));
                steps.push(FeatureAdditionStep::new("width".to_string()));
                steps.push(FeatureAdditionStep::new("count".to_string()));
            }

            Features::RectangularHoleFeature => {
                steps.push(FeatureAdditionStep::new("width".to_string()));
                steps.push(FeatureAdditionStep::new("height".to_string()));
                steps.push(FeatureAdditionStep::new("count".to_string()));
            }

            Features::FilletFeature => {
                steps.push(FeatureAdditionStep::new("radius".to_string()));
                steps.push(FeatureAdditionStep::new("count".to_string()));
            }

            Features::ChamferFeature => {
                steps.push(FeatureAdditionStep::new("size".to_string()));
                steps.push(FeatureAdditionStep::new("count".to_string()));
            }

            Features::SlopeFeature => {
                steps.push(FeatureAdditionStep::new("height".to_string()));
                steps.push(FeatureAdditionStep::new("angle".to_string()));
                steps.push(FeatureAdditionStep::new("count".to_string()));
            }

            Features::CliffFeature => {
                steps.push(FeatureAdditionStep::new("height".to_string()));
                steps.push(FeatureAdditionStep::new("angle".to_string()));
                steps.push(FeatureAdditionStep::new("count".to_string()));
            }

            Features::NotchFeature => {
                steps.push(FeatureAdditionStep::new("depth".to_string()));
                steps.push(FeatureAdditionStep::new("count".to_string()));
            }

            Features::SawtoothFeature => {
                steps.push(FeatureAdditionStep::new("height".to_string()));
                steps.push(FeatureAdditionStep::new("angle".to_string()));
                steps.push(FeatureAdditionStep::new("count".to_string()));
            }

            Features::ClawFeature => {
                steps.push(FeatureAdditionStep::new("height".to_string()));
                steps.push(FeatureAdditionStep::new("angle".to_string()));
                steps.push(FeatureAdditionStep::new("count".to_string()));
            }
            
            Features::CompositeSlopeFeature => {
                steps.push(FeatureAdditionStep::new("height".to_string()));
                steps.push(FeatureAdditionStep::new("angle".to_string()));
                steps.push(FeatureAdditionStep::new("slope type".to_string()));
                steps.push(FeatureAdditionStep::new("slope direction".to_string()));
                steps.push(FeatureAdditionStep::new("slope id".to_string()));
                steps.push(FeatureAdditionStep::new("count".to_string()));
            }

            Features::ArcFeature => {
                steps.push(FeatureAdditionStep::new("radius".to_string()));
                steps.push(FeatureAdditionStep::new("height".to_string()));
                steps.push(FeatureAdditionStep::new("count".to_string()));
            }

            Features::EllipseFeature => {
                steps.push(FeatureAdditionStep::new("width".to_string()));
                steps.push(FeatureAdditionStep::new("height".to_string()));
                steps.push(FeatureAdditionStep::new("count".to_string()));
            }

            Features::OtherFeatureFeature => {
                steps.push(FeatureAdditionStep::new("perimeter modification".to_string()));
                steps.push(FeatureAdditionStep::new("count".to_string()));
            }
        }
        steps
    }

    /// Returns the path for adding the feature.
    pub fn path(&self) -> FeatureAdditionPath {
        FeatureAdditionPath::new(self.clone(), self.steps())
    }
}
impl PartialEq for Features {
    fn eq(&self, other: &Self) -> bool {
        self.name() == other.name()
    }
}


/// Used to define features that can be added to the main Body.
pub trait Feature {
    /// Returns the type of the feature.
    fn shape(&self) -> Features;

    /// Allows type-specific field access
    fn as_any(&self) -> &dyn Any;

    /// Returns how many of the feature there are.
    fn count(&self) -> usize;

    /// Returns a collection of summary information
    fn summarize(&self) -> Vec<String> {
        let mut summary = Vec::new();
        summary.push(self.summarize_feature());
        summary.extend(self.summarize_dimensions());
        summary.push(self.summarize_modification());
        summary
    }

    /// Returns a standardized summary of the feature.
    fn summarize_feature(&self) -> String {
        format!("{}x {}", self.count(), self.shape().name())
    }

    /// Returns a standardized dimension list of the feature.
    fn summarize_dimensions(&self) -> Vec<String>;

    /// Returns a standardized modification summary of the feature.
    fn summarize_modification(&self) -> String {
        format!("modification: {}", &format!("{:.3}", self.value()))
    }

    /// Returns the perimeter modification of the feature.
    /// The perimeter modification is the difference in the body's overall perimeter that this feature will create.
    fn value(&self) -> f64;

    fn print_height(&self) -> u16 {
        self.summarize().len() as u16
    }
}



/// A path object that lists requirements for a given feature to be added.
/// The path also records each value and returns a finished feature when completed.
pub struct FeatureAdditionPath {
    /// The type of feature being added.
    pub feature: Features,
    /// The current step being completed.
    current_step: usize,
    /// The list of steps required to create the feature.
    steps: Vec<FeatureAdditionStep>,
}
impl FeatureAdditionPath {
    /// Creates a new path for adding a feature.
    pub fn new(feature: Features, steps: Vec<FeatureAdditionStep>) -> Self { Self { feature, current_step: 0, steps } }

    /// Returns the current step's field.
    pub fn current_step_value(&self) -> String { self.steps[self.current_step].field.clone() }

    /// Returns the current step's field.
    pub fn current_step_value_input(&self) -> String { self.steps[self.current_step].value_input.clone() }

    /// Updates the current step's value input.
    pub fn update_current_step_value_input(&mut self, new_value_input: String) { self.steps[self.current_step].update_value_input(new_value_input); }

    /// Finishes the current step and returns the completed feature if the last step was completed.
    pub fn finish_current_step(&mut self) -> Option<Box<dyn Feature>> {
        self.steps[self.current_step].finish();
        if self.current_step < self.steps.len() - 1 {
            self.current_step += 1;
            None
        }
        else {
            Some(self.create_feature())
        }
    }

    /// Creates a new feature.
    pub fn create_feature(&self) -> Box<dyn Feature> {
        match self.feature {
            Features::CircularHoleFeature => {
                let diameter = self.steps[0].value;
                let count = self.steps[1].value as usize;
                Box::new(CircularHole::new(diameter, count))
            }

            Features::CapsularHoleFeature => {
                let diameter = self.steps[0].value;
                let width = self.steps[1].value;
                let count = self.steps[2].value as usize;
                Box::new(CapsularHole::new(diameter, width, count))
            }

            Features::RectangularHoleFeature => {
                let width = self.steps[0].value;
                let height = self.steps[1].value;
                let count = self.steps[2].value as usize;
                Box::new(RectangularHole::new(width, height, count))
            }

            Features::FilletFeature => {
                let radius = self.steps[0].value;
                let count = self.steps[1].value as usize;
                Box::new(Fillet::new(radius, count))
            }

            Features::ChamferFeature => {
                let size = self.steps[0].value;
                let count = self.steps[1].value as usize;
                Box::new(Chamfer::new(size, count))
            }

            Features::SlopeFeature => {
                let height = self.steps[0].value;
                let angle = self.steps[1].value;
                let count = self.steps[2].value as usize;
                Box::new(Slope::new(height, angle, count))
            }

            Features::CliffFeature => {
                let height = self.steps[0].value;
                let angle = self.steps[1].value;
                let count = self.steps[2].value as usize;
                Box::new(Cliff::new(height, angle, count))
            }

            Features::NotchFeature => {
                let depth = self.steps[0].value;
                let count = self.steps[1].value as usize;
                Box::new(Notch::new(depth, count))
            }

            Features::SawtoothFeature => {
                let height = self.steps[0].value;
                let angle = self.steps[1].value;
                let count = self.steps[2].value as usize;
                Box::new(Sawtooth::new(height, angle, count))
            }

            Features::ClawFeature => {
                let height = self.steps[0].value;
                let angle = self.steps[1].value;
                let count = self.steps[2].value as usize;
                Box::new(Claw::new(height, angle, count))
            }

            Features::CompositeSlopeFeature => {
                let height = self.steps[0].value;
                let angle = self.steps[1].value;
                let slope_type = match self.steps[2].value as i32 {
                    0 => SlopeType::Convex,
                    _ => SlopeType::Concave,
                };
                let slope_direction = match self.steps[3].value as i32 {
                    0 => SlopeDirection::Up,
                    _ => SlopeDirection::Down,
                };
                let slope_id = self.steps[4].value as usize;
                let count = self.steps[5].value as usize;
                Box::new(CompositeSlope::new(height, angle, slope_type, slope_direction, slope_id, count))
            }

            Features::ArcFeature => {
                let radius = self.steps[0].value;
                let height = self.steps[1].value;
                let count = self.steps[2].value as usize;
                Box::new(Arc::new(radius, height, count))
            }

            Features::EllipseFeature => {
                let width = self.steps[0].value;
                let height = self.steps[1].value;
                let count = self.steps[2].value as usize;
                Box::new(Ellipse::new(width, height, count))
            }

            Features::OtherFeatureFeature => {
                let modification = self.steps[0].value;
                let count = self.steps[1].value as usize;
                Box::new(OtherFeature::new(modification, count))
            }
        }
    }
}



/// A step in a feature addition path.
pub struct FeatureAdditionStep {
    /// One field for a feature of a given type.
    field: String,
    /// The value of the field.
    value: f64,
    /// The user input for the field.
    pub value_input: String,
}
impl FeatureAdditionStep {
    /// Creates a new step.
    pub fn new(field: String) -> Self { Self { field, value: 0.0, value_input: "".to_string() } }

    /// Updates the value of the field.
    pub fn update_value_input(&mut self, new_value_input: String) { self.value_input = new_value_input; }

    /// Finishes the step and parses the value input.
    pub fn finish(&mut self) { self.value = self.value_input.parse::<f64>().unwrap_or(0.0); }
}



/// The main Body that contains a list of features.
/// Together the body and its features make up a full shape.
/// Every feature should fit inside the body's width and height bounds.
pub struct Body {
    /// The name of the Body.
    pub name: String,
    /// The overall width of the shape.
    pub width: f64,
    /// The overall height of the shape.
    pub height: f64,
    /// The list of modifying features.
    pub features: Vec<Box<dyn Feature>>,
}
impl Body {
    /// Creates a new Body.
    pub fn new() -> Self { Body {name: "".to_string(), width: 0.0, height: 0.0, features: Vec::new() } }

    /// Renames the Body.
    pub fn rename(&mut self, new_name: String) { self.name = new_name; }

    /// Sets the overall width of the body.
    pub fn set_width(&mut self, width: f64) { self.width = width; }

    /// Sets the overall height of the body.
    pub fn set_height(&mut self, height: f64) { self.height = height; }

    /// Adds a feature.
    pub fn add(&mut self, feature: Box<dyn Feature>) { self.features.push(feature); }

    /// Removes a feature
    pub fn remove(&mut self, index: usize) {
        if self.features.len() >= index { self.features.remove(index); }
    }

    /// Gets the full perimeter of the body with all of its features.
    pub fn perimeter(&self) -> f64 {
        let mut perimeter = (self.width + self.height) * 2.0;

        let mut composite_slope_index = CompositeSlopeIndex::new();

        for feature in &self.features {
            if let Some(composite_slope) = feature.as_any().downcast_ref::<CompositeSlope>() {
                composite_slope_index.add_height(composite_slope.height, composite_slope.slope_direction, composite_slope.slope_id);
            }
            else {
                perimeter += feature.value();
            }
        }

        perimeter -= composite_slope_index.get_height_differences();

        perimeter
    }

    /// Summarizes the body
    pub fn summarize(&self) -> Vec<String> {
        let mut summary = Vec::new();
        let name = format!("name: {}", &self.name);
        let width = format!("width: {}, ", &format!("{:.3}", self.width));
        let height = format!("height: {}", &format!("{:.3}", self.height));
        let perimeter = format!("perimeter: {}", &format!("{:.3}", self.perimeter()));
        summary.push(name);
        summary.push(width);
        summary.push(height);
        summary.push(perimeter);

        summary
    }
}



/// A hole that adds to the body's overall perimeter.
pub struct CircularHole {
    /// The diameter of the hole.
    diameter: f64,
    /// How many holes there are.
    count: usize,
}
impl CircularHole {
    /// Creates a new hole feature.
    pub fn new(diameter: f64, count: usize) -> Self { Self { diameter, count } }
}
impl Feature for CircularHole {
    /// The type of the feature.
    fn shape(&self) -> Features { Features::CircularHoleFeature }

    /// Allows type-specific field access
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> Vec<String> {
        let mut dims = Vec::new();
        let diameter = format!("diameter: {}", format!("{:.3}", self.diameter));
        dims.push(diameter);
        dims
    }

    /// Gets the perimeter modification of the hole.
    fn value(&self) -> f64 {
        formulas::circular_hole_modification(self.diameter) * self.count as f64
    }
}



/// A capsule shaped hole that adds to the body's overall perimeter.
pub struct CapsularHole {
    /// The diameter of the hole.
    diameter: f64,
    /// The total width of the hole.
    width: f64,
    /// How many holes there are.
    count: usize,
}
impl CapsularHole {
    /// Creates a new hole feature.
    pub fn new(diameter: f64, width: f64, count: usize) -> Self { Self { diameter, width, count } }
}
impl Feature for CapsularHole {
    /// The type of the feature.
    fn shape(&self) -> Features { Features::CapsularHoleFeature }

    /// Allows type-specific field access
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> Vec<String> {
        let mut dims = Vec::new();
        let diameter = format!("diameter: {}, ", &format!("{:.3}", self.diameter));
        let width = format!("width: {}", &format!("{:.3}", self.width));
        dims.push(diameter);
        dims.push(width);
        dims
    }

    /// Gets the perimeter modification of the hole.
    fn value(&self) -> f64 {
        formulas::capsular_hole_modification(self.diameter, self.width) * self.count as f64
    }
}



/// A rectangular hole that adds to the body's overall perimeter.
pub struct RectangularHole {
    /// The width of the hole.
    width: f64,
    /// The height of the hole.
    height: f64,
    /// How many holes there are.
    count: usize,
}
impl RectangularHole {
    /// Creates a new hole feature.
    pub fn new(width: f64, height: f64, count: usize) -> Self { Self { width, height, count } }
}
impl Feature for RectangularHole {
    /// The type of the feature.
    fn shape(&self) -> Features { Features::RectangularHoleFeature }

    /// Allows type-specific field access
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> Vec<String> {
        let mut dims = Vec::new();
        let width = format!("width: {}, ", &format!("{:.3}", self.width));
        let height = format!("height: {}", &format!("{:.3}", self.height));
        dims.push(width);
        dims.push(height);
        dims
    }

    /// Gets the perimeter modification of the hole.
    fn value(&self) -> f64 {
        formulas::rectangular_hole_modification(self.width, self.height) * self.count as f64
    }
}



/// A simple fillet.
pub struct Fillet {
    /// The radius of the fillet.
    radius: f64,
    /// How many fillets there are.
    count: usize,
}
impl Fillet {
    /// Creates a new fillet feature.
    pub fn new(radius: f64, count: usize) -> Self { Self { radius, count } }
}
impl Feature for Fillet {
    /// The type of the feature.
    fn shape(&self) -> Features { Features::FilletFeature }

    /// Allows type-specific field access
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> Vec<String> {
        let mut dims = Vec::new();
        let radius = format!("radius: {}", &format!("{:.3}", self.radius));
        dims.push(radius);
        dims
    }

    /// Gets the perimeter modification of the fillet.
    fn value(&self) -> f64 {
        formulas::fillet_modification(self.radius) * self.count as f64
    }
}



/// A simple chamfer.
pub struct Chamfer {
    /// The size of the fillet (equal along the width and height of the body).
    size: f64,
    /// How many fillets there are.
    count: usize,
}
impl Chamfer {
    /// Creates a new fillet feature.
    pub fn new(size: f64, count: usize) -> Self { Self { size, count } }
}
impl Feature for Chamfer {
    /// The name of the feature.
    fn shape(&self) -> Features { Features::ChamferFeature }

    /// Allows type-specific field access
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> Vec<String> {
        let mut dims = Vec::new();
        let size = format!("size: {}", &format!("{:.3}", self.size));
        dims.push(size);
        dims
    }

    /// Gets the perimeter modification of the chamfer.
    fn value(&self) -> f64 {
        formulas::chamfer_modification(self.size) * self.count as f64
    }
}



/// A convex sloping feature.
pub struct Slope {
    /// The height of the slope.
    height: f64,
    /// The angle of the slope.
    angle: f64,
    /// How many notches there are.
    count: usize,
}
impl Slope {
    /// Creates a new slope feature.
    pub fn new(height: f64, angle: f64, count: usize) -> Self { Self { height, angle, count } }
}
impl Feature for Slope {
    /// The type of the feature.
    fn shape(&self) -> Features { Features::SlopeFeature }

    /// Allows type-specific field access
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> Vec<String> {
        let mut dims = Vec::new();
        let height = format!("height: {}, ", &format!("{:.3}", self.height));
        let angle = format!("angle: {}", &format!("{:.3}", self.angle));
        dims.push(height);
        dims.push(angle);
        dims
    }

    /// Gets the perimeter modification of the slope.
    fn value(&self) -> f64 {
        formulas::slope_modification(self.height, self.angle) * self.count as f64
    }
}



/// A concave sloping feature.
pub struct Cliff {
    /// The height of the cliff.
    height: f64,
    /// The angle of the cliff.
    angle: f64,
    /// How many cliffs there are.
    count: usize,
}
impl Cliff {
    /// Creates a new cliff feature.
    pub fn new(height: f64, angle: f64, count: usize) -> Self { Self { height, angle, count } }
}
impl Feature for Cliff {
    /// The type of the feature.
    fn shape(&self) -> Features { Features::CliffFeature }

    /// Allows type-specific field access
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> Vec<String> {
        let mut dims = Vec::new();
        let height = format!("height: {}, ", &format!("{:.3}", self.height));
        let angle = format!("angle: {}", &format!("{:.3}", self.angle));
        dims.push(height);
        dims.push(angle);
        dims
    }

    /// Gets the perimeter modification of the cliff.
    fn value(&self) -> f64 {
        formulas::cliff_modification(self.height, self.angle) * self.count as f64
    }
}



/// A rectangular cutout.
pub struct Notch {
    /// The depth of the notch.
    depth: f64,
    /// How many notches there are.
    count: usize,
}
impl Notch {
    /// Creates a new notch feature.
    pub fn new(depth: f64, count: usize) -> Self { Notch { depth, count } }
}
impl Feature for Notch {
    /// The type of the feature.
    fn shape(&self) -> Features { Features::NotchFeature }

    /// Allows type-specific field access
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> Vec<String> {
        let mut dims = Vec::new();
        let depth = format!("depth: {}", &format!("{:.3}", self.depth));
        dims.push(depth);
        dims
    }

    /// Gets the perimeter modification of the notch.
    fn value(&self) -> f64 {
        formulas::notch_modification(self.depth) * self.count as f64
    }
}



/// A cutout that is straight on one side and slopes inward on the other.
pub struct Sawtooth {
    /// The height of the sawtooth.
    height: f64,
    /// The angle of the sloped side of the sawtooth.
    angle: f64,
    /// How many sawteeth there are.
    count: usize,
}
impl Sawtooth {
    /// Creates a new sawtooth feature.
    pub fn new(height: f64, angle: f64, count: usize) -> Self { Self { height, angle, count } }
}
impl Feature for Sawtooth {
    /// The type of the feature.
    fn shape(&self) -> Features { Features::SawtoothFeature }

    /// Allows type-specific field access
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> Vec<String> {
        let mut dims = Vec::new();
        let height = format!("height: {}, ", &format!("{:.3}", self.height));
        let angle = format!("angle: {}", &format!("{:.3}", self.angle));
        dims.push(height);
        dims.push(angle);
        dims
    }

    /// Gets the perimeter modification of the valley.
    fn value(&self) -> f64 {
        formulas::sawtooth_modification(self.height, self.angle) * self.count as f64
    }
}



/// A cutout that is straight on one side and cuts backwards and downwards on the other.
pub struct Claw {
    /// The height of the claw.
    height: f64,
    /// The angle of the sloped side of the sawtooth.
    angle: f64,
    /// How many sawteeth there are.
    count: usize,
}
impl Claw {
    /// Creates a new sawtooth feature.
    pub fn new(height: f64, angle: f64, count: usize) -> Self { Self { height, angle, count } }
}
impl Feature for Claw {
    /// The type of the feature.
    fn shape(&self) -> Features { Features::ClawFeature }

    /// Allows type-specific field access
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> Vec<String> {
        let mut dims = Vec::new();
        let height = format!("height: {}, ", &format!("{:.3}", self.height));
        let angle = format!("angle: {}", &format!("{:.3}", self.angle));
        dims.push(height);
        dims.push(angle);
        dims
    }

    /// Gets the perimeter modification of the valley.
    fn value(&self) -> f64 {
        formulas::claw_modification(self.height, self.angle) * self.count as f64
    }
}



/// Denotes which direction the slope is going in.
#[derive(Clone, Copy)]
pub enum SlopeDirection {
    Up,
    Down,
}



/// Denotes whether the slope is convex or concave.
#[derive(Clone, Copy)]
pub enum SlopeType {
    Convex,
    Concave,
}



/// Stores and automatically manages the height differences for various collections of composite slopes.
pub struct CompositeSlopeIndex {
    /// A collection of all the composite slope height trackers.
    pub trackers: Vec<CompositeSlopeHeightTracker>
}
impl CompositeSlopeIndex {
    /// Creates a new composite slope index.
    pub fn new() -> Self { Self { trackers: Vec::new() } }

    pub fn add_height(&mut self, height: f64, slope_direction: SlopeDirection, slope_id: usize) {
        let tracker = self.trackers.iter_mut().find(|tracker| tracker.slope_id == slope_id);
        if let Some(existing_tracker) = tracker {
            existing_tracker.add_height(height, slope_direction);
        } else {
            let mut new_tracker = CompositeSlopeHeightTracker::new(slope_id);
            new_tracker.add_height(height, slope_direction);
            self.trackers.push(new_tracker);
        }
    }

    pub fn get_height_differences(&self) -> f64 {
        self.trackers.iter().map(|tracker| tracker.get_height_difference()).sum()
    }
}


/// A height tracker for a single collection of composite slopes.
pub struct CompositeSlopeHeightTracker {
    /// The id of the collection of composite slopes.
    slope_id: usize,
    /// The difference in height from the series of composite slopes.
    height_tracker: f64,
}
impl CompositeSlopeHeightTracker {
    /// Creates a new height tracker for a collection of composite slopes.
    pub fn new(slope_id: usize) -> Self { Self { slope_id, height_tracker: 0.0 } }

    /// Adds a height to the height tracker, keeping track of the direction of the slope.
    pub fn add_height(&mut self, height: f64, slope_direction: SlopeDirection) {
        match slope_direction {
            SlopeDirection::Up => self.height_tracker -= height,
            SlopeDirection::Down => self.height_tracker += height,
        }
    }

    /// Returns the difference in height from the last height tracker.
    pub fn get_height_difference(&self) -> f64 { self.height_tracker.abs() }
}



/// A cutout that is a collection of individual composite slopes.
/// These slopes are groups by id's with each id being meant to designate an entire complex slope or valley.
pub struct CompositeSlope {
    /// The height of the composite slope (only current section).
    height: f64,
    /// The angle of the sloped side of the composite slope.
    angle: f64,
    /// Determined whether the composite slope is convex or concave.
    slope_type: SlopeType,
    /// The direction of the composite slope.
    /// Down goes into the body and Up goes out of the body.
    slope_direction: SlopeDirection,
    /// Slope id
    slope_id: usize,
    /// How many composite slopes there are.
    count: usize,
}
impl CompositeSlope {
    /// Creates a new composite slope feature.
    pub fn new(height: f64, angle: f64, slope_type: SlopeType, slope_direction: SlopeDirection, slope_id: usize, count: usize) -> Self { Self { height, angle, slope_type, slope_direction, slope_id, count } }
}
impl Feature for CompositeSlope {
    /// The type of the feature.
    fn shape(&self) -> Features { Features::CompositeSlopeFeature }

    /// Allows type-specific field access
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> Vec<String> {
        let mut dims = Vec::new();
        let height = format!("height: {}, ", &format!("{:.3}", self.height));
        let angle = format!("angle: {}, ", &format!("{:.3}", self.angle));
        let slope_type = format!("slope_type: {}, ", match self.slope_type { SlopeType::Convex => "convex", SlopeType::Concave => "concave" });
        let slope_direction = format!("slope_direction: {}, ", match self.slope_direction { SlopeDirection::Up => "up", SlopeDirection::Down => "down" });
        let slope_id = format!("slope_id: {}", self.slope_id);
        dims.push(height);
        dims.push(angle);
        dims.push(slope_type);
        dims.push(slope_direction);
        dims.push(slope_id);
        dims
    }

    /// Gets the perimeter modification of the valley.
    fn value(&self) -> f64 {
        formulas::composite_slope_modification(self.height, self.angle, self.slope_type) * self.count as f64
    }
}



/// An arc shaped cutout.
pub struct Arc {
    /// The radius of the circle that the arc comes from.
    radius: f64,
    /// The height of the arc cutout from the middle.
    height: f64,
    /// How many arcs there are.
    count: usize,
}
impl Arc {
    /// Creates a new arc feature.
    pub fn new(radius: f64, height: f64, count: usize) -> Self { Self { radius, height, count } }
}
impl Feature for Arc {
    /// The type of the feature.
    fn shape(&self) -> Features { Features::ArcFeature }

    /// Allows type-specific field access
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> Vec<String> {
        let mut dims = Vec::new();
        let radius = format!("radius: {}, ", &format!("{:.3}", self.radius));
        let height = format!("height: {}", &format!("{:.3}", self.height));
        dims.push(radius);
        dims.push(height);
        dims
    }

    /// Gets the perimeter modification of the arc.
    fn value(&self) -> f64 {
        formulas::arc_modification(self.radius, self.height) * self.count as f64
    }
}



/// An elliptical cutout.
pub struct Ellipse {
    /// The width of the cutout.
    width: f64,
    /// The height of the cutout (not the height of the full ellipse).
    height: f64,
    /// How many ellipses there are.
    count: usize,
}
impl Ellipse {
    /// Creates a new ellipse.
    pub fn new(width: f64, height: f64, count: usize) -> Self { Self { width, height, count } }
}
impl Feature for Ellipse {
    /// The type of the feature.
    fn shape(&self) -> Features { Features::EllipseFeature }

    /// Allows type-specific field access
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> Vec<String> {
        let mut dims = Vec::new();
        let width = format!("width: {}, ", &format!("{:.3}", self.width));
        let height = format!("height: {}", &format!("{:.3}", self.height));
        dims.push(width);
        dims.push(height);
        dims
    }

    /// Gets the perimeter modification of the ellipse.
    fn value(&self) -> f64 {
        formulas::ellipse_modification(self.width, self.height) * self.count as f64
    }
}



/// A catch-all other feature.
pub struct OtherFeature {
    /// Because this is a general catch-all feature, it is only for manual perimeter modifications.
    perimeter_modification: f64,
    /// How of these features there are.
    count: usize,
}
impl OtherFeature {
    /// Creates a new hole feature.
    pub fn new(perimeter_modification: f64, count: usize) -> Self { Self { perimeter_modification, count } }
}
impl Feature for OtherFeature {
    /// The type of the feature.
    fn shape(&self) -> Features { Features::OtherFeatureFeature }

    /// Allows type-specific field access
    fn as_any(&self) -> &dyn Any {
        self
    }

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> Vec<String> {
        let mut dims = Vec::new();
        let count = format!("count: {}", self.count);
        dims.push(count);
        dims
    }

    /// Gets the perimeter modification of the catch-all feature.
    fn value(&self) -> f64 {
        self.perimeter_modification
    }
}



/// A collection of perimeter modification formulas for various features.
pub mod formulas {
    use std::f64::consts::PI;
    use crate::shapes::SlopeType;

    /// Calculates the perimeter modification for a circular hole.
    pub fn circular_hole_modification(diameter: f64) -> f64 {
        diameter * PI
    }

    /// Calculates the perimeter modification for a capsular hole.
    pub fn capsular_hole_modification(diameter: f64, width: f64) -> f64 {
        circular_hole_modification(diameter) + (2.0 * (width - diameter))
    }

    /// Calculates the perimeter modification for a rectangular hole.
    pub fn rectangular_hole_modification(width: f64, height: f64) -> f64 {
        (width + height) * 2.0
    }



    /// Calculates the perimeter modification for a fillet.
    pub fn fillet_modification(radius: f64) -> f64 {
        ((PI * radius) / 2.0) - (radius * 2.0)
    }

    /// Calculates the perimeter modification for a chamfer.
    pub fn chamfer_modification(size: f64) -> f64 {
        slope_modification(size, 45.0)
    }

    /// Calculates the perimeter modification for a slope.
    pub fn slope_modification(height: f64, angle: f64) -> f64 {
        let rad_angle = angle.to_radians();
        ((1.0 / rad_angle.sin()) - (1.0 / rad_angle.tan()) - 1.0) * height
    }

    /// Calculates the perimeter modification for a cliff.
    pub fn cliff_modification(height: f64, angle: f64) -> f64 {
        let rad_angle = angle.to_radians();
        ((1.0 / rad_angle.sin()) + (1.0 / rad_angle.tan()) - 1.0) * height
    }



    /// Calculates the perimeter modification for a notch.
    pub fn notch_modification(depth: f64) -> f64 {
        depth * 2.0
    }

    /// Calculates the perimeter modification for a sawtooth.
    pub fn sawtooth_modification(height: f64, angle: f64) -> f64 {
        slope_modification(height, angle) + height
    }

    /// Calculates the perimeter modification for a claw.
    pub fn claw_modification(height: f64, angle: f64) -> f64 {
        cliff_modification(height, angle) + height
    }
    
    /// Calculates the perimeter modification for a composite slope.
    pub fn composite_slope_modification(height: f64, angle: f64, slope_type: SlopeType) -> f64 {
        let rad_angle = angle.to_radians();
        match slope_type {
            SlopeType::Convex => { ((1.0 / rad_angle.sin()) - (1.0 / rad_angle.tan())) * height }
            SlopeType::Concave => { ((1.0 / rad_angle.sin()) + (1.0 / rad_angle.tan())) * height }
        }
    }



    /// Calculates the perimeter modification for a circular arc.
    pub fn arc_modification(radius: f64, height: f64) -> f64 {
        let arc = radius * 2.0 * ((radius - height) / radius).acos();
        let width = 2.0 * (height * ((2.0 * radius) - height)).sqrt();
        arc - width
    }

    /// Calculates the perimeter modification for an ellipse.
    pub fn ellipse_modification(width: f64, height: f64) -> f64 {
        let ellipse = PI * (3.0 * ( (width/2.0) + height) - ( (((3.0*width)/2.0) + height) * ((width/2.0) + (3.0*height)) ).sqrt() );
        (ellipse / 2.0) - width
    }
}