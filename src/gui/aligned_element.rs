//! Fixes a gui element to an edge of the window

use super::gui_element::{ChangePositionEvent, MouseEventResult};
use super::GuiElement;

#[allow(dead_code)]
pub enum Alignment {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

pub struct AlignedElement<ElementId, PressedId, ReleasedId>
where
    ElementId: Copy,
    PressedId: Copy,
    ReleasedId: Copy,
{
    alignment: Alignment,
    x: u32,
    y: u32,

    element: GuiElement<ElementId, PressedId, ReleasedId>,

    // cache sizes
    abs_x: u32,
    abs_y: u32,
    width: u32,
    height: u32,

    active: bool,
}

impl<ElementId, PressedId, ReleasedId> AlignedElement<ElementId, PressedId, ReleasedId>
where
    ElementId: Copy,
    PressedId: Copy,
    ReleasedId: Copy,
{
    pub fn new(
        alignment: Alignment,
        x: u32,
        y: u32,
        element: GuiElement<ElementId, PressedId, ReleasedId>,
    ) -> Self {
        Self {
            alignment,
            x,
            y,
            element,

            abs_x: 0,
            abs_y: 0,
            width: 0,
            height: 0,

            active: false,
        }
    }

    fn calculate_absolute_position(&mut self, gui_width: u32, gui_height: u32) {
        match self.alignment {
            Alignment::TopLeft => {
                self.abs_x = self.x;
                self.abs_y = gui_height - self.y - self.height;
            }
            Alignment::TopRight => {
                self.abs_x = gui_width - self.x - self.width;
                self.abs_y = gui_height - self.y - self.height;
            }
            Alignment::BottomLeft => {
                self.abs_x = self.x;
                self.abs_y = self.y;
            }
            Alignment::BottomRight => {
                self.abs_x = gui_width - self.x - self.width;
                self.abs_y = self.y;
            }
            Alignment::Center => {
                self.abs_x = gui_width / 2 - self.x - self.width / 2;
                self.abs_y = gui_height / 2 - self.y - self.height / 2;
            }
        }
    }

    fn calculate_element_size(&mut self) {
        let element = self.element.visit();
        self.width = element.width();
        self.height = element.height();
    }

    pub fn resize(
        &mut self,
        gui_width: u32,
        gui_height: u32,
        res: &mut Vec<ChangePositionEvent<ElementId>>,
    ) {
        self.calculate_element_size();
        self.calculate_absolute_position(gui_width, gui_height);

        let element = self.element.visit();
        element.resize(self.abs_x, self.abs_y, res);
    }

    fn is_inside(&self, x: u32, y: u32) -> bool {
        x >= self.abs_x
            && x < self.abs_x + self.width
            && y >= self.abs_y
            && y < self.abs_y + self.height
    }

    pub fn mouse_event(
        &mut self,
        abs_x: u32,
        abs_y: u32,
        pressed: bool,
        res: &mut MouseEventResult<PressedId, ReleasedId>,
    ) {
        if !self.is_inside(abs_x, abs_y) && !self.active {
            return;
        }

        let element = self.element.visit();
        element.mouse_event(abs_x, abs_y, pressed, res);

        self.active = res.consumed;
    }
}
