use quick_xml::{events::Event, Reader};
use std::{
    fs::File,
    io::{self, BufReader, Read, Seek},
    path::{Path, PathBuf},
};
use thiserror::Error;
use zip::ZipArchive;

pub struct Qdpx<R> {
    archive: ZipArchive<R>,
}

impl Qdpx<File> {
    pub fn open(filename: impl AsRef<Path>) -> Result<Self, Error> {
        let filename = filename.as_ref();
        let file = File::open(filename).map_err(|e| Error::OpeningFile {
            filename: filename.to_owned(),
            source: e,
        })?;
        let mut archive = ZipArchive::new(file)?;
        let project = archive.by_name("project.qde")?;
        let mut xml = Reader::from_reader(BufReader::new(project));
        let mut xml_buf = Vec::new();
        loop {
            let event = xml.read_event(&mut xml_buf)?;
            if matches!(event, Event::Eof) {
                break;
            }
            println!("{:?}", event);
        }
        drop(xml);
        Ok(Qdpx { archive })
    }
}

impl<R: Read + Seek> Qdpx<R> {}

#[derive(Error, Debug)]
pub enum Error {
    #[error("cannot open qdpx file \"{}\"", Path::display(.filename))]
    OpeningFile {
        filename: PathBuf,
        source: io::Error,
    },
    #[error("qdpx file corrupt")]
    Zip(
        #[from]
        #[source]
        zip::result::ZipError,
    ),
    #[error("qdpx file corrupt")]
    Xml(
        #[from]
        #[source]
        quick_xml::Error,
    ),
    #[error("integer conversion failed")]
    IntegerConversion,
    #[error("generic i/o error")]
    Io(
        #[from]
        #[source]
        io::Error,
    ),
}
