//! A clickable button

use super::gui_element::ElementSelectedEvent;
use super::gui_element::ElementState;
use super::gui_element::GuiElementInterface;
use super::gui_element::MouseEventResult;

pub struct Rectangle<ElementId> 
    where ElementId: Copy
{
    width: u32,
    height: u32,
    boarder: u32,
    element_id: ElementId,

    // cache sizes
    abs_x: u32,
    abs_y: u32,
    pressed: bool,
}

impl<ElementId>  Rectangle<ElementId> 
    where ElementId: Copy
{
    pub fn new(element_id: ElementId,
        width: u32,
        height: u32,
        boarder: u32,
    ) -> Self
    {
        Self{ 
            element_id,

            width, 
            height, 
            boarder,
                 
            abs_x: 0,
            abs_y: 0,
            pressed: false,
        }
    }

    fn is_inside(&self, x: u32, y: u32) -> bool {
        x >= self.abs_x + self.boarder && x <= self.abs_x + self.width + self.boarder &&
        y >= self.abs_y + self.boarder && y <= self.abs_y + self.height + self.boarder 
    }

    pub fn boarder(&self) -> u32 {
        self.boarder 
    }
}

impl<ElementId>  GuiElementInterface<ElementId> for Rectangle<ElementId>  
where ElementId: Copy
{
    fn width(&self) -> u32 {
        self.width + 2 * self.boarder 
    }

    fn height(&self) -> u32 {
        self.height + 2 * self.boarder 
    }

    fn resize(&mut self, abs_x: u32, abs_y: u32) {
        self.abs_x = abs_x;
        self.abs_y = abs_y;
    }

    fn mouse_event(&mut self, abs_x: u32, abs_y: u32, pressed: bool, res: &mut MouseEventResult<ElementId>) {
        let is_inside = self.is_inside(abs_x, abs_y);

        // pressed
        if !self.pressed && (is_inside && pressed) {
            self.pressed = true;

            for element in res.mouse_events.iter_mut() {
                if element.is_none() {
                    *element = Some(ElementSelectedEvent{
                        element_id:  self.element_id,
                        state: ElementState::Pressed,
                    });
                    break;
                }
            }
        }

        // released
        if self.pressed && (!is_inside || !pressed) {
            self.pressed = false;

            for element in res.mouse_events.iter_mut() {
                if element.is_none() {
                    *element = Some(ElementSelectedEvent{
                        element_id:  self.element_id,
                        state: ElementState::Released,
                    });
                    break;
                }
            }
        }

        res.consumed = res.consumed || is_inside;
    }
}
