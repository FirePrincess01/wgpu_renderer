//! Fixes a gui element to an edge of the window

use super::GuiElement;
use super::ChangePositionEvent;
use super::ButtonPressedEvent;

#[allow(dead_code)]
pub enum Alignment {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub struct AlignedElement<ButtonId, LabelId>
where LabelId: Copy,
    ButtonId: Copy,
{
    alignment: Alignment,
    x: u32,
    y: u32,
    
    element: GuiElement<ButtonId, LabelId>,

    // cache sizes
    abs_x: u32,
    abs_y: u32,
    width: u32,
    height: u32,

}

impl<ButtonId, LabelId> AlignedElement<ButtonId, LabelId>
where LabelId: Copy,
    ButtonId: Copy,
{
    pub fn new(alignment: Alignment, x: u32, y:u32, element: GuiElement<ButtonId, LabelId>) -> Self 
    {
        Self {
            alignment,
            x,
            y,
            element,

            abs_x: 0,
            abs_y: 0,
            width: 0,
            height: 0,
        }
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
        }
    }

    fn calculate_element_size(&mut self) {
        match &mut self.element {
            GuiElement::Button(button) => {
                self.width = button.width();
                self.height = button.height();
            }
            GuiElement::Label(label) => {
                self.width = label.width();
                self.height = label.height();
            }
            GuiElement::VerticalLayout(vertical_layout) => {
                self.width = vertical_layout.width();
                self.height = vertical_layout.height();
            }
        }
    }

    pub fn resize(&mut self, gui_width: u32, gui_height: u32, res: &mut Vec::<ChangePositionEvent<ButtonId, LabelId>>)
    {
        self.calculate_element_size();
        self.calculate_absolute_position(gui_width, gui_height);

        match &mut self.element {
            GuiElement::Button(button) => {
                button.set_abs_pos(self.abs_x, self.abs_y);
                res.push(button.change_position_event());
            }
            GuiElement::Label(label) => {
                label.set_abs_pos(self.abs_x, self.abs_y);
                res.push(label.change_position_event());
            }
            GuiElement::VerticalLayout(vertical_layout) => {
                vertical_layout.resize(self.abs_x, self.abs_y, res);
            }
        }
    }

    fn is_inside(&self, x: u32, y: u32) -> bool {
        x >= self.abs_x && x < self.abs_x + self.width &&
        y >= self.abs_y && y < self.abs_y + self.height 
    }

    pub fn mouse_pressed(&mut self, abs_x: u32, abs_y: u32) -> (bool, Option<ButtonPressedEvent<ButtonId>>) {
        if !self.is_inside(abs_x, abs_y) {
            return (false, None);
        }

        match &mut self.element {
            GuiElement::Button(button) => button.mouse_pressed(abs_x, abs_y),
            GuiElement::Label(label) => (label.mouse_pressed(abs_x, abs_y), None),
            GuiElement::VerticalLayout(vertical_layout) => vertical_layout.mouse_pressed(abs_x, abs_y),
        }        
    }

    pub fn mouse_released(&mut self, abs_x: u32, abs_y: u32) -> (bool, Option<ButtonPressedEvent<ButtonId>>) {
        if !self.is_inside(abs_x, abs_y) {
            return (false, None);
        }

        match &mut self.element {
            GuiElement::Button(button) => button.mouse_released(abs_x, abs_y),
            GuiElement::Label(label) => (label.mouse_released(abs_x, abs_y), None),
            GuiElement::VerticalLayout(vertical_layout) => vertical_layout.mouse_released(abs_x, abs_y),
        }  
    }
}
