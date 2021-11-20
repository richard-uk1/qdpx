//! Code for (de)serializing a qdpx project (as specified in project.xsd).

use crate::{Error, Result};
use anyhow::anyhow;
use chrono::{DateTime, FixedOffset};
use quick_xml::{
    events::{attributes::Attribute, Event},
    Reader,
};
use std::{io, mem, str};
use uuid::Uuid;

#[derive(Debug)]
pub struct Project {
    name: String,
    origin: Option<String>,
    creating_user: Option<Uuid>,
    creation_time: Option<DateTime<FixedOffset>>,
    modifying_user: Option<Uuid>,
    modified_time: Option<DateTime<FixedOffset>>,
    base_path: Option<String>,
    users: Option<Users>,
    codebook: Option<Codebook>,
    variables: Option<Variables>,
    cases: Option<Cases>,
    sources: Option<Sources>,
    notes: Option<Notes>,
    links: Option<Links>,
    sets: Option<Sets>,
    graphs: Option<Graphs>,
    description: Option<String>,
    note_ref: Vec<NoteRef>,
}

impl Default for Project {
    fn default() -> Self {
        Project {
            name: String::new(),
            origin: None,
            creating_user: None,
            creation_time: None,
            modifying_user: None,
            modified_time: None,
            base_path: None,
            users: None,
            codebook: None,
            variables: None,
            cases: None,
            sources: None,
            notes: None,
            links: None,
            sets: None,
            graphs: None,
            description: None,
            note_ref: vec![],
        }
    }
}

impl Project {
    pub fn from_xml<B: io::BufRead>(mut reader: Reader<B>) -> Result<Self> {
        let mut xml_buf = Vec::new();

        reader.trim_text(true);
        let mut project = Project::default();
        let mut name = None;
        parse_start(&mut reader, &mut xml_buf, b"Project", |attr, reader| {
            match attr.key {
                b"name" => name = Some(attr.unescape_and_decode_value(reader)?),
                b"origin" => project.origin = Some(attr.unescape_and_decode_value(reader)?),
                b"creatingUserGUID" => {
                    project.creating_user = Some(Uuid::parse_str(reader.decode(&attr.value)?)?)
                }
                b"creationDateTime" => {
                    project.creation_time =
                        Some(DateTime::parse_from_rfc3339(reader.decode(&attr.value)?)?)
                }
                b"modifyingUserGUID" => {
                    project.modifying_user = Some(Uuid::parse_str(reader.decode(&attr.value)?)?)
                }
                b"modifiedDateTime" => {
                    project.modified_time =
                        Some(DateTime::parse_from_rfc3339(reader.decode(&attr.value)?)?)
                }
                b"basePath" => project.base_path = Some(attr.unescape_and_decode_value(reader)?),
                b"xmlns" | b"xmlns:xsd" | b"xmlns:xsi" => (), // ignore these
                _ => log::warn!("unexpected attribute {}", String::from_utf8_lossy(attr.key)),
            }
            Ok(())
        })?;
        xml_buf.clear();

        // name is required, so error on the case it was not present.
        match name {
            Some(name) => project.name = name,
            None => return Err(Error::from(anyhow!("project missing name"))),
        }

        // we use Option for element, so we can take it if we use it (and get another for the next
        // try), or leave it if we don't use it (for reuse).
        let mut event = reader.read_event(&mut xml_buf)?;
        match event {
            Event::Start(ref bytes) if bytes.name() == b"Users" => {
                drop(event);
                xml_buf.clear();
                project.users = Some(parse_users(&mut reader, &mut xml_buf)?);
                event = reader.read_event(&mut xml_buf)?;
            }
            _ => (),
        }
        println!("{:#?}", project);
        loop {
            let event = reader.read_event(&mut xml_buf)?;
            if matches!(event, Event::Eof) {
                break;
            }
            //println!("{:?}", event);
            xml_buf.clear();
        }
        drop(reader);
        todo!()
    }
}

/// Parse a sequence of users
fn parse_users<B: io::BufRead>(reader: &mut Reader<B>, buf: &mut Vec<u8>) -> Result<Users> {
    let mut users = Users(vec![]);
    buf.clear();
    Ok(users)
}

/// expect and parse a start event
fn parse_start<B: io::BufRead>(
    reader: &mut Reader<B>,
    buf: &mut Vec<u8>,
    name: &[u8],
    mut attrs: impl FnMut(Attribute, &mut Reader<B>) -> Result<()>,
) -> Result<()> {
    loop {
        let event = reader.read_event(buf)?;
        match event {
            Event::Start(bytes) if bytes.name() == name => {
                for attr in bytes.attributes() {
                    attrs(attr?, reader)?;
                }
                bulf.clear();
                return Ok(());
            }
            Event::Start(_) | Event::End(_) | Event::Text(_) | Event::Empty(_) | Event::Eof => {
                return Err(anyhow!("expected element <{}>", String::from_utf8_lossy(name)).into())
            }
            _ => (), // ignore other events
        }
    }
}

#[derive(Debug)]
struct Users(Vec<User>);

#[derive(Debug)]
struct User {
    guid: Uuid,
    name: Option<String>,
    id: Option<String>,
}

type Codebook = ();
type Variables = ();
type Cases = ();
type Sources = ();
type Notes = ();
type Links = ();
type Sets = ();
type Graphs = ();
type NoteRef = ();
