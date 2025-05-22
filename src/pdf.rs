use crate::exception::LoadException;
use crate::php_stream::PhpStream;
use ext_php_rs::convert::IntoZval;
use ext_php_rs::exception::{PhpException, PhpResult};
use ext_php_rs::{php_class, php_impl};

#[php_class]
#[php(name = "LoPdf\\Document")]
#[derive(Default)]
pub struct Document {
    doc: lopdf::Document,
}

#[php_impl]
impl Document {
    /// Opens a PDF file
    pub fn open(path: String) -> PhpResult<Self> {
        let stream = match PhpStream::open(&path, "rb") {
            Ok(s) => s,
            Err(e) => {
                return Err(PhpException::from_class::<LoadException>(format!(
                    "Unable to open file: {}",
                    e
                ))
                .with_object(LoadException.into_zval(false)?));
            }
        };

        match lopdf::Document::load_from(stream) {
            Ok(doc) => Ok(Self { doc }),
            Err(e) => Err(PhpException::from_class::<LoadException>(format!(
                "Unable to load file: {}",
                e
            ))
            .with_object(LoadException.into_zval(false)?)),
        }
    }

    /// Opens a PDF file
    pub fn load(contents: String) -> PhpResult<Self> {
        match lopdf::Document::load_mem(contents.as_bytes()) {
            Ok(doc) => Ok(Self { doc }),
            Err(e) => Err(PhpException::from_class::<LoadException>(format!(
                "Unable to load file: {}",
                e
            ))
            .with_object(LoadException.into_zval(false)?)),
        }
    }

    pub fn compress(&mut self) {
        self.doc.compress();
    }

    pub fn decompress(&mut self) {
        self.doc.decompress();
    }

    pub fn is_encrypted(&self) -> bool {
        self.doc.is_encrypted()
    }

    pub fn count_pages(&self) -> usize {
        self.doc.page_iter().count()
    }
}
