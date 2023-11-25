//! A clickable button

use super::RectanglePressedEvent;
use super::ChangePositionEvent;

pub struct Rectangle<RectangleId> 
    where RectangleId: Copy
{
    width: u32,
    height: u32,
    boarder: u32,
    rectangle_id: RectangleId,

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
        Self{ 
            width, 
            height, 
            boarder,
            rectangle_id,

            abs_x: 0,
            abs_y: 0,
            pressed: false,
        }
    }

    pub fn _id(&self) -> RectangleId {
        self.rectangle_id
    }

    pub fn width(&self) -> u32 {
        self.width + 2 * self.boarder 
    }

    pub fn height(&self) -> u32 {
        self.height + 2 * self.boarder 
    }

    pub fn set_abs_pos(&mut self, abs_x: u32, abs_y: u32) {
        self.abs_x = abs_x;
        self.abs_y = abs_y;
    }

    fn is_inside(&self, x: u32, y: u32) -> bool {
        x >= self.abs_x + self.boarder && x <= self.abs_x + self.width + self.boarder &&
        y >= self.abs_y + self.boarder && y <= self.abs_y + self.height + self.boarder 
    }

    pub fn mouse_pressed(&mut self, abs_x: u32, abs_y: u32) -> (bool, Option<RectanglePressedEvent<RectangleId>>) {
        if !self.is_inside(abs_x, abs_y) {
            return (false, None);
        }

        self.pressed = true;
        (true, None)
    }

    pub fn mouse_released(&mut self, abs_x: u32, abs_y: u32) -> (bool, Option<RectanglePressedEvent<RectangleId>>) {
        if !self.is_inside(abs_x, abs_y) {
            return (false, None);
        }

        if self.pressed {
            self.pressed = false;

            let event = RectanglePressedEvent{rectangle_id: self.rectangle_id};
            (true, Some(event))
        }
        else {
            (true, None)
        }
    }

    pub fn change_position_event(&self) -> ChangePositionEvent::<RectangleId>
    {
        ChangePositionEvent::new(
            self.rectangle_id, 
            self.abs_x + self.boarder, 
            self.abs_y + self.boarder)
    }
}