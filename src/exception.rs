use ext_php_rs::{prelude::*, zend::ce};

#[php_class]
#[php(name = "LoPdf\\Exception\\Exception")]
#[php(extends(ce = ce::exception, stub = "\\Exception"))]
#[derive(Default)]
pub struct Exception;

#[php_impl]
impl Exception {
    pub fn __construct() -> Self {
        Self {}
    }
}

#[php_class]
#[php(name = "LoPdf\\Exception\\LoadException")]
#[php(extends(ce = ce::exception, stub = "\\LoPdf\\Exception\\Exception"))]
#[derive(Default)]
pub struct LoadException;

#[php_impl]
impl LoadException {
    pub fn __construct() -> Self {
        Self {}
    }
}

#[php_class]
#[php(name = "LoPdf\\Exception\\SaveException")]
#[php(extends(ce = ce::exception, stub = "\\LoPdf\\Exception\\Exception"))]
#[derive(Default)]
pub struct SaveException;

#[php_impl]
impl SaveException {
    pub fn __construct() -> Self {
        Self {}
    }
}
