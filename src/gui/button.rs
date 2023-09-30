//! A clickable button

use super::ButtonPressedEvent;
use super::ChangePositionEvent;

pub struct Button<ButtonId> 
    where ButtonId: Copy
{
    width: u32,
    height: u32,
    boarder: u32,
    button_id: ButtonId,

    // cache sizes
    abs_x: u32,
    abs_y: u32,
    pressed: bool,
}

impl<ButtonId> Button<ButtonId>
    where ButtonId: Copy
{
    pub fn new(width: u32,
        height: u32,
        boarder: u32,
        button_id: ButtonId) -> Self
    {
        Self{ 
            width, 
            height, 
            boarder,
            button_id,

            abs_x: 0,
            abs_y: 0,
            pressed: false,
        }
    }

    pub fn _id(&self) -> ButtonId {
        self.button_id
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

    pub fn mouse_pressed(&mut self, abs_x: u32, abs_y: u32) -> (bool, Option<ButtonPressedEvent<ButtonId>>) {
        if !self.is_inside(abs_x, abs_y) {
            return (false, None);
        }

        self.pressed = true;
        (true, None)
    }

    pub fn mouse_released(&mut self, abs_x: u32, abs_y: u32) -> (bool, Option<ButtonPressedEvent<ButtonId>>) {
        if !self.is_inside(abs_x, abs_y) {
            return (false, None);
        }

        if self.pressed {
            self.pressed = false;

            let event = ButtonPressedEvent{button_id: self.button_id};
            (true, Some(event))
        }
        else {
            (true, None)
        }
    }

    pub fn change_position_event<LabelId>(&self) -> ChangePositionEvent::<ButtonId, LabelId>
    {
        ChangePositionEvent::<ButtonId, LabelId>::new_button(
            self.button_id, 
            self.abs_x + self.boarder, 
            self.abs_y + self.boarder)
    }
}