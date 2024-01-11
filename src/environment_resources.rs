use crate::{Resources, Coordinates};
#[derive(Debug)]
pub struct EnvResource {
    kind: Resources,
    coordinates: Coordinates,
}
impl EnvResource {
    pub fn new(at_most: i32) -> EnvResource {
        EnvResource {
            kind: Resources::randomize(at_most),
            coordinates: Coordinates::randomize(),
        }
    }
    pub fn get_kind(&self) -> Resources {
        self.kind
    }
    pub fn get_coordinates(&self) -> Coordinates {
        self.coordinates
    }
}