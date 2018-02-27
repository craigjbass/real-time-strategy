use domain::Unit;
use std::sync::mpsc::Receiver;
use std::sync::Arc;

pub trait UnitGateway {
    fn get_units_stream(&self) -> Arc<Receiver<Unit>>;
}
