//! Handles the collision detection of gui elements

mod gui_element;
mod aligned_element;
mod vertical_layout;
mod horizontal_layout;
mod rectangle;

#[cfg(test)]
mod tests;

// pub use gui_element::GuiElement;
// pub use gui::Gui;
pub use gui_element::MouseEventResult;
pub use aligned_element::MouseEvent;
pub use aligned_element::AlignedElement;
pub use aligned_element::AlignedElementAssembled;
pub use aligned_element::Alignment;
pub use vertical_layout::VerticalLayout;
// pub use horizontal_layout::HorizontalLayout;
pub use rectangle::Rectangle;

#[derive(Copy, Clone)]
pub enum NoId{

}