//! The Yew Prelude
//!
//! The purpose of this module is to alleviate imports of many common types:
//!
//! ```
//! # #![allow(unused_imports)]
//! use yew::prelude::*;
//! ```
extern crate stdweb;

pub use crate::html::{
    Component,
    ComponentLink,
    Href,
    Html,
    ChangeData,
    InputData,
    Renderable,
    ShouldRender,
};

pub use crate::app::App;

pub use crate::callback::Callback;

pub use stdweb::web::event::{
    BlurEvent,
    ClickEvent,
    ContextMenuEvent,
    DoubleClickEvent,
    DragDropEvent,
    DragEndEvent,
    DragEnterEvent,
    DragEvent,
    DragExitEvent,
    DragLeaveEvent,
    DragOverEvent,
    DragStartEvent,
    FocusEvent,
    GotPointerCaptureEvent,
    IKeyboardEvent,
    IMouseEvent,
    IPointerEvent,
    KeyDownEvent,
    KeyPressEvent,
    KeyUpEvent,
    LostPointerCaptureEvent,
    MouseDownEvent,
    MouseMoveEvent,
    MouseOutEvent,
    MouseEnterEvent,
    MouseLeaveEvent,
    MouseOverEvent,
    MouseUpEvent,
    MouseWheelEvent,
    PointerCancelEvent,
    PointerDownEvent,
    PointerEnterEvent,
    PointerLeaveEvent,
    PointerMoveEvent,
    PointerOutEvent,
    PointerOverEvent,
    PointerUpEvent,
    ScrollEvent,
    SubmitEvent
};

pub use crate::agent::{
    Bridge,
    Bridged,
    Threaded,
};

/// Prelude module for creating worker.
pub mod worker {
    pub use crate::agent::{
        Agent,
        AgentLink,
        Bridge,
        Bridged,
        Context,
        Global,
        HandlerId,
        Job,
        Private,
        Public,
        Transferable,
    };
}
