use std::str::FromStr;
use xsd_macro_utils::{UtilsDefaultSerde, UtilsTupleIo};
use xsd_types::types as xs;
use yaserde_derive::{YaDeserialize, YaSerialize};

pub trait Validate {
    fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
//// This element MUST be conveyed as the root element in any instance document
// based on this Schema expression
pub type Project = ProjectType;
#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct ProjectType {
    #[yaserde(rename = "Users")]
    pub users: Option<UsersType>,

    #[yaserde(rename = "CodeBook")]
    pub code_book: Option<CodeBookType>,

    #[yaserde(rename = "Variables")]
    pub variables: Option<VariablesType>,

    #[yaserde(rename = "Cases")]
    pub cases: Option<CasesType>,

    #[yaserde(rename = "Sources")]
    pub sources: Option<SourcesType>,

    #[yaserde(rename = "Notes")]
    pub notes: Option<NotesType>,

    #[yaserde(rename = "Links")]
    pub links: Option<LinksType>,

    #[yaserde(rename = "Sets")]
    pub sets: Option<SetsType>,

    #[yaserde(rename = "Graphs")]
    pub graphs: Option<GraphsType>,

    #[yaserde(rename = "Description")]
    pub description: Option<String>,

    #[yaserde(rename = "NoteRef")]
    pub note_ref: Vec<NoteRefType>,

    #[yaserde(attribute, rename = "name")]
    pub name: String,

    #[yaserde(attribute, rename = "origin")]
    pub origin: Option<String>,

    #[yaserde(attribute, rename = "creatingUserGUID")]
    pub creating_user_guid: Option<Guidtype>,

    #[yaserde(attribute, rename = "creationDateTime")]
    pub creation_date_time: Option<xs::DateTime>,

    #[yaserde(attribute, rename = "modifyingUserGUID")]
    pub modifying_user_guid: Option<Guidtype>,

    #[yaserde(attribute, rename = "modifiedDateTime")]
    pub modified_date_time: Option<xs::DateTime>,

    #[yaserde(attribute, rename = "basePath")]
    pub base_path: Option<String>,
}

impl Validate for ProjectType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct UsersType {
    #[yaserde(rename = "User")]
    pub user: Vec<UserType>,
}

impl Validate for UsersType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct UserType {
    #[yaserde(attribute, rename = "guid")]
    pub guid: Guidtype,

    #[yaserde(attribute, rename = "name")]
    pub name: Option<String>,

    #[yaserde(attribute, rename = "id")]
    pub id: Option<String>,
}

impl Validate for UserType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct CodeBookType {
    #[yaserde(rename = "Codes")]
    pub codes: CodesType,
}

impl Validate for CodeBookType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct CodesType {
    #[yaserde(rename = "Code")]
    pub code: Vec<CodeType>,
}

impl Validate for CodesType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct CodeType {
    #[yaserde(rename = "Description")]
    pub description: Option<String>,

    #[yaserde(rename = "NoteRef")]
    pub note_ref: Vec<NoteRefType>,

    #[yaserde(rename = "Code")]
    pub code: Vec<CodeType>,

    #[yaserde(attribute, rename = "guid")]
    pub guid: Guidtype,

    #[yaserde(attribute, rename = "name")]
    pub name: String,

    #[yaserde(attribute, rename = "isCodable")]
    pub is_codable: bool,

    #[yaserde(attribute, rename = "color")]
    pub color: Option<Rgbtype>,
}

impl Validate for CodeType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct CasesType {
    #[yaserde(rename = "Case")]
    pub case: Vec<CaseType>,
}

impl Validate for CasesType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct CaseType {
    #[yaserde(rename = "Description")]
    pub description: Option<String>,

    #[yaserde(rename = "CodeRef")]
    pub code_ref: Vec<CodeRefType>,

    #[yaserde(rename = "VariableValue")]
    pub variable_value: Vec<VariableValueType>,

    #[yaserde(rename = "SourceRef")]
    pub source_ref: Vec<SourceRefType>,

    #[yaserde(rename = "SelectionRef")]
    pub selection_ref: Vec<SelectionRefType>,

    #[yaserde(attribute, rename = "guid")]
    pub guid: Guidtype,

    #[yaserde(attribute, rename = "name")]
    pub name: Option<String>,
}

impl Validate for CaseType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct VariablesType {
    #[yaserde(rename = "Variable")]
    pub variable: Vec<VariableType>,
}

impl Validate for VariablesType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct VariableType {
    #[yaserde(rename = "Description")]
    pub description: Option<String>,

    #[yaserde(attribute, rename = "guid")]
    pub guid: Guidtype,

    #[yaserde(attribute, rename = "name")]
    pub name: String,

    #[yaserde(attribute, rename = "typeOfVariable")]
    pub type_of_variable: TypeOfVariableType,
}

impl Validate for VariableType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct VariableValueType {
    #[yaserde(rename = "VariableRef")]
    pub variable_ref: VariableRefType,

    #[yaserde(rename = "VariableValueTypeChoice")]
    pub variable_value_type_choice: variable_value_type::VariableValueTypeChoice,
}

impl Validate for VariableValueType {}

pub mod variable_value_type {
    use super::*;

    #[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
    #[yaserde(namespace = "urn:QDA-XML:project:1.0")]

    pub enum VariableValueTypeChoice {
        TextValue(Option<String>),
        BooleanValue(Option<bool>),
        IntegerValue(Option<xs::Integer>),
        FloatValue(Option<xs::Decimal>),
        DateValue(Option<xs::Date>),
        DateTimeValue(Option<xs::DateTime>),
        __Unknown__(String),
    }

    impl Default for VariableValueTypeChoice {
        fn default() -> VariableValueTypeChoice {
            Self::__Unknown__("No valid variants".into())
        }
    }

    impl Validate for VariableValueTypeChoice {}
}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct SetsType {
    #[yaserde(rename = "Set")]
    pub set: Vec<SetType>,
}

impl Validate for SetsType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct SetType {
    #[yaserde(rename = "Description")]
    pub description: Option<String>,

    #[yaserde(rename = "MemberCode")]
    pub member_code: Vec<CodeRefType>,

    #[yaserde(rename = "MemberSource")]
    pub member_source: Vec<SourceRefType>,

    #[yaserde(rename = "MemberNote")]
    pub member_note: Vec<NoteRefType>,

    #[yaserde(attribute, rename = "guid")]
    pub guid: Guidtype,

    #[yaserde(attribute, rename = "name")]
    pub name: String,
}

impl Validate for SetType {}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]

pub enum SourcesTypeChoice {
    TextSource(TextSourceType),
    PictureSource(PictureSourceType),
    #[yaserde(rename = "PDFSource")]
    Pdfsource(PdfsourceType),
    AudioSource(AudioSourceType),
    VideoSource(VideoSourceType),
    __Unknown__(String),
}

impl Default for SourcesTypeChoice {
    fn default() -> SourcesTypeChoice {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for SourcesTypeChoice {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct SourcesType {
    #[yaserde(flatten)]
    pub sources_type_choice: SourcesTypeChoice,
}

impl Validate for SourcesType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct TextSourceType {
    #[yaserde(rename = "Description")]
    pub description: Option<String>,

    #[yaserde(rename = "PlainTextContent")]
    pub plain_text_content: Option<String>,

    #[yaserde(rename = "PlainTextSelection")]
    pub plain_text_selection: Vec<PlainTextSelectionType>,

    #[yaserde(rename = "Coding")]
    pub coding: Vec<CodingType>,

    #[yaserde(rename = "NoteRef")]
    pub note_ref: Vec<NoteRefType>,

    #[yaserde(rename = "VariableValue")]
    pub variable_value: Vec<VariableValueType>,

    #[yaserde(attribute, rename = "guid")]
    pub guid: Guidtype,

    #[yaserde(attribute, rename = "name")]
    pub name: Option<String>,

    #[yaserde(attribute, rename = "richTextPath")]
    pub rich_text_path: Option<String>,

    #[yaserde(attribute, rename = "plainTextPath")]
    pub plain_text_path: Option<String>,

    #[yaserde(attribute, rename = "creatingUser")]
    pub creating_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "creationDateTime")]
    pub creation_date_time: Option<xs::DateTime>,

    #[yaserde(attribute, rename = "modifyingUser")]
    pub modifying_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "modifiedDateTime")]
    pub modified_date_time: Option<xs::DateTime>,
}

impl Validate for TextSourceType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct PlainTextSelectionType {
    #[yaserde(rename = "Description")]
    pub description: Option<String>,

    #[yaserde(rename = "Coding")]
    pub coding: Vec<CodingType>,

    #[yaserde(rename = "NoteRef")]
    pub note_ref: Vec<NoteRefType>,

    #[yaserde(attribute, rename = "guid")]
    pub guid: Guidtype,

    #[yaserde(attribute, rename = "name")]
    pub name: Option<String>,

    #[yaserde(attribute, rename = "startPosition")]
    pub start_position: xs::Integer,

    #[yaserde(attribute, rename = "endPosition")]
    pub end_position: xs::Integer,

    #[yaserde(attribute, rename = "creatingUser")]
    pub creating_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "creationDateTime")]
    pub creation_date_time: Option<xs::DateTime>,

    #[yaserde(attribute, rename = "modifyingUser")]
    pub modifying_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "modifiedDateTime")]
    pub modified_date_time: Option<xs::DateTime>,
}

impl Validate for PlainTextSelectionType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct PictureSourceType {
    #[yaserde(rename = "Description")]
    pub description: Option<String>,

    #[yaserde(rename = "TextDescription")]
    pub text_description: Option<TextSourceType>,

    #[yaserde(rename = "PictureSelection")]
    pub picture_selection: Vec<PictureSelectionType>,

    #[yaserde(rename = "Coding")]
    pub coding: Vec<CodingType>,

    #[yaserde(rename = "NoteRef")]
    pub note_ref: Vec<NoteRefType>,

    #[yaserde(rename = "VariableValue")]
    pub variable_value: Vec<VariableValueType>,

    #[yaserde(attribute, rename = "guid")]
    pub guid: Guidtype,

    #[yaserde(attribute, rename = "name")]
    pub name: Option<String>,

    #[yaserde(attribute, rename = "path")]
    pub path: Option<String>,

    #[yaserde(attribute, rename = "currentPath")]
    pub current_path: Option<String>,

    #[yaserde(attribute, rename = "creatingUser")]
    pub creating_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "creationDateTime")]
    pub creation_date_time: Option<xs::DateTime>,

    #[yaserde(attribute, rename = "modifyingUser")]
    pub modifying_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "modifiedDateTime")]
    pub modified_date_time: Option<xs::DateTime>,
}

impl Validate for PictureSourceType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct PictureSelectionType {
    #[yaserde(rename = "Description")]
    pub description: Option<String>,

    #[yaserde(rename = "Coding")]
    pub coding: Vec<CodingType>,

    #[yaserde(rename = "NoteRef")]
    pub note_ref: Vec<NoteRefType>,

    #[yaserde(attribute, rename = "guid")]
    pub guid: Guidtype,

    #[yaserde(attribute, rename = "name")]
    pub name: Option<String>,

    #[yaserde(attribute, rename = "firstX")]
    pub first_x: xs::Integer,

    #[yaserde(attribute, rename = "firstY")]
    pub first_y: xs::Integer,

    #[yaserde(attribute, rename = "secondX")]
    pub second_x: xs::Integer,

    #[yaserde(attribute, rename = "secondY")]
    pub second_y: xs::Integer,

    #[yaserde(attribute, rename = "creatingUser")]
    pub creating_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "creationDateTime")]
    pub creation_date_time: Option<xs::DateTime>,

    #[yaserde(attribute, rename = "modifyingUser")]
    pub modifying_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "modifiedDateTime")]
    pub modified_date_time: Option<xs::DateTime>,
}

impl Validate for PictureSelectionType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct PdfsourceType {
    #[yaserde(rename = "Description")]
    pub description: Option<String>,

    #[yaserde(rename = "PDFSelection")]
    pub pdf_selection: Vec<PdfselectionType>,

    #[yaserde(rename = "Representation")]
    pub representation: Option<TextSourceType>,

    #[yaserde(rename = "Coding")]
    pub coding: Vec<CodingType>,

    #[yaserde(rename = "NoteRef")]
    pub note_ref: Vec<NoteRefType>,

    #[yaserde(rename = "VariableValue")]
    pub variable_value: Vec<VariableValueType>,

    #[yaserde(attribute, rename = "guid")]
    pub guid: Guidtype,

    #[yaserde(attribute, rename = "name")]
    pub name: Option<String>,

    #[yaserde(attribute, rename = "path")]
    pub path: Option<String>,

    #[yaserde(attribute, rename = "currentPath")]
    pub current_path: Option<String>,

    #[yaserde(attribute, rename = "creatingUser")]
    pub creating_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "creationDateTime")]
    pub creation_date_time: Option<xs::DateTime>,

    #[yaserde(attribute, rename = "modifyingUser")]
    pub modifying_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "modifiedDateTime")]
    pub modified_date_time: Option<xs::DateTime>,
}

impl Validate for PdfsourceType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct PdfselectionType {
    #[yaserde(rename = "Description")]
    pub description: Option<String>,

    #[yaserde(rename = "Representation")]
    pub representation: Option<TextSourceType>,

    #[yaserde(rename = "Coding")]
    pub coding: Vec<CodingType>,

    #[yaserde(rename = "NoteRef")]
    pub note_ref: Vec<NoteRefType>,

    #[yaserde(attribute, rename = "guid")]
    pub guid: Guidtype,

    #[yaserde(attribute, rename = "name")]
    pub name: Option<String>,

    #[yaserde(attribute, rename = "page")]
    pub page: xs::Integer,

    #[yaserde(attribute, rename = "firstX")]
    pub first_x: xs::Integer,

    #[yaserde(attribute, rename = "firstY")]
    pub first_y: xs::Integer,

    #[yaserde(attribute, rename = "secondX")]
    pub second_x: xs::Integer,

    #[yaserde(attribute, rename = "secondY")]
    pub second_y: xs::Integer,

    #[yaserde(attribute, rename = "creatingUser")]
    pub creating_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "creationDateTime")]
    pub creation_date_time: Option<xs::DateTime>,

    #[yaserde(attribute, rename = "modifyingUser")]
    pub modifying_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "modifiedDateTime")]
    pub modified_date_time: Option<xs::DateTime>,
}

impl Validate for PdfselectionType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct AudioSourceType {
    #[yaserde(rename = "Description")]
    pub description: Option<String>,

    #[yaserde(rename = "Transcript")]
    pub transcript: Vec<TranscriptType>,

    #[yaserde(rename = "AudioSelection")]
    pub audio_selection: Vec<AudioSelectionType>,

    #[yaserde(rename = "Coding")]
    pub coding: Vec<CodingType>,

    #[yaserde(rename = "NoteRef")]
    pub note_ref: Vec<NoteRefType>,

    #[yaserde(rename = "VariableValue")]
    pub variable_value: Vec<VariableValueType>,

    #[yaserde(attribute, rename = "guid")]
    pub guid: Guidtype,

    #[yaserde(attribute, rename = "name")]
    pub name: Option<String>,

    #[yaserde(attribute, rename = "path")]
    pub path: Option<String>,

    #[yaserde(attribute, rename = "currentPath")]
    pub current_path: Option<String>,

    #[yaserde(attribute, rename = "creatingUser")]
    pub creating_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "creationDateTime")]
    pub creation_date_time: Option<xs::DateTime>,

    #[yaserde(attribute, rename = "modifyingUser")]
    pub modifying_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "modifiedDateTime")]
    pub modified_date_time: Option<xs::DateTime>,
}

impl Validate for AudioSourceType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct AudioSelectionType {
    #[yaserde(rename = "Description")]
    pub description: Option<String>,

    #[yaserde(rename = "Coding")]
    pub coding: Vec<CodingType>,

    #[yaserde(rename = "NoteRef")]
    pub note_ref: Vec<NoteRefType>,

    #[yaserde(attribute, rename = "guid")]
    pub guid: Guidtype,

    #[yaserde(attribute, rename = "name")]
    pub name: Option<String>,

    #[yaserde(attribute, rename = "begin")]
    pub begin: xs::Integer,

    #[yaserde(attribute, rename = "end")]
    pub end: xs::Integer,

    #[yaserde(attribute, rename = "creatingUser")]
    pub creating_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "creationDateTime")]
    pub creation_date_time: Option<xs::DateTime>,

    #[yaserde(attribute, rename = "modifyingUser")]
    pub modifying_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "modifiedDateTime")]
    pub modified_date_time: Option<xs::DateTime>,
}

impl Validate for AudioSelectionType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct VideoSourceType {
    #[yaserde(rename = "Description")]
    pub description: Option<String>,

    #[yaserde(rename = "Transcript")]
    pub transcript: Vec<TranscriptType>,

    #[yaserde(rename = "VideoSelection")]
    pub video_selection: Vec<VideoSelectionType>,

    #[yaserde(rename = "Coding")]
    pub coding: Vec<CodingType>,

    #[yaserde(rename = "NoteRef")]
    pub note_ref: Vec<NoteRefType>,

    #[yaserde(rename = "VariableValue")]
    pub variable_value: Vec<VariableValueType>,

    #[yaserde(attribute, rename = "guid")]
    pub guid: Guidtype,

    #[yaserde(attribute, rename = "name")]
    pub name: Option<String>,

    #[yaserde(attribute, rename = "path")]
    pub path: Option<String>,

    #[yaserde(attribute, rename = "currentPath")]
    pub current_path: Option<String>,

    #[yaserde(attribute, rename = "creatingUser")]
    pub creating_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "creationDateTime")]
    pub creation_date_time: Option<xs::DateTime>,

    #[yaserde(attribute, rename = "modifyingUser")]
    pub modifying_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "modifiedDateTime")]
    pub modified_date_time: Option<xs::DateTime>,
}

impl Validate for VideoSourceType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct VideoSelectionType {
    #[yaserde(rename = "Description")]
    pub description: Option<String>,

    #[yaserde(rename = "Coding")]
    pub coding: Vec<CodingType>,

    #[yaserde(rename = "NoteRef")]
    pub note_ref: Vec<NoteRefType>,

    #[yaserde(attribute, rename = "guid")]
    pub guid: Guidtype,

    #[yaserde(attribute, rename = "name")]
    pub name: Option<String>,

    #[yaserde(attribute, rename = "begin")]
    pub begin: xs::Integer,

    #[yaserde(attribute, rename = "end")]
    pub end: xs::Integer,

    #[yaserde(attribute, rename = "creatingUser")]
    pub creating_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "creationDateTime")]
    pub creation_date_time: Option<xs::DateTime>,

    #[yaserde(attribute, rename = "modifyingUser")]
    pub modifying_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "modifiedDateTime")]
    pub modified_date_time: Option<xs::DateTime>,
}

impl Validate for VideoSelectionType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct TranscriptType {
    #[yaserde(rename = "Description")]
    pub description: Option<String>,

    #[yaserde(rename = "PlainTextContent")]
    pub plain_text_content: Option<String>,

    #[yaserde(rename = "SyncPoint")]
    pub sync_point: Vec<SyncPointType>,

    #[yaserde(rename = "TranscriptSelection")]
    pub transcript_selection: Vec<TranscriptSelectionType>,

    #[yaserde(rename = "NoteRef")]
    pub note_ref: Vec<NoteRefType>,

    #[yaserde(attribute, rename = "guid")]
    pub guid: Guidtype,

    #[yaserde(attribute, rename = "name")]
    pub name: Option<String>,

    #[yaserde(attribute, rename = "richTextPath")]
    pub rich_text_path: Option<String>,

    #[yaserde(attribute, rename = "plainTextPath")]
    pub plain_text_path: Option<String>,

    #[yaserde(attribute, rename = "creatingUser")]
    pub creating_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "creationDateTime")]
    pub creation_date_time: Option<xs::DateTime>,

    #[yaserde(attribute, rename = "modifyingUser")]
    pub modifying_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "modifiedDateTime")]
    pub modified_date_time: Option<xs::DateTime>,
}

impl Validate for TranscriptType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct TranscriptSelectionType {
    #[yaserde(rename = "Description")]
    pub description: Option<String>,

    #[yaserde(rename = "Coding")]
    pub coding: Vec<CodingType>,

    #[yaserde(rename = "NoteRef")]
    pub note_ref: Vec<NoteRefType>,

    #[yaserde(attribute, rename = "guid")]
    pub guid: Guidtype,

    #[yaserde(attribute, rename = "name")]
    pub name: Option<String>,

    #[yaserde(attribute, rename = "fromSyncPoint")]
    pub from_sync_point: Option<Guidtype>,

    #[yaserde(attribute, rename = "toSyncPoint")]
    pub to_sync_point: Option<Guidtype>,

    #[yaserde(attribute, rename = "creatingUser")]
    pub creating_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "creationDateTime")]
    pub creation_date_time: Option<xs::DateTime>,

    #[yaserde(attribute, rename = "modifyingUser")]
    pub modifying_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "modifiedDateTime")]
    pub modified_date_time: Option<xs::DateTime>,
}

impl Validate for TranscriptSelectionType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct SyncPointType {
    #[yaserde(attribute, rename = "guid")]
    pub guid: Guidtype,

    #[yaserde(attribute, rename = "timeStamp")]
    pub time_stamp: Option<xs::Integer>,

    #[yaserde(attribute, rename = "position")]
    pub position: Option<xs::Integer>,
}

impl Validate for SyncPointType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct CodingType {
    #[yaserde(rename = "CodeRef")]
    pub code_ref: CodeRefType,

    #[yaserde(rename = "NoteRef")]
    pub note_ref: Vec<NoteRefType>,

    #[yaserde(attribute, rename = "guid")]
    pub guid: Guidtype,

    #[yaserde(attribute, rename = "creatingUser")]
    pub creating_user: Option<Guidtype>,

    #[yaserde(attribute, rename = "creationDateTime")]
    pub creation_date_time: Option<xs::DateTime>,
}

impl Validate for CodingType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct GraphsType {
    #[yaserde(rename = "Graph")]
    pub graph: Vec<GraphType>,
}

impl Validate for GraphsType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct GraphType {
    #[yaserde(rename = "Vertex")]
    pub vertex: Vec<VertexType>,

    #[yaserde(rename = "Edge")]
    pub edge: Vec<EdgeType>,

    #[yaserde(attribute, rename = "guid")]
    pub guid: Guidtype,

    #[yaserde(attribute, rename = "name")]
    pub name: Option<String>,
}

impl Validate for GraphType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct VertexType {
    #[yaserde(attribute, rename = "guid")]
    pub guid: Guidtype,

    #[yaserde(attribute, rename = "representedGUID")]
    pub represented_guid: Option<Guidtype>,

    #[yaserde(attribute, rename = "name")]
    pub name: Option<String>,

    #[yaserde(attribute, rename = "firstX")]
    pub first_x: xs::Integer,

    #[yaserde(attribute, rename = "firstY")]
    pub first_y: xs::Integer,

    #[yaserde(attribute, rename = "secondX")]
    pub second_x: Option<xs::Integer>,

    #[yaserde(attribute, rename = "secondY")]
    pub second_y: Option<xs::Integer>,

    #[yaserde(attribute, rename = "shape")]
    pub shape: Option<ShapeType>,

    #[yaserde(attribute, rename = "color")]
    pub color: Option<Rgbtype>,
}

impl Validate for VertexType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct EdgeType {
    #[yaserde(attribute, rename = "guid")]
    pub guid: Guidtype,

    #[yaserde(attribute, rename = "representedGUID")]
    pub represented_guid: Option<Guidtype>,

    #[yaserde(attribute, rename = "name")]
    pub name: Option<String>,

    #[yaserde(attribute, rename = "sourceVertex")]
    pub source_vertex: Guidtype,

    #[yaserde(attribute, rename = "targetVertex")]
    pub target_vertex: Guidtype,

    #[yaserde(attribute, rename = "color")]
    pub color: Option<Rgbtype>,

    #[yaserde(attribute, rename = "direction")]
    pub direction: Option<DirectionType>,

    #[yaserde(attribute, rename = "lineStyle")]
    pub line_style: Option<LineStyleType>,
}

impl Validate for EdgeType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct NotesType {
    #[yaserde(rename = "Note")]
    pub note: Vec<TextSourceType>,
}

impl Validate for NotesType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct LinksType {
    #[yaserde(rename = "Link")]
    pub link: Vec<LinkType>,
}

impl Validate for LinksType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct LinkType {
    #[yaserde(rename = "NoteRef")]
    pub note_ref: Vec<NoteRefType>,

    #[yaserde(attribute, rename = "guid")]
    pub guid: Guidtype,

    #[yaserde(attribute, rename = "name")]
    pub name: Option<String>,

    #[yaserde(attribute, rename = "direction")]
    pub direction: Option<DirectionType>,

    #[yaserde(attribute, rename = "color")]
    pub color: Option<Rgbtype>,

    #[yaserde(attribute, rename = "originGUID")]
    pub origin_guid: Option<Guidtype>,

    #[yaserde(attribute, rename = "targetGUID")]
    pub target_guid: Option<Guidtype>,
}

impl Validate for LinkType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct NoteRefType {
    #[yaserde(attribute, rename = "targetGUID")]
    pub target_guid: Guidtype,
}

impl Validate for NoteRefType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct CodeRefType {
    #[yaserde(attribute, rename = "targetGUID")]
    pub target_guid: Guidtype,
}

impl Validate for CodeRefType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct SourceRefType {
    #[yaserde(attribute, rename = "targetGUID")]
    pub target_guid: Guidtype,
}

impl Validate for SourceRefType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct SelectionRefType {
    #[yaserde(attribute, rename = "targetGUID")]
    pub target_guid: Guidtype,
}

impl Validate for SelectionRefType {}

#[derive(Default, PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]
pub struct VariableRefType {
    #[yaserde(attribute, rename = "targetGUID")]
    pub target_guid: Guidtype,
}

impl Validate for VariableRefType {}

#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Guidtype(pub String);

impl Validate for Guidtype {}
#[derive(Default, PartialEq, Debug, UtilsTupleIo, UtilsDefaultSerde)]
pub struct Rgbtype(pub String);

impl Validate for Rgbtype {}
#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]

pub enum DirectionType {
    Associative,
    OneWay,
    Bidirectional,
    __Unknown__(String),
}

impl Default for DirectionType {
    fn default() -> DirectionType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for DirectionType {}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]

pub enum TypeOfVariableType {
    Text,
    Boolean,
    Integer,
    Float,
    Date,
    DateTime,
    __Unknown__(String),
}

impl Default for TypeOfVariableType {
    fn default() -> TypeOfVariableType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for TypeOfVariableType {}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]

pub enum ShapeType {
    Person,
    Oval,
    Rectangle,
    RoundedRectangle,
    Star,
    LeftTriangle,
    RightTriangle,
    UpTriangle,
    DownTriangle,
    Note,
    __Unknown__(String),
}

impl Default for ShapeType {
    fn default() -> ShapeType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for ShapeType {}

#[derive(PartialEq, Debug, YaSerialize, YaDeserialize)]
#[yaserde(namespace = "urn:QDA-XML:project:1.0")]

pub enum LineStyleType {
    #[yaserde(rename = "dotted")]
    Dotted,
    #[yaserde(rename = "dashed")]
    Dashed,
    #[yaserde(rename = "solid")]
    Solid,
    __Unknown__(String),
}

impl Default for LineStyleType {
    fn default() -> LineStyleType {
        Self::__Unknown__("No valid variants".into())
    }
}

impl Validate for LineStyleType {}
