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

pub enum ElementId<ButtonId, LabelId>
{
    Button(ButtonId),
    Label(LabelId),
}

pub struct ChangePositionEvent<ButtonId, LabelId>{
    pub element_id: ElementId<ButtonId, LabelId>,
    pub x: u32,
    pub y: u32,
}

impl<ButtonId, LabelId> ChangePositionEvent<ButtonId, LabelId> {
    pub fn new_button(button_id: ButtonId,
        x: u32,
        y: u32) -> Self 
    {
        let element_id = ElementId::<ButtonId, LabelId>::Button(button_id);

        Self {
            element_id,
            x,
            y,
        }
    }

    pub fn new_label(label_id: LabelId,
        x: u32,
        y: u32) -> Self 
    {
        let element_id = ElementId::<ButtonId, LabelId>::Label(label_id);

        Self {
            element_id,
            x,
            y,
        }
    }
}

pub struct ButtonPressedEvent<ButtonId>{
    pub button_id: ButtonId,
}

pub struct Gui<ButtonId, LabelId> 
where LabelId: Copy,
    ButtonId: Copy,
{
    width: u32,
    height: u32,

    mouse_pos_x: u32,
    mouse_pos_y: u32,

    elements: Vec<AlignedElement<ButtonId, LabelId>>,
}

impl<ButtonId, LabelId> Gui<ButtonId, LabelId> 
where LabelId: Copy,
    ButtonId: Copy,
{
    pub fn new(width: u32, height: u32, elements: Vec<AlignedElement<ButtonId, LabelId>>) -> Self {
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

    pub fn resize(&mut self, width: u32, height: u32) -> Vec<ChangePositionEvent<ButtonId, LabelId>> {
        self.width = width;
        self.height = height;
        
        let mut res = Vec::<ChangePositionEvent<ButtonId, LabelId>>::new();

        for elem in &mut self.elements {
            elem.resize(self.width, self.height, &mut res);
        }

        res
    }

    pub fn mouse_event(&mut self, mouse_event: MouseEvent) -> (bool, Option<ButtonPressedEvent<ButtonId>>) {
        
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