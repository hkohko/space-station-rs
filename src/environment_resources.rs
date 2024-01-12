use crate::{Resources, Coordinates, TransferResources};
#[derive(Debug)]
/// Struct for resources available in the environment.
pub struct EnvResource {
    kind: Resources,
    coordinates: Coordinates,
}
impl EnvResource {
    pub fn randomize(at_most: i32) -> EnvResource {

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
impl TransferResources for EnvResource {
    fn give_resources(&mut self, _rsc: Resources, _: i32) -> bool {
        match self.get_kind() {
            Resources::FoodWater(val) => {
                if let Resources::FoodWater(rate) = _rsc {
                    if val - rate != -1 {
                        self.kind = Resources::FoodWater(val - rate);
                        true
                    } else {
                        false
                    }
                } else {
                    true
                }
            },
            Resources::Oxygen(val) => {
                if let Resources::Oxygen(rate) = _rsc {
                    if val - rate != -1 {
                        self.kind = Resources::Oxygen(val - rate);
                        true
                    } else {
                        false
                    }
                } else {
                    true
                }
            },
            Resources::Fuel(val) => {
                if let Resources::Fuel(rate) = _rsc {
                    if val - rate != -1 {
                        self.kind = Resources::Fuel(val - rate);
                        true
                    } else {
                        false
                    }
                } else {
                    true
                }
            },
        }
    }
}