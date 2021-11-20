use quick_xml::{events::Event, Reader};
use std::{
    fs::File,
    io::{self, BufRead as _, BufReader, Read, Seek},
    path::{Path, PathBuf},
};
use thiserror::Error;
use zip::ZipArchive;

mod xml;

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub struct Qdpx<R> {
    archive: ZipArchive<R>,
}

impl Qdpx<File> {
    pub fn open(filename: impl AsRef<Path>) -> Result<Self> {
        let filename = filename.as_ref();
        let file = File::open(filename).map_err(|e| Error::OpeningFile {
            filename: filename.to_owned(),
            source: e,
        })?;
        let mut archive = ZipArchive::new(file)?;
        let project = BufReader::new(archive.by_name("project.qde")?);
        let project = xml::Project::from_xml(Reader::from_reader(project))?;
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
    #[error("qdpx file corrupt")]
    Uuid(
        #[from]
        #[source]
        uuid::Error,
    ),
    #[error("qdpx file corrupt")]
    DateTime(
        #[from]
        #[source]
        chrono::format::ParseError,
    ),
    // this is errors I'm too lazy to manage properly. it should disappear eventually
    #[error("uncategorised")]
    Uncategorised(
        #[from]
        #[source]
        anyhow::Error,
    ),
    #[error("encoding error")]
    Utf8(
        #[from]
        #[source]
        std::str::Utf8Error,
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
