//! Handles the collision detection of gui elements

mod gui_element;
mod gui;
mod aligned_element;
mod vertical_layout;
mod button;
mod label;

#[cfg(test)]
mod tests;

pub use gui_element::GuiElement;
pub use gui::Gui;
pub use gui::ChangePositionEvent;
pub use gui::ButtonPressedEvent;
pub use gui::ElementId;
pub use gui::MouseEvent;
pub use aligned_element::AlignedElement;
pub use aligned_element::Alignment;
pub use vertical_layout::VerticalLayout;
pub use button::Button;
pub use label::Label;
