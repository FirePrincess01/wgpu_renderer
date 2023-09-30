//! Shows a text label

use super::ChangePositionEvent;

pub struct Label<LabelId> 
    where LabelId: Copy
{
    width: u32,
    height: u32,
    boarder: u32,
    label_id: LabelId,

    // cache sizes
    abs_x: u32,
    abs_y: u32,
}

#[allow(dead_code)]
impl<LabelId> Label<LabelId>
    where LabelId: Copy
{
    pub fn new(width: u32,
        height: u32,
        boarder: u32,
        label_id: LabelId) -> Self
    {
        Self{ 
            width, 
            height, 
            boarder,
            label_id,

            abs_x: 0,
            abs_y: 0,
        }
    }

    pub fn id(&self) -> LabelId {
        self.label_id
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

    pub fn mouse_pressed(&self, abs_x: u32, abs_y: u32) -> bool {
        if !self.is_inside(abs_x, abs_y) {
            return false;
        }

        true
    }

    pub fn mouse_released(&self, abs_x: u32, abs_y: u32) -> bool {
        if !self.is_inside(abs_x, abs_y) {
            return false;
        }

        true
    }

    pub fn change_position_event<ButtonId>(&self) -> ChangePositionEvent::<ButtonId, LabelId>
    {
        ChangePositionEvent::<ButtonId, LabelId>::new_label(
            self.label_id, 
            self.abs_x + self.boarder, 
            self.abs_y + self.boarder)
    }
}