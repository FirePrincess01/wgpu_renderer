//! Handles the collision detection of gui elements

use super::AlignedElement;

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

pub struct ChangePositionEvent<RectangleId>{
    pub rectangle_id: RectangleId,
    pub x: u32,
    pub y: u32,
}

impl<RectangleId> ChangePositionEvent<RectangleId> {
    pub fn new(rectangle_id: RectangleId,
        x: u32,
        y: u32) -> Self 
    {
        Self {
            rectangle_id,
            x,
            y,
        }
    }
}

pub struct RectanglePressedEvent<RectangleId>{
    pub rectangle_id: RectangleId,
    pub pressed: bool,
}

pub struct Gui<RectangleId> 
where RectangleId: Copy,
{
    width: u32,
    height: u32,

    mouse_pos_x: u32,
    mouse_pos_y: u32,

    elements: Vec<AlignedElement<RectangleId>>,
}

impl<RectangleId> Gui<RectangleId> 
where RectangleId: Copy,
{
    pub fn new(width: u32, height: u32, elements: Vec<AlignedElement<RectangleId>>) -> Self {
        let mut gui = Self {
            width,
            height,

            mouse_pos_x: 0,
            mouse_pos_y: 0,
            
            elements,
        };

        gui.resize(width, height);  // resize runs through all the elements

        gui
    }

    pub fn resize(&mut self, width: u32, height: u32) -> Vec<ChangePositionEvent<RectangleId>> {
        self.width = width;
        self.height = height;
        
        let mut res = Vec::<ChangePositionEvent<RectangleId>>::new();

        for elem in &mut self.elements {
            elem.resize(self.width, self.height, &mut res);
        }

        res
    }

    pub fn mouse_event(&mut self, mouse_event: MouseEvent) -> (bool, Option<RectanglePressedEvent<RectangleId>>) {
        
        match mouse_event {
            MouseEvent::Pressed => {
                for elem in &mut self.elements {
                    let (consumed, res) = elem.mouse_pressed(self.mouse_pos_x, self.mouse_pos_y);
                    if consumed {
                        return (consumed, res);
                    }
                }
            },
            MouseEvent::Released => {
                for elem in &mut self.elements {
                    let (consumed, res) = elem.mouse_released(self.mouse_pos_x, self.mouse_pos_y);
                    if consumed {
                        return (consumed, res);
                    }
                }
            },
            MouseEvent::Moved { x, y } => {
                self.mouse_pos_x = x;
                self.mouse_pos_y = y;
            },
        }

        (false, None)
    }
}