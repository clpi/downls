use std::vec;

use tracing::Value;

pub mod action;
pub mod completion;
pub mod document;
pub mod hint;
pub mod lens;
pub mod workspace;

pub use self::{
    action::{code_action, code_action_resolve},
    completion::{completion, completion_resolve},
    document::{
        did_change, did_close, did_open, did_save,
        format::{format, format_on_type, format_range},
        will_save,
    },
    hint::{inlay_hint, inlay_hint_resolve},
    lens::{code_lens, code_lens_resolve},
    workspace::{configure, execute_command},
};

pub type Result<T> = anyhow::Result<T>;

pub trait Handle {}
