use std::f64::consts::PI;
use std::net::ToSocketAddrs;
use crate::shapes::Features::SawtoothFeature;

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
    ValleyFeature,
    SinkholeFeature,
    CaveFeature,

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
            Features::ValleyFeature => { "valley".to_string() }
            Features::SinkholeFeature => { "sinkhole".to_string() }
            Features::CaveFeature => { "cave".to_string() }
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

            Features::ValleyFeature => {
                steps.push(FeatureAdditionStep::new("height 1".to_string()));
                steps.push(FeatureAdditionStep::new("angle 1".to_string()));
                steps.push(FeatureAdditionStep::new("height 2".to_string()));
                steps.push(FeatureAdditionStep::new("angle 2".to_string()));
                steps.push(FeatureAdditionStep::new("count".to_string()));
            }

            Features::SinkholeFeature => {
                steps.push(FeatureAdditionStep::new("height 1".to_string()));
                steps.push(FeatureAdditionStep::new("angle 1".to_string()));
                steps.push(FeatureAdditionStep::new("height 2".to_string()));
                steps.push(FeatureAdditionStep::new("angle 2".to_string()));
                steps.push(FeatureAdditionStep::new("count".to_string()));
            }

            Features::CaveFeature => {
                steps.push(FeatureAdditionStep::new("height 1".to_string()));
                steps.push(FeatureAdditionStep::new("angle 1".to_string()));
                steps.push(FeatureAdditionStep::new("height 2".to_string()));
                steps.push(FeatureAdditionStep::new("angle 2".to_string()));
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


/// Used to define features that can be added to the main Body.
pub trait Feature {
    /// Returns the type of the feature.
    fn shape(&self) -> Features;

    /// Returns how many of the feature there are.
    fn count(&self) -> usize;

    /// Returns a standardized summary of the feature.
    fn summarize_feature(&self) -> String {
        let mut sum = "".to_string();
        sum += &format!("{}x {}", self.count(), Self::shape(&self).name());
        sum
    }

    /// Returns a standardized dimension list of the feature.
    fn summarize_dimensions(&self) -> String;

    /// Returns a standardized modification summary of the feature.
    fn summarize_modification(&self) -> String {
        let mut sum = "".to_string();
        sum += &format!("modification: {} ", &format!("{:.3}", self.value()));
        sum
    }

    /// Returns the perimeter modification of the feature.
    /// The perimeter modification is the difference in body's overall perimeter that this feature will create.
    fn value(&self) -> f64;
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

            Features::ValleyFeature => {
                let height_1 = self.steps[0].value;
                let angle_1 = self.steps[1].value;
                let height_2 = self.steps[2].value;
                let angle_2 = self.steps[3].value;
                let count = self.steps[4].value as usize;
                Box::new(Valley::new(height_1, angle_1, height_2, angle_2, count))
            }

            Features::SinkholeFeature => {
                let height_1 = self.steps[0].value;
                let angle_1 = self.steps[1].value;
                let height_2 = self.steps[2].value;
                let angle_2 = self.steps[3].value;
                let count = self.steps[4].value as usize;
                Box::new(Sinkhole::new(height_1, angle_1, height_2, angle_2, count))
            }

            Features::CaveFeature => {
                let slope_height = self.steps[0].value;
                let slope_angle = self.steps[1].value;
                let cliff_height = self.steps[2].value;
                let cliff_angle = self.steps[3].value;
                let count = self.steps[4].value as usize;
                Box::new(Cave::new(slope_height, slope_angle, cliff_height, cliff_angle, count))
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

    /// Gets the full perimeter of the body with all of its features.
    pub fn perimeter(&self) -> f64 {
        let mut perimeter = (self.width + self.height) * 2.0;
        for feature in &self.features {
            perimeter += feature.value();
        }
        perimeter
    }

    /// Adds a feature.
    pub fn add(&mut self, feature: Box<dyn Feature>) { self.features.push(feature); }

    /// Removes a feature
    pub fn remove(&mut self, index: usize) {
        if self.features.len() >= index { self.features.remove(index); }
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

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> String {
        let mut dims = "".to_string();
        dims += &format!("diameter: {}", &format!("{:.3}", self.diameter));
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

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> String {
        let mut dims = "".to_string();
        dims += &format!("diameter: {}, ", &format!("{:.3}", self.diameter));
        dims += &format!("width: {}", &format!("{:.3}", self.width));
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

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> String {
        let mut dims = "".to_string();
        dims += &format!("width: {}, ", &format!("{:.3}", self.width));
        dims += &format!("height: {}", &format!("{:.3}", self.height));
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

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> String {
        let mut dims = "".to_string();
        dims += &format!("radius: {}", &format!("{:.3}", self.radius));
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

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> String {
        let mut dims = "".to_string();
        dims += &format!("size: {}", &format!("{:.3}", self.size));
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

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> String {
        let mut dims = "".to_string();
        dims += &format!("height: {}, ", &format!("{:.3}", self.height));
        dims += &format!("angle: {}", &format!("{:.3}", self.angle));
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

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> String {
        let mut dims = "".to_string();
        dims += &format!("height: {}, ", &format!("{:.3}", self.height));
        dims += &format!("angle: {}", &format!("{:.3}", self.angle));
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

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> String {
        let mut dims = "".to_string();
        dims += &format!("depth: {}", &format!("{:.3}", self.depth));
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

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> String {
        let mut dims = "".to_string();
        dims += &format!("height: {}, ", &format!("{:.3}", self.height));
        dims += &format!("angle: {}", &format!("{:.3}", self.angle));
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

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> String {
        let mut dims = "".to_string();
        dims += &format!("height: {}, ", &format!("{:.3}", self.height));
        dims += &format!("angle: {}", &format!("{:.3}", self.angle));
        dims
    }

    /// Gets the perimeter modification of the valley.
    fn value(&self) -> f64 {
        formulas::claw_modification(self.height, self.angle) * self.count as f64
    }
}



/// A cutout that slopes inward on both sides (2x slopes).
pub struct Valley {
    /// The height of the first slope.
    height_1: f64,
    /// The angle of the first slope.
    angle_1: f64,
    /// The height of the second slope.
    height_2: f64,
    /// The angle of the second slope.
    angle_2: f64,
    /// How many valleys there are.
    count: usize,
}
impl Valley {
    /// Creates a new valley feature.
    pub fn new(height_1: f64, angle_1: f64, height_2: f64, angle_2: f64, count: usize) -> Self { Self { height_1, angle_1, height_2, angle_2, count } }
}
impl Feature for Valley {
    /// The type of the feature.
    fn shape(&self) -> Features { Features::ValleyFeature }

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> String {
        let mut dims = "".to_string();
        dims += &format!("height 1: {}, ", &format!("{:.3}", self.height_1));
        dims += &format!("angle 1: {}, ", &format!("{:.3}", self.angle_1));
        dims += &format!("height 2: {}, ", &format!("{:.3}", self.height_2));
        dims += &format!("angle 2: {}", &format!("{:.3}", self.angle_2));
        dims
    }

    /// Gets the perimeter modification of the valley.
    fn value(&self) -> f64 {
        formulas::valley_modification(self.height_1, self.angle_1, self.height_2, self.angle_2) * self.count as f64
    }
}



/// A cutout that slopes backwards and downwards on both sides (2x claws).
pub struct Sinkhole {
    /// The height of the first slope.
    height_1: f64,
    /// The angle of the first slope.
    angle_1: f64,
    /// The height of the second slope.
    height_2: f64,
    /// The angle of the second slope.
    angle_2: f64,
    /// How many valleys there are.
    count: usize,
}
impl Sinkhole {
    /// Creates a new sinkhole feature.
    pub fn new(height_1: f64, angle_1: f64, height_2: f64, angle_2: f64, count: usize) -> Self { Self { height_1, angle_1, height_2, angle_2, count } }
}
impl Feature for Sinkhole {
    /// The type of the feature.
    fn shape(&self) -> Features { Features::SinkholeFeature }

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> String {
        let mut dims = "".to_string();
        dims += &format!("height 1: {}, ", &format!("{:.3}", self.height_1));
        dims += &format!("angle 1: {}, ", &format!("{:.3}", self.angle_1));
        dims += &format!("height 2: {}, ", &format!("{:.3}", self.height_2));
        dims += &format!("angle 2: {}", &format!("{:.3}", self.angle_2));
        dims
    }

    /// Gets the perimeter modification of the valley.
    fn value(&self) -> f64 {
        formulas::sinkhole_modification(self.height_1, self.angle_1, self.height_2, self.angle_2) * self.count as f64
    }
}



/// A cutout that slopes inward on one side (slope) and backwards and downwards on the other (claw).
pub struct Cave {
    /// The height of the first slope.
    slope_height: f64,
    /// The angle of the first slope.
    slope_angle: f64,
    /// The height of the second slope.
    cliff_height: f64,
    /// The angle of the second slope.
    cliff_angle: f64,
    /// How many valleys there are.
    count: usize,
}
impl Cave {
    /// Creates a new valley feature.
    pub fn new(slope_height: f64, slope_angle: f64, cliff_height: f64, cliff_angle: f64, count: usize) -> Self { Self { slope_height, slope_angle, cliff_height, cliff_angle, count } }
}
impl Feature for Cave {
    /// The type of the feature.
    fn shape(&self) -> Features { Features::CaveFeature }

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> String {
        let mut dims = "".to_string();
        dims += &format!("height 1: {}, ", &format!("{:.3}", self.slope_height));
        dims += &format!("angle 1: {}, ", &format!("{:.3}", self.slope_angle));
        dims += &format!("height 2: {}, ", &format!("{:.3}", self.cliff_height));
        dims += &format!("angle 2: {}", &format!("{:.3}", self.cliff_angle));
        dims
    }

    /// Gets the perimeter modification of the valley.
    fn value(&self) -> f64 {
        formulas::cave_modification(self.slope_height, self.slope_angle, self.cliff_height, self.cliff_angle) * self.count as f64
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

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> String {
        let mut dims = "".to_string();
        dims += &format!("radius: {}, ", &format!("{:.3}", self.radius));
        dims += &format!("height: {}", &format!("{:.3}", self.height));
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

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> String {
        let mut dims = "".to_string();
        dims += &format!("width: {}, ", &format!("{:.3}", self.width));
        dims += &format!("height: {}", &format!("{:.3}", self.height));
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

    /// The count of the feature.
    fn count(&self) -> usize { self.count }

    /// A basic dimension overview.
    fn summarize_dimensions(&self) -> String { "no dimensions".to_string() }

    /// Gets the perimeter modification of the catch-all feature.
    fn value(&self) -> f64 {
        self.perimeter_modification
    }
}



/// A collection of perimeter modification formulas for various features.
pub mod formulas {
    use std::f64::consts::PI;

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
        ((1.0 / rad_angle.sin()) - (1.0 / rad_angle.tan())) * height
    }

    /// Calculates the perimeter modification for a cliff.
    pub fn cliff_modification(height: f64, angle: f64) -> f64 {
        let rad_angle = angle.to_radians();
        ((1.0 / rad_angle.sin()) + (1.0 / rad_angle.tan())) * height
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

    /// Calculates the perimeter modification for a valley.
    pub fn valley_modification(height_1: f64, angle_1: f64, height_2: f64, angle_2: f64) -> f64 {
        slope_modification(height_1, angle_1) + slope_modification(height_2, angle_2)
    }

    /// Calculates the perimeter modification for a sinkhole.
    pub fn sinkhole_modification(height_1: f64, angle_1: f64, height_2: f64, angle_2: f64) -> f64 {
        cliff_modification(height_1, angle_1) + cliff_modification(height_2, angle_2)
    }

    /// Calculates the perimeter modification for a cave.
    pub fn cave_modification(slope_height: f64, slope_angle: f64, cliff_height: f64, cliff_angle: f64) -> f64 {
        slope_modification(slope_height, slope_angle) + cliff_modification(cliff_height, cliff_angle)
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