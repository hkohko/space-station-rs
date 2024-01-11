use crate::{Resources, Coordinates};
#[derive(Debug)]
/// Struct for resources available in the environment.
pub struct EnvResource {
    kind: Resources,
    coordinates: Coordinates,
}
impl EnvResource {
    /// Creates a new resource with randomized values.
    pub fn new(at_most: i32) -> EnvResource {
        EnvResource {
            kind: Resources::randomize(at_most),
            coordinates: Coordinates::randomize(),
        }
    }
    /// Returns the resource's kind.
    pub fn get_kind(&self) -> Resources {
        self.kind
    }
    /// Returns the resource's coordinates.
    pub fn get_coordinates(&self) -> Coordinates {
        self.coordinates
    }
}