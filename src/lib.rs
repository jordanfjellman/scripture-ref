mod bvc;
mod parse;
mod position;
mod refs;

// Make testing module available for all test builds (lib and bin tests)
#[cfg(test)]
pub mod testing;

use crate::refs::ScriptureRef;

// #[macro_export]
// macro_rules! scripture_ref {
//     ($string:expr) => {
//         ScriptureRef::new($string)
//     };
// }

pub fn validate_scripture_ref(str: &str) -> Result<(), String> {
    ScriptureRef::new(str).map(|_| ())
}

// pub struct ScriptureReferenceSeeker;
//
// impl ScriptureReferenceSeeker {}
//
// pub struct ScriptureReferenceSorter;
//
// impl ScriptureReferenceSorter {}

// TODO: isn't this the same as just creating a scripture reference?
// My thought is that we may not want to expose the "ScriptureReference" type
// pub struct ScriptureReferenceValidator;
//
// impl ScriptureReferenceValidator {}
