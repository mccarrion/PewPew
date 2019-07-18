/**
 * Struct for keyboard input and mouse clicks
 */
#[derive(Debug)]
pub struct InputState {
    pub xaxis: f32,
    pub yaxis: f32,
    pub xclick: f32,
    pub yclick: f32,
    pub fire: bool,
}

/**
 * Base implementation of InputState struct
 */
impl Default for InputState {
    fn default() -> Self {
        InputState {
            xaxis: 0.0,
            yaxis: 0.0,
            xclick: 0.0,
            yclick: 0.0,
            fire: false,
        }
    }
}