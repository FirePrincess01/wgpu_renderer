//! Unit tests

use crate::gui;

use super::*;

#[derive(Debug, PartialEq, Copy, Clone)]
enum RectangleId {
    PerformanceGraph,
    SwitchViewPoint,
    SwitchTexture,
    Fps,
    Menu,
}

struct TestGui {
    pub gui: Gui<RectangleId, NoId, RectangleId>,
}

impl TestGui {
    fn new() -> Self {
        let vertical_layout = VerticalLayout::new(vec![
            GuiElement::Rectangle(Rectangle::new_btn(
                RectangleId::Menu,
                RectangleId::Menu,
                40,
                30,
                5,
            )),
            GuiElement::Rectangle(Rectangle::new_btn(
                RectangleId::Fps,
                RectangleId::Fps,
                40,
                30,
                5,
            )),
            GuiElement::Rectangle(Rectangle::new_btn(
                RectangleId::SwitchTexture,
                RectangleId::SwitchTexture,
                40,
                30,
                5,
            )),
            GuiElement::Rectangle(Rectangle::new_btn(
                RectangleId::SwitchViewPoint,
                RectangleId::SwitchViewPoint,
                40,
                30,
                5,
            )),
            GuiElement::Rectangle(Rectangle::new_btn(
                RectangleId::PerformanceGraph,
                RectangleId::PerformanceGraph,
                40,
                30,
                5,
            )),
        ]);

        let width = 800;
        let height = 600;
        let gui = Gui::new(
            width,
            height,
            vec![AlignedElement::new(
                Alignment::BottomRight,
                10,
                10,
                GuiElement::VerticalLayout(vertical_layout),
            )],
        );

        Self { gui }
    }
}

#[test]
fn mouse_event() -> Result<(), String> {
    let mut gui = TestGui::new();

    // left bottom egdge of the 4 th button
    let res = gui
        .gui
        .mouse_event(gui::MouseEvent::Moved { x: 800 - 10, y: 10 });
    assert_eq!(res.consumed, false);
    assert_eq!(res.released_event.is_none(), true);

    let res = gui.gui.mouse_event(gui::MouseEvent::Pressed);
    assert_eq!(res.consumed, false);
    assert_eq!(res.released_event.is_none(), true);

    let res = gui.gui.mouse_event(gui::MouseEvent::Released);
    assert_eq!(res.consumed, false);
    assert_eq!(res.released_event.is_none(), true);

    // left bottom boarder of the 4 th button
    let res = gui
        .gui
        .mouse_event(gui::MouseEvent::Moved { x: 800 - 14, y: 14 });
    assert_eq!(res.consumed, false);
    assert_eq!(res.released_event.is_none(), true);

    let res = gui.gui.mouse_event(gui::MouseEvent::Pressed);
    assert_eq!(res.consumed, false);
    assert_eq!(res.released_event.is_none(), true);

    let res = gui.gui.mouse_event(gui::MouseEvent::Released);
    assert_eq!(res.consumed, false);
    assert_eq!(res.released_event.is_none(), true);

    // left bottom of the 4 th button
    let res = gui
        .gui
        .mouse_event(gui::MouseEvent::Moved { x: 800 - 15, y: 15 });
    assert_eq!(res.consumed, true);
    assert_eq!(res.released_event.is_none(), true);

    let res = gui.gui.mouse_event(gui::MouseEvent::Pressed);
    assert_eq!(res.consumed, true);
    assert_eq!(res.released_event.is_none(), true);

    let res = gui.gui.mouse_event(gui::MouseEvent::Released);
    assert_eq!(res.consumed, true);
    assert_eq!(res.released_event.is_some(), true);
    match res.released_event {
        Some(event) => {
            assert_eq!(event, RectangleId::PerformanceGraph);
        }
        None => {}
    }

    // right top of the 4 th button
    let res = gui
        .gui
        .mouse_event(gui::MouseEvent::Moved { x: 800 - 55, y: 45 });
    assert_eq!(res.consumed, true);
    assert_eq!(res.released_event.is_none(), true);

    let res = gui.gui.mouse_event(gui::MouseEvent::Pressed);
    assert_eq!(res.consumed, true);
    assert_eq!(res.released_event.is_none(), true);

    let res = gui.gui.mouse_event(gui::MouseEvent::Released);
    assert_eq!(res.consumed, true);
    assert_eq!(res.released_event.is_some(), true);
    match res.released_event {
        Some(event) => {
            assert_eq!(event, RectangleId::PerformanceGraph);
        }
        None => {}
    }

    // right top boarder of the 4 th button
    let res = gui
        .gui
        .mouse_event(gui::MouseEvent::Moved { x: 800 - 60, y: 50 });
    assert_eq!(res.consumed, false);
    assert_eq!(res.released_event.is_none(), true);

    let res = gui.gui.mouse_event(gui::MouseEvent::Pressed);
    assert_eq!(res.consumed, false);
    assert_eq!(res.released_event.is_none(), true);

    let res = gui.gui.mouse_event(gui::MouseEvent::Released);
    assert_eq!(res.consumed, false);
    assert_eq!(res.released_event.is_none(), true);

    Ok(())
}

#[test]
fn call_resize() -> Result<(), String> {
    let mut gui = TestGui::new();

    let gui_width = 700;
    let gui_height = 700;
    let res = gui.gui.resize(gui_width, gui_height);
    assert_eq!(res.len(), 5);

    for event in &res {
        match event.element_id {
            RectangleId::PerformanceGraph => {
                assert_eq!(event.x, gui_width - 55);
                assert_eq!(event.y, 15);
            }
            RectangleId::SwitchViewPoint => {
                assert_eq!(event.x, gui_width - 55);
                assert_eq!(event.y, 15 + 1 * 40);
            }
            RectangleId::SwitchTexture => {
                assert_eq!(event.x, gui_width - 55);
                assert_eq!(event.y, 15 + 2 * 40);
            }
            RectangleId::Fps => {
                assert_eq!(event.x, gui_width - 55);
                assert_eq!(event.y, 15 + 3 * 40);
            }
            RectangleId::Menu => {
                assert_eq!(event.x, gui_width - 55);
                assert_eq!(event.y, 15 + 4 * 40);
            }
        }
    }

    Ok(())
}
