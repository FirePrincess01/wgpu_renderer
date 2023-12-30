//! Interface for the gui elements

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum ElementState {
    Pressed,
    Released,
}

#[derive(Copy, Clone)]
pub struct ElementSelectedEvent<ElementId> {
    pub element_id: ElementId,
    pub state: ElementState,
}

#[derive(Copy, Clone)]
pub struct MouseEventResult<ElementId>  {
    pub mouse_events: [Option<ElementSelectedEvent<ElementId>>; 2],
    pub consumed: bool,
}

impl<ElementId> MouseEventResult<ElementId> {
    pub fn new() -> Self {
        Self { mouse_events: [None, None], consumed: false }
    }
}

pub trait GuiElementInterface<ElementId> {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn resize(&mut self, abs_x: u32, abs_y: u32);
    fn mouse_event(&mut self, abs_x: u32, abs_y: u32, pressed: bool, res: &mut MouseEventResult<ElementId>);
}
