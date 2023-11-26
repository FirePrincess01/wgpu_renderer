//! All possible elements of the gui

use super::ChangePositionEvent;
use super::Rectangle;
use super::RectanglePressedEvent;
use super::VerticalLayout;
use super::HorizontalLayout;

pub enum GuiElement<RectangleId>
where RectangleId: Copy,
{
    Rectangle(Rectangle<RectangleId>),
    VerticalLayout(VerticalLayout<RectangleId>),
    HorizontalLayout(HorizontalLayout<RectangleId>),
}


impl<RectangleId> From<Rectangle<RectangleId>> for GuiElement<RectangleId> 
    where RectangleId: Copy,
{
    fn from(value: Rectangle<RectangleId>) -> Self {
        Self::Rectangle(value)
    }
}

impl<RectangleId> From<VerticalLayout<RectangleId>> for GuiElement<RectangleId> 
    where RectangleId: Copy,
{
    fn from(value: VerticalLayout<RectangleId>) -> Self {
        Self::VerticalLayout(value)
    }
}

impl<RectangleId> From<HorizontalLayout<RectangleId>> for GuiElement<RectangleId> 
    where RectangleId: Copy,
{
    fn from(value: HorizontalLayout<RectangleId>) -> Self {
        Self::HorizontalLayout(value)
    }
}

pub trait GuiElementInterface<RectangleId> {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn resize(&mut self, abs_x: u32, abs_y: u32, res: &mut Vec::<ChangePositionEvent<RectangleId>>);
    fn mouse_pressed(&mut self, abs_x: u32, abs_y: u32) -> (bool, Option<RectanglePressedEvent<RectangleId>>);
    fn mouse_released(&mut self, abs_x: u32, abs_y: u32) -> (bool, Option<RectanglePressedEvent<RectangleId>>);
}

impl<RectangleId> GuiElement<RectangleId>
    where RectangleId: Copy,
{
    pub fn visit(&mut self) -> &mut dyn GuiElementInterface<RectangleId> {
        match self {
            GuiElement::Rectangle(elem) => elem,
            GuiElement::VerticalLayout(elem) => elem,
            GuiElement::HorizontalLayout(elem) => elem,
        }
    }
}