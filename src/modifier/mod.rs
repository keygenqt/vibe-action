//! Modifier system for transforming tag values via pipe syntax.
//! Each modifier implements the Modifier trait and is registered in ModifierRegistry.

pub mod ast;
pub mod join;
pub mod lower;
pub mod modifier;
pub mod reverse;
pub mod sort;
pub mod split;
pub mod take;
pub mod trim;
pub mod upper;
