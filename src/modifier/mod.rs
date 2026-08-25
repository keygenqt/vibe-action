//! Modifier system for transforming tag values via pipe syntax.
//! Each modifier implements the Modifier trait and is registered in ModifierRegistry.

pub mod ast;
pub mod clipboard;
pub mod contains;
pub mod empty;
pub mod equals;
pub mod format;
pub mod is_dir;
pub mod is_file;
pub mod join;
pub mod load;
pub mod lower;
pub mod modifier;
pub mod resolve;
pub mod reverse;
pub mod scan;
pub mod size;
pub mod sort;
pub mod split;
pub mod strip;
pub mod take;
pub mod text;
pub mod trim;
pub mod uniq;
pub mod upper;
