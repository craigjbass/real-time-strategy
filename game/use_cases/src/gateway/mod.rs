use domain::Unit;

use std::sync::mpsc::Receiver;

pub trait UnitGateway {
    fn get_units_stream(&self) -> Receiver<Unit>;
}
