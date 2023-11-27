//! A clickable button

use super::gui_element::ChangePositionEvent;
use super::gui_element::GuiElementInterface;
use super::gui_element::MouseEventResult;

pub struct Rectangle<ElementId, PressedId, ReleasedId> 
    where ElementId: Copy, PressedId: Copy, ReleasedId:Copy
{
    width: u32,
    height: u32,
    boarder: u32,
    rectangle_id: ElementId,
    pressed_id: Option<PressedId>,
    released_id: Option<ReleasedId>,

    // cache sizes
    abs_x: u32,
    abs_y: u32,
    pressed: bool,
}

impl<ElementId, PressedId, ReleasedId>  Rectangle<ElementId, PressedId, ReleasedId> 
    where ElementId: Copy, PressedId: Copy, ReleasedId:Copy
{
    pub fn new(rectangle_id: ElementId, 
        width: u32,
        height: u32,
        boarder: u32,
    ) -> Self
    {
        Self::new_param(rectangle_id, 
            None,
            None,
            width, 
            height, 
            boarder)
    }

    pub fn new_btn(rectangle_id: ElementId, 
        released_id: ReleasedId,
        width: u32,
        height: u32,
        boarder: u32,
    ) -> Self
    {
        Self::new_param(rectangle_id, 
            None,
            Some(released_id),
            width, 
            height, 
            boarder)
    }

    pub fn new_generic(rectangle_id: ElementId,
        pressed_id: PressedId,
        released_id: ReleasedId,
        width: u32,
        height: u32,
        boarder: u32
    ) -> Self
    {
        Self::new_param(rectangle_id, 
            Some(pressed_id),
            Some(released_id),
            width, 
            height, 
            boarder)
    }

    pub fn new_param(rectangle_id: ElementId,
        pressed_id: Option<PressedId>,
        released_id: Option<ReleasedId>,
        width: u32,
        height: u32,
        boarder: u32,
    ) -> Self
    {
        Self{ 
            rectangle_id,
            pressed_id,
            released_id,

            width, 
            height, 
            boarder,
                 
            abs_x: 0,
            abs_y: 0,
            pressed: false,
        }
    }

    pub fn _id(&self) -> ElementId {
        self.rectangle_id
    }

    fn is_inside(&self, x: u32, y: u32) -> bool {
        x >= self.abs_x + self.boarder && x <= self.abs_x + self.width + self.boarder &&
        y >= self.abs_y + self.boarder && y <= self.abs_y + self.height + self.boarder 
    }
}

impl<ElementId, PressedId, ReleasedId>  GuiElementInterface<ElementId, PressedId, ReleasedId> for Rectangle<ElementId, PressedId, ReleasedId> 
where ElementId: Copy, PressedId: Copy, ReleasedId:Copy
{
    fn width(&self) -> u32 {
        self.width + 2 * self.boarder 
    }

    fn height(&self) -> u32 {
        self.height + 2 * self.boarder 
    }

    fn resize(&mut self, abs_x: u32, abs_y: u32, res: &mut Vec::<ChangePositionEvent<ElementId>>) {
        self.abs_x = abs_x;
        self.abs_y = abs_y;

        res.push(ChangePositionEvent{ 
            element_id: self.rectangle_id, 
            x: self.abs_x + self.boarder, 
            y: self.abs_y + self.boarder });
    }

    fn mouse_event(&mut self, abs_x: u32, abs_y: u32, pressed: bool, res: &mut MouseEventResult<PressedId, ReleasedId>)
    {   
        let is_inside = self.is_inside(abs_x, abs_y);

        if !self.pressed && (is_inside && pressed) {
            self.pressed = true;
            res.pressed_event = self.pressed_id;
        }

        if self.pressed && (!is_inside || !pressed) {
            self.pressed = false;
            res.released_event = self.released_id;
        }

        res.consumed = res.consumed || is_inside;
    }


}