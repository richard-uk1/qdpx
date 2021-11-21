use quick_xml::{events::Event, Reader};
use std::{
    fs::{self, File},
    io::{self, BufRead as _, BufReader, Read, Seek},
    path::{Path, PathBuf},
};
use thiserror::Error;
use zip::ZipArchive;

mod project;
pub use project::{Project, ProjectVisitor, Traversal};

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct Qdpx<R> {
    archive: ZipArchive<R>,
    project: Project,
}

impl Qdpx<io::Cursor<Vec<u8>>> {
    /// Open a `qdpx` file by loading the file into memory.
    ///
    /// This means fewer errors and fast UX, but can fail if the user has a very large project and
    /// small memory.
    pub fn load_into_memory(filename: impl AsRef<Path>) -> Result<Self> {
        let filename = filename.as_ref();
        // Load file into memory.
        let file = fs::read(filename).map_err(|e| Error::OpeningFile {
            filename: filename.to_owned(),
            source: e,
        })?;
        Qdpx::from_reader(io::Cursor::new(file))
    }
}

impl<R: io::Read + io::Seek> Qdpx<R> {
    pub fn from_reader(reader: R) -> Result<Self> {
        let mut archive = ZipArchive::new(reader)?;
        let project = BufReader::new(archive.by_name("project.qde")?);
        let project = Project::from_reader(project)?;
        Ok(Qdpx { archive, project })
    }
    pub fn project(&self) -> &Project {
        &self.project
    }
}

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
    #[error("qdpx file corrupt")]
    ReadingFile(
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
