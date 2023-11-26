//! Arranges gui elements vertically

use super::GuiElement;
use super::ChangePositionEvent;
use super::RectanglePressedEvent;
use super::gui_element::GuiElementInterface;

pub struct VerticalLayout<RectangleId> 
where RectangleId: Copy,
{
    elements: Vec<GuiElement<RectangleId>>,

    // cache sizes
    abs_x: u32,
    abs_y: u32,
    width: u32,
    height: u32,
}

impl<RectangleId> VerticalLayout<RectangleId> 
where RectangleId: Copy,
{
    pub fn new(elements: Vec<GuiElement<RectangleId>>) -> Self 
    {
        let mut vertical_layout = Self {
            elements,

            abs_x: 0,
            abs_y: 0,
            width: 0,
            height: 0,
        };

        vertical_layout.calculate_element_size();

        vertical_layout
    }

    fn calculate_element_size(&mut self) {

        let mut width =  0;
        let mut height = 0;

        for element in &mut self.elements {
            let element = element.visit();
            width = width.max(element.width()); 
            height = height + element.height();
        }

        self.width = width;
        self.height = height;


    }

    fn is_inside(&self, x: u32, y: u32) -> bool {
        x >= self.abs_x && x < self.abs_x + self.width &&
        y >= self.abs_y && y < self.abs_y + self.height 
    }
}

impl<RectangleId> GuiElementInterface<RectangleId> for VerticalLayout<RectangleId> 
where RectangleId: Copy,
{
    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn resize(&mut self, abs_x: u32, abs_y: u32, res: &mut Vec::<ChangePositionEvent<RectangleId>>) {
        self.abs_x = abs_x;
        self.abs_y = abs_y;
        let mut delta_height = self.height;

        for element in &mut self.elements {
            let element = element.visit();

            delta_height -= element.height();

            let element_abs_x = abs_x  + self.width/2 - element.width()/2;
            let element_abs_y = abs_y + delta_height;
            element.resize(element_abs_x, element_abs_y, res);
        }
    }

    fn mouse_pressed(&mut self, abs_x: u32, abs_y: u32) -> (bool, Option<RectanglePressedEvent<RectangleId>>) {
        if !self.is_inside(abs_x, abs_y) {
            return (false, None);
        }

        for element in &mut self.elements {
            let element = element.visit();

            let (consumed, event) = element.mouse_pressed(abs_x, abs_y);
            if consumed {
                return (true, event);
            }        
        }

        (false, None) 
    }

    fn mouse_released(&mut self, abs_x: u32, abs_y: u32) -> (bool, Option<RectanglePressedEvent<RectangleId>>) {
        if !self.is_inside(abs_x, abs_y) {
            return (false, None);
        }

        for element in &mut self.elements {
            let element = element.visit();
            let (consumed, event) = element.mouse_released(abs_x, abs_y);
            if consumed {
                return (true, event);
            }        
        }

        (false, None) 
    }
}