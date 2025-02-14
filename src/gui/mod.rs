//! Handles the collision detection of gui elements

mod aligned_element;
mod gui;
mod gui_element;
mod horizontal_layout;
mod rectangle;
mod vertical_layout;

#[cfg(test)]
mod tests;

pub use aligned_element::AlignedElement;
pub use aligned_element::Alignment;
pub use gui::Gui;
pub use gui::MouseEvent;
pub use gui_element::GuiElement;
pub use gui_element::MouseEventResult;
pub use horizontal_layout::HorizontalLayout;
pub use rectangle::Rectangle;
pub use vertical_layout::VerticalLayout;

#[derive(Copy, Clone)]
pub enum NoId {}
