//! This module contains the implementation of reactive virtual dom concept.

pub mod vnode;
pub mod vtag;
pub mod vtext;

use std::fmt;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use stdweb::web::{Element, EventListenerHandle};

pub use self::vnode::VNode;
pub use self::vtag::VTag;
pub use self::vtext::VText;

/// `Listener` trait is an universal implementation of an event listener
/// which helps to bind Rust-listener to JS-listener (DOM).
pub trait Listener<MSG> {
    /// Returns standard name of DOM's event.
    fn kind(&self) -> &'static str;
    /// Attaches listener to the element and uses messages pool to send
    /// prepaired event back to the yew main loop.
    fn attach(&mut self, element: &Element, messages: Messages<MSG>) -> EventListenerHandle;
}

impl<MSG> fmt::Debug for Listener<MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Listener {{ kind: {} }}", self.kind())
    }
}

/// A reference to a messages pool.
pub type Messages<MSG> = Rc<RefCell<Vec<MSG>>>;

/// A list of event listeners.
type Listeners<MSG> = Vec<Box<Listener<MSG>>>;

/// A map of attributes.
type Attributes = HashMap<String, String>;

/// A set of classes.
type Classes = HashSet<String>;

/// Patch for DOM node modification.
enum Patch<ID, T> {
    Add(ID, T),
    Replace(ID, T),
    Remove(ID),
}

