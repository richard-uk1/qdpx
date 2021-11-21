//! Code for (de)serializing a qdpx project (as specified in project.xsd).

use crate::{Error, Result};
use anyhow::{anyhow, bail, ensure, Context};
use chrono::{DateTime, FixedOffset};
use quick_xml::de::from_reader;
use serde::{Deserialize, Serialize};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{fmt, io, mem, str};
use uuid::Uuid;

// NOTE: the serialize impl will not match the correct format - xml deserialization is lossy. A
// custom serializer will be written using quick-xml. Only use the serde serializer for internal
// storage, or when exporting data outside the qdpx format.
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    pub origin: Option<String>,
    #[serde(rename = "creatingUserGUID")]
    pub creating_user: Option<Uuid>,
    #[serde(rename = "creationDateTime")]
    pub creation_datetime: Option<DateTime<FixedOffset>>,
    #[serde(rename = "modifyingUserGUID")]
    pub modifying_user: Option<Uuid>,
    #[serde(rename = "modifiedDateTime")]
    pub modified_datetime: Option<DateTime<FixedOffset>>,
    #[serde(rename = "basePath")]
    pub base_path: Option<String>,
    #[serde(rename = "Users")]
    pub users: Option<Users>,
    #[serde(rename = "CodeBook")]
    pub codebook: Option<CodeBook>,
    pub variables: Option<Variables>,
    pub cases: Option<Cases>,
    #[serde(rename = "Sources")]
    pub sources: Option<Sources>,
    pub notes: Option<Notes>,
    pub links: Option<Links>,
    pub sets: Option<Sets>,
    pub graphs: Option<Graphs>,
    pub description: Option<String>,
    #[serde(rename = "NoteRef", default)]
    pub note_ref: Vec<NoteRef>,
}

impl Project {
    /// Decode a project from a reader.
    pub fn from_reader<B: io::BufRead>(input: B) -> Result<Self> {
        from_reader(input)
            .context("parsing project.qde")
            .map_err(Error::ReadingFile)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Users {
    #[serde(rename = "User")]
    users: Vec<User>,
}

impl fmt::Debug for Users {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_list().entries(self.users.iter()).finish()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub guid: Uuid,
    pub name: Option<String>,
    pub id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeBook {
    #[serde(rename = "Codes")]
    pub code_sets: Vec<Codes>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Codes {
    #[serde(rename = "Code")]
    pub codes: Vec<Code>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Code {
    pub guid: Uuid,
    pub name: String,
    #[serde(rename = "isCodable")]
    pub is_codable: bool,
    pub color: Option<Color>,
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(default, rename = "NoteRef")]
    pub note_refs: Vec<String>,
    #[serde(default, rename = "Code")]
    pub children: Vec<Code>,
}

// TODO
type Variables = ();
type Cases = ();

// Note that strictly speaking there should be at least one source inside a `Sources`, but we don't
// check this for now.
#[derive(Debug, Serialize, Deserialize)]
pub struct Sources {
    #[serde(default, rename = "TextSource")]
    pub text: Vec<TextSource>,
    #[serde(default, rename = "PictureSource")]
    pub picture: Vec<PictureSource>,
    #[serde(default, rename = "PDFSource")]
    pub pdf: Vec<PDFSource>,
    #[serde(default, rename = "AudioSource")]
    pub audio: Vec<AudioSource>,
    #[serde(default, rename = "VideoSource")]
    pub video: Vec<VideoSource>,
}

impl Sources {
    pub fn validate(&self) -> Result {
        if self.text.is_empty()
            && self.picture.is_empty()
            && self.pdf.is_empty()
            && self.audio.is_empty()
            && self.video.is_empty()
        {
            Err(anyhow!("sources must contain at least 1 source").into())
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextSource {
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "PlainTextContent")]
    pub plain_text_content: Option<String>,
    #[serde(default, rename = "PlainTextSelection")]
    pub plain_text_selection: Vec<PlainTextSelection>,
    #[serde(default, rename = "Coding")]
    pub coding: Vec<Coding>,
    #[serde(default, rename = "NoteRef")]
    pub note_ref: Vec<NoteRef>,
    #[serde(default, rename = "VariableValue")]
    pub variable_value: Vec<VariableValue>,
    pub guid: Uuid,
    pub name: Option<String>,
    #[serde(rename = "richTextPath")]
    pub rich_text_path: Option<String>,
    #[serde(rename = "plainTextPath")]
    pub plain_text_path: Option<String>,
    #[serde(rename = "creatingUser")]
    pub creating_user: Option<Uuid>,
    #[serde(rename = "creationDateTime")]
    pub creation_datetime: Option<DateTime<FixedOffset>>,
    #[serde(rename = "modifyingUser")]
    pub modifying_user: Option<Uuid>,
    #[serde(rename = "modifiedDateTime")]
    pub modified_datetime: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlainTextSelection {
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(default, rename = "Coding")]
    pub codings: Vec<Coding>,
    #[serde(default, rename = "NoteRef")]
    pub note_refs: Vec<NoteRef>,
    pub guid: Uuid,
    pub name: Option<String>,
    #[serde(rename = "startPosition")]
    pub start_position: u64,
    #[serde(rename = "endPosition")]
    pub end_position: u64,
    #[serde(rename = "creatingUser")]
    pub creating_user: Option<Uuid>,
    #[serde(rename = "creationDateTime")]
    pub creation_datetime: Option<DateTime<FixedOffset>>,
    #[serde(rename = "modifyingUser")]
    pub modifying_user: Option<Uuid>,
    #[serde(rename = "modifiedDateTime")]
    pub modified_datetime: Option<DateTime<FixedOffset>>,
}

type PictureSource = ();
type PDFSource = ();
type AudioSource = ();
type VideoSource = ();

type Notes = ();
type Links = ();
type Sets = ();
type Graphs = ();

#[derive(Debug, Serialize, Deserialize)]
pub struct Coding {
    #[serde(rename = "CodeRef")]
    pub code_ref: CodeRef,
    #[serde(default, rename = "NoteRef")]
    pub note_refs: Vec<NoteRef>,
}

type VariableValue = ();

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteRef {
    #[serde(rename = "targetGUID")]
    pub target: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CodeRef {
    #[serde(rename = "targetGUID")]
    pub target: Uuid,
}

#[derive(Debug, DeserializeFromStr, SerializeDisplay)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl str::FromStr for Color {
    type Err = anyhow::Error;
    // TODO unchecked
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        #[inline]
        fn hex2u8(input: u8) -> Option<u8> {
            match input {
                v @ b'0'..=b'9' => Some(v - b'0'),
                v @ b'a'..=b'f' => Some(v - b'a' + 10),
                v @ b'A'..=b'F' => Some(v - b'A' + 10),
                _ => None,
            }
        }

        #[inline]
        fn parse_short(input: [u8; 3]) -> Option<Color> {
            let r = hex2u8(input[0])? << 4;
            let g = hex2u8(input[1])? << 4;
            let b = hex2u8(input[2])? << 4;
            Some(Color { r, g, b })
        }

        #[inline]
        fn parse_long(input: [u8; 6]) -> Option<Color> {
            let r = hex2u8(input[0])? << 4 + hex2u8(input[1])?;
            let g = hex2u8(input[2])? << 4 + hex2u8(input[3])?;
            let b = hex2u8(input[4])? << 4 + hex2u8(input[5])?;
            Some(Color { r, g, b })
        }

        let i = input.as_bytes();
        ensure!(
            matches!(i.get(0), Some(v) if *v == b'#'),
            "invalid color \"{}\"",
            input
        );
        if let Ok(long) = <[u8; 6]>::try_from(&i[1..]) {
            parse_long(long).ok_or(anyhow!("invalid color \"{}\"", input))
        } else if let Ok(short) = <[u8; 3]>::try_from(&i[1..]) {
            parse_short(short).ok_or(anyhow!("invalid color \"{}\"", input))
        } else {
            bail!("invalid color \"{}\"", input);
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO unchecked
        write!(f, "#{:x}{:x}{:x}", self.r, self.g, self.b)
    }
}

// A visitor trait to make finding all things of a certain type easier

/// Whether to keep traversing in the visitor
#[derive(Debug, Copy, Clone)]
pub enum Traversal {
    /// Return after this traversal
    Stop,
    /// Keep on traversing
    Continue,
}

pub trait ProjectVisitor {
    fn visit_code(&self, f: impl FnMut(&Code) -> Traversal) {}
}

fn visit_code_inner(code: &Code, mut f: &mut impl FnMut(&Code) -> Traversal) -> Traversal {
    if matches!(f(code), Traversal::Stop) {
        return Traversal::Stop;
    }
    for child in &code.children {
        if matches!(visit_code_inner(child, f), Traversal::Stop) {
            return Traversal::Stop;
        }
    }
    Traversal::Continue
}

impl ProjectVisitor for Project {
    fn visit_code(&self, mut f: impl FnMut(&Code) -> Traversal) {
        if let Some(codebook) = &self.codebook {
            for code_set in codebook.code_sets.iter() {
                for code in code_set.codes.iter() {
                    if matches!(visit_code_inner(code, &mut f), Traversal::Stop) {
                        return;
                    }
                }
            }
        }
    }
}
