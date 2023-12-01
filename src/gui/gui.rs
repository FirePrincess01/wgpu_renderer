//! Handles the collision detection of gui elements

use super::{AlignedElement, gui_element::{MouseEventResult, ChangePositionEvent}};

//
// (x:0, y:height)   (x:width, y:height)
//     ---------------
//     |             |
//     |             |
//     |             |
//     |             |
//     |             |
//     |             |
//     ---------------
//  (x:0, y:0)       (x:width, y:0)

#[derive(Copy, Clone)]
pub enum MouseEvent {
    Pressed,
    Released,
    Moved{
        x: u32,
        y: u32,
    }
}

pub struct Gui<ElementId, PressedId, ReleasedId> 
where ElementId: Copy, PressedId: Copy, ReleasedId:Copy
{
    width: u32,
    height: u32,

    mouse_pos_x: u32,
    mouse_pos_y: u32,
    mouse_pressed: bool,

    elements: Vec<AlignedElement<ElementId, PressedId, ReleasedId>>,
}

impl<ElementId, PressedId, ReleasedId> Gui<ElementId, PressedId, ReleasedId> 
where ElementId: Copy, PressedId: Copy, ReleasedId:Copy
{
    pub fn new(width: u32, height: u32, elements: Vec<AlignedElement<ElementId, PressedId, ReleasedId>>) -> Self {
        let mut gui = Self {
            width,
            height,

            mouse_pos_x: 0,
            mouse_pos_y: 0,
            mouse_pressed: false,
            
            elements,
        };

        gui.resize(width, height);  // resize runs through all the elements

        gui
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Vec<ChangePositionEvent<ElementId>> {
        self.width = width;
        self.height = height;
        
        let mut res = Vec::<ChangePositionEvent<ElementId>>::new();

        for elem in &mut self.elements {
            elem.resize(self.width, self.height, &mut res);
        }

        res
    }

    pub fn mouse_event(&mut self, mouse_event: MouseEvent) -> MouseEventResult<PressedId, ReleasedId> {
        
        match mouse_event {
            MouseEvent::Pressed => {
                self.mouse_pressed = true;
            },
            MouseEvent::Released => {
                self.mouse_pressed = false;
            },
            MouseEvent::Moved { x, y } => {
                self.mouse_pos_x = x;
                self.mouse_pos_y = y;
            },
        }

        let mut res = MouseEventResult{ released_event: None, pressed_event: None, consumed: false };
        for elem in &mut self.elements {
            elem.mouse_event(self.mouse_pos_x, self.mouse_pos_y, self.mouse_pressed, &mut res);
        }

        res
    }
}