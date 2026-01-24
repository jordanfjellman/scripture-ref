mod bvc;
mod lexer;
mod parser;
mod scripture_ref_builder;

pub use scripture_ref_builder::ScriptureRef;

pub struct ScriptureReferenceBuilder;

impl ScriptureReferenceBuilder {}

pub struct ScriptureReferenceSeeker;

impl ScriptureReferenceSeeker {}

pub struct ScriptureReferenceSorter;

impl ScriptureReferenceSorter {}

// TODO: isn't this the same as just creating a scripture reference?
// My thought is that we may not want to expose the "ScriptureReference" type
pub struct ScriptureReferenceValidator;

impl ScriptureReferenceValidator {}
