pub mod dman;
pub mod err;

use dman::DIAGMAN;
use err::Diagnostic;

pub fn diag(diag: Box<dyn Diagnostic>) {
    DIAGMAN.lock().as_mut().unwrap().publish(diag);
}
