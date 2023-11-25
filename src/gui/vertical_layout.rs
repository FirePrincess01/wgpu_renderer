//! Arranges gui elements vertically

use super::GuiElement;
use super::ChangePositionEvent;
use super::RectanglePressedEvent;

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

        for element in &self.elements 
        {
            match element {
                GuiElement::Rectangle(rectangle) => {
                    width = width.max(rectangle.width()); 
                    height = height + rectangle.height();
                }
                GuiElement::VerticalLayout(vertical_layout) => {
                    width = width.max(vertical_layout.width()); 
                    height = height + vertical_layout.height();
                }
            }
        }

        self.width = width;
        self.height = height;


    }

    pub fn resize(&mut self, abs_x: u32, abs_y: u32, res: &mut Vec::<ChangePositionEvent<RectangleId>>)
    {   
        self.abs_x = abs_x;
        self.abs_y = abs_y;
        let mut delta_height = self.height;

        for element in &mut self.elements {
            match element {
                GuiElement::Rectangle(rectangle) => {
                    delta_height -= rectangle.height();

                    let rectangle_abs_x = abs_x;
                    let rectangle_abs_y = abs_y + delta_height;
                    rectangle.set_abs_pos(rectangle_abs_x, rectangle_abs_y);
                    res.push(rectangle.change_position_event());
                }
                GuiElement::VerticalLayout(vertical_layout) => {
                    delta_height -= vertical_layout.height();

                    let vertical_layout_abs_x = abs_x;
                    let vertical_layout_abs_y = abs_y + delta_height;
                    vertical_layout.resize(vertical_layout_abs_x, vertical_layout_abs_y, res);
                }
            }
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    fn is_inside(&self, x: u32, y: u32) -> bool {
        x >= self.abs_x && x < self.abs_x + self.width &&
        y >= self.abs_y && y < self.abs_y + self.height 
    }

    pub fn mouse_pressed(&mut self, abs_x: u32, abs_y: u32) -> (bool, Option<RectanglePressedEvent<RectangleId>>) {
        if !self.is_inside(abs_x, abs_y) {
            return (false, None);
        }

        for element in &mut self.elements {
            match element {
                GuiElement::Rectangle(rectangle) => {
                    let (consumed, event) = rectangle.mouse_pressed(abs_x, abs_y);
                    if consumed {
                        return (true, event);
                    }
                },
                GuiElement::VerticalLayout(vertical_layout) => {
                    let (consumed, event) = vertical_layout.mouse_pressed(abs_x, abs_y);
                    if consumed {
                        return (true, event);
                    }
                }
            }              
        }

        (false, None) 

    }

    pub fn mouse_released(&mut self, abs_x: u32, abs_y: u32) -> (bool, Option<RectanglePressedEvent<RectangleId>>) {
        if !self.is_inside(abs_x, abs_y) {
            return (false, None);
        }

        for element in &mut self.elements {
            match element {
                GuiElement::Rectangle(rectangle) => {
                    let (consumed, event) = rectangle.mouse_released(abs_x, abs_y);
                    if consumed {
                        return (true, event);
                    }
                },
                GuiElement::VerticalLayout(vertical_layout) => {
                    let (consumed, event) = vertical_layout.mouse_released(abs_x, abs_y);
                    if consumed {
                        return (true, event);
                    }
                }
            }              
        }

        (false, None) 
    }

}
