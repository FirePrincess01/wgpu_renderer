//! Unit tests

use crate::gui3::gui_element::ElementState;

use super::{*, gui_element::GuiElementInterface};

struct GuiLabel<ElementId> 
where ElementId: Copy
{
    _mesh: u32,
    mesh_abs_x: u32,
    mesh_abs_y: u32,

    rect: Rectangle<ElementId>,
}

impl<ElementId> GuiLabel<ElementId>  
where ElementId: Copy
{
    pub fn new(element_id: ElementId) -> Self {
        let btn_width = 40;
        let btn_height = 30;
        let btn_boarder = 5;

        let mesh = 10;

        let rect: Rectangle<ElementId> = Rectangle::new(element_id, btn_width, btn_height, btn_boarder);
        
        Self {
            _mesh: mesh,
            mesh_abs_x: 0,
            mesh_abs_y: 0,

            rect,
        }
    }
}

impl<ElementId> GuiElementInterface<ElementId> for GuiLabel<ElementId> 
where ElementId: Copy
{
    fn width(&self) -> u32 {
        self.rect.width()
    }

    fn height(&self) -> u32 {
        self.rect.height()
    }

    fn resize(&mut self, abs_x: u32, abs_y: u32) {
        self.mesh_abs_x = abs_x + self.rect.boarder();
        self.mesh_abs_y = abs_y + self.rect.boarder();

        self.rect.resize(abs_x, abs_y);
    }

    fn mouse_event(&mut self, abs_x: u32, abs_y: u32, pressed: bool, res: &mut MouseEventResult<ElementId>) {
        self.rect.mouse_event(abs_x, abs_y, pressed, res)
    }


}


#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum RectangleId{
    Menu, 
    Fps,
    SwitchTexture,
    SwitchViewPoint,
    PerformanceGraph,
} 


struct TestGui {
    pub button_menu: GuiLabel<RectangleId>,
    pub button_fps: GuiLabel<RectangleId>,
    pub button_switch_texture: GuiLabel<RectangleId>,
    pub button_switch_view_point: GuiLabel<RectangleId>,
    pub button_performance_graph: GuiLabel<RectangleId>,

    pub layout_vertical: VerticalLayout,
    pub layout_aligned: AlignedElement,
}

impl TestGui {
    fn new() -> Self {

        let button_menu = GuiLabel::new(RectangleId::Menu);
        let button_fps = GuiLabel::new(RectangleId::Fps);
        let button_switch_texture = GuiLabel::new(RectangleId::SwitchTexture);
        let button_switch_view_point = GuiLabel::new(RectangleId::SwitchViewPoint);
        let button_performance_graph = GuiLabel::new(RectangleId::PerformanceGraph);

        let layout_vertical = VerticalLayout::new();

        let layout_aligned = AlignedElement::new(Alignment::BottomRight, 10, 10);

        let mut obj = Self {
            button_menu,
            button_fps,
            button_switch_texture,
            button_switch_view_point,
            button_performance_graph,

            layout_vertical,
            layout_aligned,
        };

        let width = 800;
        let height = 600;
        obj.resize(width, height);

        obj
    }

    fn call_gui_function<F>(&mut self, func: F)
    where F: FnOnce(&mut AlignedElementAssembled<'_, RectangleId>) -> ()
    {
        let mut elements: [&mut dyn GuiElementInterface<RectangleId>; 5]= [
            &mut self.button_menu,
            &mut self.button_fps,
            &mut self.button_switch_texture,
            &mut self.button_switch_view_point,
            &mut self.button_performance_graph, 
            ];

        let mut layout_vertical_assembled = self.layout_vertical.assemble(&mut elements);
        let mut layout_aligned_assembled: aligned_element::AlignedElementAssembled<'_, RectangleId> = self.layout_aligned.assemble(&mut layout_vertical_assembled);

        func(&mut layout_aligned_assembled);
    }

    pub fn resize(&mut self, gui_width: u32, gui_height: u32) 
    {
        let func = move | 
            layout_aligned_assembled: &mut AlignedElementAssembled<'_, RectangleId> | 
            layout_aligned_assembled.resize(gui_width, gui_height);
        self.call_gui_function(func);

    }

    pub fn mouse_event(&mut self, mouse_event: MouseEvent) -> MouseEventResult<RectangleId> {

        let mut res = MouseEventResult::new();

        let func = | 
            layout_aligned_assembled: &mut AlignedElementAssembled<'_, RectangleId> | 
            {
                res = layout_aligned_assembled.mouse_event(mouse_event);
            };
            
        self.call_gui_function(func);

        res
    }
}

#[test]
fn mouse_event() -> Result<(), String> {

    let mut gui = TestGui::new();

    // left bottom egdge of the 4 th button
    let res = gui.mouse_event(MouseEvent::Moved { x: 800 - 10, y: 10 });
    assert_eq!(res.consumed, false);
    assert_eq!(res.mouse_events[0].is_none(), true);

    let res= gui.mouse_event(MouseEvent::Pressed);
    assert_eq!(res.consumed, false);
    assert_eq!(res.mouse_events[0].is_none(), true);

    let res= gui.mouse_event(MouseEvent::Released);
    assert_eq!(res.consumed, false);
    assert_eq!(res.mouse_events[0].is_none(), true);

    // left bottom boarder of the 4 th button
    let res = gui.mouse_event(MouseEvent::Moved { x: 800 - 14, y: 14 });
    assert_eq!(res.consumed, false);
    assert_eq!(res.mouse_events[0].is_none(), true);

    let res= gui.mouse_event(MouseEvent::Pressed);
    assert_eq!(res.consumed, false);
    assert_eq!(res.mouse_events[0].is_none(), true);

    let res = gui.mouse_event(MouseEvent::Released);
    assert_eq!(res.consumed, false);
    assert_eq!(res.mouse_events[0].is_none(), true);

    // left bottom of the 4 th button
    let res = gui.mouse_event(MouseEvent::Moved { x: 800 - 15, y: 15 });
    assert_eq!(res.consumed, true);
    assert_eq!(res.mouse_events[0].is_none(), true);

    let res= gui.mouse_event(MouseEvent::Pressed);
    assert_eq!(res.consumed, true);
    assert_eq!(res.mouse_events[0].is_some(), true);

    let res = gui.mouse_event(MouseEvent::Released);
    assert_eq!(res.consumed, true);
    assert_eq!(res.mouse_events[0].is_some(), true);
    match &res.mouse_events[0] {
        Some(event) => {
            assert_eq!(event.element_id, RectangleId::PerformanceGraph);
            assert_eq!(event.state, ElementState::Released);
        },
        None => {},
    }

    // right top of the 4 th button
    let res = gui.mouse_event(MouseEvent::Moved { x: 800 - 55, y: 45 });
    assert_eq!(res.consumed, true);
    assert_eq!(res.mouse_events[0].is_none(), true);

    let res = gui.mouse_event(MouseEvent::Pressed);
    assert_eq!(res.consumed, true);
    assert_eq!(res.mouse_events[0].is_some(), true);

    let res = gui.mouse_event(MouseEvent::Released);
    assert_eq!(res.consumed, true);
    assert_eq!(res.mouse_events[0].is_some(), true);
    match &res.mouse_events[0] {
        Some(event) => {
            assert_eq!(event.element_id, RectangleId::PerformanceGraph);
            assert_eq!(event.state, ElementState::Released);
        },
        None => {},
    }

    // right top boarder of the 4 th button
    let res = gui.mouse_event(MouseEvent::Moved { x: 800 - 60, y: 50 });
    assert_eq!(res.consumed, false);
    assert_eq!(res.mouse_events[0].is_none(), true);

    let res = gui.mouse_event(MouseEvent::Pressed);
    assert_eq!(res.consumed, false);
    assert_eq!(res.mouse_events[0].is_none(), true);

    let res = gui.mouse_event(MouseEvent::Released);
    assert_eq!(res.consumed, false);
    assert_eq!(res.mouse_events[0].is_none(), true);

    Ok(())
}

#[test]
fn call_resize() -> Result<(), String> {
    let mut gui = TestGui::new();

    let gui_width = 700;
    let gui_height = 700;
    gui.resize(gui_width, gui_height);
    
    assert_eq!(gui.button_menu.mesh_abs_x, gui_width - 55);
    assert_eq!(gui.button_menu.mesh_abs_y, 15 + 4*40);

    assert_eq!(gui.button_fps.mesh_abs_x, gui_width - 55);
    assert_eq!(gui.button_fps.mesh_abs_y, 15 + 3*40);

    assert_eq!(gui.button_switch_texture.mesh_abs_x, gui_width - 55);
    assert_eq!(gui.button_switch_texture.mesh_abs_y, 15 + 2*40);

    assert_eq!(gui.button_switch_view_point.mesh_abs_x, gui_width - 55);
    assert_eq!(gui.button_switch_view_point.mesh_abs_y, 15 + 1*40);
    
    assert_eq!(gui.button_performance_graph.mesh_abs_x, gui_width - 55);
    assert_eq!(gui.button_performance_graph.mesh_abs_y, 15);

    Ok(())
}
