//! Arranges gui elements horizontally

use super::gui_element::GuiElementInterface;
use super::gui_element::MouseEventResult;

pub struct HorizontalLayout
{
    // cache sizes
    abs_x: u32,
    abs_y: u32,
    width: u32,
    height: u32,

    active: bool,
}

impl HorizontalLayout
{
    pub fn new() -> Self 
    {
        Self {

            abs_x: 0,
            abs_y: 0,
            width: 0,
            height: 0,

            active: false,
        }
    }

    pub fn assemble<'a, ElementId>(&'a mut self, 
        elements: &'a mut [&'a mut dyn GuiElementInterface<ElementId>]
    ) -> HorizontalLayoutAssembled<'a, ElementId> 
    {
        HorizontalLayoutAssembled::new(self, elements)
    }

    fn is_inside(&self, x: u32, y: u32) -> bool {
        x >= self.abs_x && x < self.abs_x + self.width &&
        y >= self.abs_y && y < self.abs_y + self.height 
    }
}

pub struct HorizontalLayoutAssembled<'a, ElementId>
{
    layout: &'a mut HorizontalLayout,
    elements: &'a mut [&'a mut (dyn GuiElementInterface<ElementId>)], 
}

impl<'a, ElementId> HorizontalLayoutAssembled<'a, ElementId> {
    fn new(layout: &'a mut HorizontalLayout, elements: &'a mut [&'a mut dyn GuiElementInterface<ElementId>]) -> Self 
    { 
        let mut obj = Self { layout, elements };
        obj.calculate_element_size();
        obj
    }

    fn calculate_element_size(&mut self) {

        let mut width =  0;
        let mut height = 0;

        for element in self.elements.iter() {
            width = width + element.width();
            height = height.max(element.height());
        }

        self.layout.width = width;
        self.layout.height = height;
    }
}

impl<'a, ElementId>  GuiElementInterface<ElementId> for HorizontalLayoutAssembled<'a, ElementId>  
{
    fn width(&self) -> u32 {
        self.layout.width
    }

    fn height(&self) -> u32 {
        self.layout.height
    }

    fn resize(&mut self, abs_x: u32, abs_y: u32) {

        // self.calculate_element_size();

        self.layout.abs_x = abs_x;
        self.layout.abs_y = abs_y;
        let mut delta_width = 0;

        for element in self.elements.iter_mut() {
 
            let element_abs_x = abs_x + delta_width;
            let element_abs_y = abs_y + self.layout.height/2 - element.height()/2;
            element.resize(element_abs_x, element_abs_y);

            delta_width += element.width();
        }
    }

    fn mouse_event(&mut self, abs_x: u32, abs_y: u32, pressed: bool, res: &mut MouseEventResult<ElementId>) {
        if !self.layout.is_inside(abs_x, abs_y) && !self.layout.active {
            return;
        }

        for element in self.elements.iter_mut() {
            element.mouse_event(abs_x, abs_y, pressed, res);   
        }

        self.layout.active = res.consumed;
    }
}
