//! All possible elements of the gui

use super::Rectangle;
use super::VerticalLayout;
use super::HorizontalLayout;

// Interface

pub struct ChangePositionEvent<ElementId>{
    pub element_id: ElementId,
    pub x: u32,
    pub y: u32,
}

pub struct MouseEventResult<PressedId, ReleasedId>  {
    pub pressed_event:  Option<PressedId>,
    pub released_event: Option<ReleasedId>,
    pub consumed: bool,
}

pub trait GuiElementInterface<ElementId, PressedId, ReleasedId>  {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn resize(&mut self, abs_x: u32, abs_y: u32, res: &mut Vec::<ChangePositionEvent<ElementId>>);
    fn mouse_event(&mut self, abs_x: u32, abs_y: u32, pressed: bool, res: &mut MouseEventResult<PressedId, ReleasedId>);
}


// All possible elements

pub enum GuiElement<ElementId, PressedId, ReleasedId>
where ElementId: Copy, PressedId: Copy, ReleasedId:Copy
{
    Rectangle(Rectangle<ElementId, PressedId, ReleasedId>),
    VerticalLayout(VerticalLayout<ElementId, PressedId, ReleasedId>),
    HorizontalLayout(HorizontalLayout<ElementId, PressedId, ReleasedId>),
}

impl<ElementId, PressedId, ReleasedId>From<Rectangle<ElementId, PressedId, ReleasedId>> for GuiElement<ElementId, PressedId, ReleasedId>
    where ElementId: Copy, PressedId: Copy, ReleasedId:Copy
{
    fn from(value: Rectangle<ElementId, PressedId, ReleasedId>) -> Self {
        Self::Rectangle(value)
    }
}

impl<ElementId, PressedId, ReleasedId> From<VerticalLayout<ElementId, PressedId, ReleasedId>> for GuiElement<ElementId, PressedId, ReleasedId>
    where ElementId: Copy, PressedId: Copy, ReleasedId:Copy
{
    fn from(value: VerticalLayout<ElementId, PressedId, ReleasedId>) -> Self {
        Self::VerticalLayout(value)
    }
}

impl<ElementId, PressedId, ReleasedId> From<HorizontalLayout<ElementId, PressedId, ReleasedId>> for GuiElement<ElementId, PressedId, ReleasedId>
    where ElementId: Copy, PressedId: Copy, ReleasedId:Copy
{
    fn from(value: HorizontalLayout<ElementId, PressedId, ReleasedId>) -> Self {
        Self::HorizontalLayout(value)
    }
}

impl<ElementId, PressedId, ReleasedId> GuiElement<ElementId, PressedId, ReleasedId>
    where ElementId: Copy, PressedId: Copy, ReleasedId:Copy
{
    pub fn visit(&mut self) -> &mut dyn GuiElementInterface<ElementId, PressedId, ReleasedId>{
        match self {
            GuiElement::Rectangle(elem) => elem,
            GuiElement::VerticalLayout(elem) => elem,
            GuiElement::HorizontalLayout(elem) => elem,
        }
    }
}