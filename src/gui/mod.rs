//! Handles the collision detection of gui elements

mod gui_element;
mod gui;
mod aligned_element;
mod vertical_layout;
mod horizontal_layout;
mod rectangle;

#[cfg(test)]
mod tests;

pub use gui_element::GuiElement;
pub use gui::Gui;
pub use gui::ChangePositionEvent;
pub use gui::RectanglePressedEvent;
pub use gui::MouseEvent;
pub use aligned_element::AlignedElement;
pub use aligned_element::Alignment;
pub use vertical_layout::VerticalLayout;
pub use horizontal_layout::HorizontalLayout;
pub use rectangle::Rectangle;
