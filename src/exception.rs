use ext_php_rs::{prelude::*, zend::ce};

#[php_class]
#[php(name = "LoPdf\\Exception\\LoadException")]
#[php(extends(ce = ce::exception, stub = "\\Exception"))]
#[derive(Default)]
pub struct LoadException;

#[php_impl]
impl LoadException {
    pub fn __construct() -> Self {
        Self {}
    }
}
