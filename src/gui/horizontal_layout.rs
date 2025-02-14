//! Arranges gui elements horizontally

use super::gui_element::ChangePositionEvent;
use super::gui_element::GuiElementInterface;
use super::gui_element::MouseEventResult;
use super::GuiElement;

pub struct HorizontalLayout<ElementId, PressedId, ReleasedId>
where
    ElementId: Copy,
    PressedId: Copy,
    ReleasedId: Copy,
{
    elements: Vec<GuiElement<ElementId, PressedId, ReleasedId>>,

    // cache sizes
    abs_x: u32,
    abs_y: u32,
    width: u32,
    height: u32,

    active: bool,
}

impl<ElementId, PressedId, ReleasedId> HorizontalLayout<ElementId, PressedId, ReleasedId>
where
    ElementId: Copy,
    PressedId: Copy,
    ReleasedId: Copy,
{
    pub fn new(elements: Vec<GuiElement<ElementId, PressedId, ReleasedId>>) -> Self {
        let mut vertical_layout = Self {
            elements,

            abs_x: 0,
            abs_y: 0,
            width: 0,
            height: 0,

            active: false,
        };

        vertical_layout.calculate_element_size();

        vertical_layout
    }

    fn calculate_element_size(&mut self) {
        let mut width = 0;
        let mut height = 0;

        for element in &mut self.elements {
            let element = element.visit();
            width = width + element.width();
            height = height.max(element.height());
        }

        self.width = width;
        self.height = height;
    }

    fn is_inside(&self, x: u32, y: u32) -> bool {
        x >= self.abs_x
            && x < self.abs_x + self.width
            && y >= self.abs_y
            && y < self.abs_y + self.height
    }
}

impl<ElementId, PressedId, ReleasedId> GuiElementInterface<ElementId, PressedId, ReleasedId>
    for HorizontalLayout<ElementId, PressedId, ReleasedId>
where
    ElementId: Copy,
    PressedId: Copy,
    ReleasedId: Copy,
{
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn resize(&mut self, abs_x: u32, abs_y: u32, res: &mut Vec<ChangePositionEvent<ElementId>>) {
        self.abs_x = abs_x;
        self.abs_y = abs_y;
        let mut delta_width = 0;

        for element in &mut self.elements {
            let element = element.visit();

            let element_abs_x = abs_x + delta_width;
            let element_abs_y = abs_y + self.height / 2 - element.height() / 2;
            element.resize(element_abs_x, element_abs_y, res);

            delta_width += element.width();
        }
    }

    fn mouse_event(
        &mut self,
        abs_x: u32,
        abs_y: u32,
        pressed: bool,
        res: &mut MouseEventResult<PressedId, ReleasedId>,
    ) {
        if !self.is_inside(abs_x, abs_y) && !self.active {
            return;
        }

        for element in &mut self.elements {
            let element = element.visit();
            element.mouse_event(abs_x, abs_y, pressed, res);
        }

        self.active = res.consumed;
    }
}
