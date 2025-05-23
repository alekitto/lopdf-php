use crate::exception::{LoadException, SaveException};
use crate::php_stream::PhpStream;
use ext_php_rs::binary::Binary;
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
    /// Opens a PDF file.
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

    /// Loads a PDF file from filename.
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

    #[php(defaults(path = None))]
    pub fn save(&mut self, path: Option<String>) -> PhpResult<Option<Binary<u8>>> {
        if let Some(path) = path {
            self.save_to_path(path)
        } else {
            let mut vec = vec![];
            if let Err(e) = self.doc.save_to(&mut vec) {
                return Err(PhpException::from_class::<SaveException>(format!(
                    "Unable to save: {}",
                    e,
                ))
                .with_object(SaveException.into_zval(false)?));
            }

            Ok(Some(vec.into()))
        }
    }

    /// Compress PDF file.
    pub fn compress(&mut self) {
        self.doc.compress();
    }

    /// Decompress PDF file.
    pub fn decompress(&mut self) {
        self.doc.decompress();
    }

    /// Whether the PDF file is encrypted.
    pub fn is_encrypted(&self) -> bool {
        self.doc.is_encrypted()
    }

    /// Returns the pages count.
    pub fn count_pages(&self) -> usize {
        self.doc.page_iter().count()
    }
}

impl Document {
    fn save_to_path(&mut self, path: String) -> PhpResult<Option<Binary<u8>>> {
        let mut stream = match PhpStream::open(&path, "wb") {
            Ok(s) => s,
            Err(e) => {
                return Err(PhpException::from_class::<SaveException>(format!(
                    "Unable to open file: {}",
                    e
                ))
                .with_object(SaveException.into_zval(false)?));
            }
        };

        match self.doc.save_to(&mut stream) {
            Ok(()) => Ok(None),
            Err(e) => Err(PhpException::from_class::<SaveException>(format!(
                "Unable to save file: {}",
                e
            ))
            .with_object(SaveException.into_zval(false)?)),
        }
    }
}
