//! A clickable button

use super::RectanglePressedEvent;
use super::ChangePositionEvent;
use super::gui_element::GuiElementInterface;

pub struct Rectangle<RectangleId> 
    where RectangleId: Copy
{
    width: u32,
    height: u32,
    boarder: u32,
    rectangle_id: RectangleId,

    pressed_event: bool,
    released_event: bool,

    // cache sizes
    abs_x: u32,
    abs_y: u32,
    pressed: bool,
}

impl<RectangleId> Rectangle<RectangleId>
    where RectangleId: Copy
{
    pub fn new(rectangle_id: RectangleId,
        width: u32,
        height: u32,
        boarder: u32,
    ) -> Self
    {
        Self::new_raw(rectangle_id, 
            width, 
            height, 
            boarder, 
            false, 
            true)
    }

    pub fn new_raw(rectangle_id: RectangleId,
        width: u32,
        height: u32,
        boarder: u32,
        pressed_event: bool,
        released_event: bool,
    ) -> Self
    {
        Self{ 
            width, 
            height, 
            boarder,
            rectangle_id,
         
            pressed_event,
            released_event,
        
            abs_x: 0,
            abs_y: 0,
            pressed: false,
        }
    }

    pub fn _id(&self) -> RectangleId {
        self.rectangle_id
    }

    fn is_inside(&self, x: u32, y: u32) -> bool {
        x >= self.abs_x + self.boarder && x <= self.abs_x + self.width + self.boarder &&
        y >= self.abs_y + self.boarder && y <= self.abs_y + self.height + self.boarder 
    }
}

impl<RectangleId> GuiElementInterface<RectangleId> for Rectangle<RectangleId> 
where RectangleId: Copy
{
    fn width(&self) -> u32 {
        self.width + 2 * self.boarder 
    }

    fn height(&self) -> u32 {
        self.height + 2 * self.boarder 
    }

    fn resize(&mut self, abs_x: u32, abs_y: u32, res: &mut Vec::<ChangePositionEvent<RectangleId>>) {
        self.abs_x = abs_x;
        self.abs_y = abs_y;

        res.push(ChangePositionEvent::new(
            self.rectangle_id, 
            self.abs_x + self.boarder, 
            self.abs_y + self.boarder));
    }

    fn mouse_pressed(&mut self, abs_x: u32, abs_y: u32) -> (bool, Option<RectanglePressedEvent<RectangleId>>) {
        if !self.is_inside(abs_x, abs_y) {
            return (false, None);
        }

        if !self.pressed {
            self.pressed = true;

            if self.pressed_event {
                let event = RectanglePressedEvent{rectangle_id: self.rectangle_id, pressed: true};
                return (true, Some(event));
            }
        }
        
        (true, None)
    }

    fn mouse_released(&mut self, abs_x: u32, abs_y: u32) -> (bool, Option<RectanglePressedEvent<RectangleId>>) {
        if !self.is_inside(abs_x, abs_y) {
            return (false, None);
        }

        if self.pressed {
            self.pressed = false;

            if self.released_event {
                let event = RectanglePressedEvent{rectangle_id: self.rectangle_id, pressed: false};
                return (true, Some(event));
            }
        }
        
        (true, None)
    }


}