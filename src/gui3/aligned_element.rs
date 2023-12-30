//! Fixes a gui element to an edge of the window

use super::{gui_element::GuiElementInterface, MouseEventResult};

#[derive(Copy, Clone)]
pub enum MouseEvent {
    Pressed,
    Released,
    Moved{
        x: u32,
        y: u32,
    }
}

pub enum Alignment {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

pub struct AlignedElement
{
    alignment: Alignment,
    x: u32,
    y: u32,

    // cache sizes
    abs_x: u32,
    abs_y: u32,
    width: u32,
    height: u32,

    active: bool,

    // handle mouse
    mouse_pos_x: u32,
    mouse_pos_y: u32,
    mouse_pressed: bool,
}

impl AlignedElement
{
    pub fn new(alignment: Alignment, x: u32, y:u32) -> Self 
    {
        Self {
            alignment,
            x,
            y,

            abs_x: 0,
            abs_y: 0,
            width: 0,
            height: 0,

            active: false,

            mouse_pos_x: 0,
            mouse_pos_y: 0,
            mouse_pressed: false,
        }
    }

    pub fn assemble<'a, ElementId>(&'a mut self, element: &'a mut dyn GuiElementInterface<ElementId>) -> AlignedElementAssembled<'a, ElementId> {
        AlignedElementAssembled::new(self, element)
    }

    fn calculate_absolute_position(&mut self, gui_width: u32, gui_height: u32) {
        match self.alignment {
            Alignment::TopLeft =>     {
                self.abs_x = self.x;
                self.abs_y = gui_height - self.y - self.height;
            }
            Alignment::TopRight =>    {
                self.abs_x = gui_width - self.x - self.width;
                self.abs_y = gui_height - self.y - self.height;
            }
            Alignment::BottomLeft =>  {
                self.abs_x = self.x;
                self.abs_y = self.y;
            }
            Alignment::BottomRight => {
                self.abs_x = gui_width - self.x - self.width;
                self.abs_y = self.y;
            }
            Alignment::Center => {
                self.abs_x = gui_width/2 - self.x - self.width/2;
                self.abs_y = gui_height/2 - self.y - self.height/2;
            },
        }
    }

    fn is_inside(&self, x: u32, y: u32) -> bool {
        x >= self.abs_x && x < self.abs_x + self.width &&
        y >= self.abs_y && y < self.abs_y + self.height 
    }
}


pub struct AlignedElementAssembled<'a, ElementId>
{
    layout: &'a mut AlignedElement,
    element: &'a mut dyn GuiElementInterface<ElementId>, 
}

impl<'a, ElementId> AlignedElementAssembled<'a, ElementId> {
    fn new(layout: &'a mut AlignedElement, element: &'a mut dyn GuiElementInterface<ElementId>) -> Self 
    { 
        let mut obj = Self { layout, element };
        obj.calculate_element_size();
        obj
    }

    fn calculate_element_size(&mut self) {
        self.layout.width = self.element.width();
        self.layout.height = self.element.height();
    }

    pub fn resize(&mut self, gui_width: u32, gui_height: u32)
    {
        // self.calculate_element_size();

        self.layout.calculate_absolute_position(gui_width, gui_height);

        self.element.resize(self.layout.abs_x, self.layout.abs_y);
    }

    fn mouse_event_(&mut self, abs_x: u32, abs_y: u32, pressed: bool, res: &mut MouseEventResult<ElementId>)
    {
        if !self.layout.is_inside(abs_x, abs_y) && !self.layout.active {
            return;
        }

        self.element.mouse_event(abs_x, abs_y, pressed, res);   

        self.layout.active = res.consumed;
    }

    pub fn mouse_event(&mut self, mouse_event: MouseEvent) -> MouseEventResult<ElementId> {
        
        match mouse_event {
            MouseEvent::Pressed => {
                self.layout.mouse_pressed = true;
            },
            MouseEvent::Released => {
                self.layout.mouse_pressed = false;
            },
            MouseEvent::Moved { x, y } => {
                self.layout.mouse_pos_x = x;
                self.layout.mouse_pos_y = y;
            },
        }

        let mut res = MouseEventResult{ mouse_events: [None, None], consumed: false };
        self.mouse_event_(self.layout.mouse_pos_x, self.layout.mouse_pos_y, self.layout.mouse_pressed, &mut res);

        res
    }
}
