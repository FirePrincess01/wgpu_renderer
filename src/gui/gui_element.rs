//! All possible elements of the gui

use super::Rectangle;
use super::VerticalLayout;

#[allow(dead_code)]
pub enum GuiElement<RectangleId>
where RectangleId: Copy,
{
    Rectangle(Rectangle<RectangleId>),
    VerticalLayout(VerticalLayout<RectangleId>),
    // HorizontalLayout(GuiHorizontalLayout),
}