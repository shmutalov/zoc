use std::sync::mpsc::{Sender};
use std::collections::{HashMap};
use glutin::{self, Event, MouseButton, VirtualKeyCode};
use glutin::ElementState::{Released};
use core::unit::{UnitTypeId};
use core::db::{Db};
use types::{Time, ScreenPos};
use screen::{Screen, ScreenCommand, EventStatus};
use context::{Context};
use gui::{ButtonManager, Button, ButtonId, is_tap, basic_text_size};

#[derive(Clone, Debug)]
pub struct ReinforcementsPopup {
    game_screen_tx: Sender<Option<UnitTypeId>>,
    button_manager: ButtonManager,
    button_ids: HashMap<ButtonId, UnitTypeId>,
}

impl ReinforcementsPopup {
    pub fn new(
        db: &Db,
        context: &mut Context,
        pos: ScreenPos,
        tx: Sender<Option<UnitTypeId>>,
    ) -> ReinforcementsPopup {
        let mut button_manager = ButtonManager::new();
        let mut button_ids = HashMap::new();
        let mut pos = pos;
        let text_size = basic_text_size(context);
        pos.v.y -= text_size as i32;
        let vstep = (text_size * 0.8) as i32;
        for (i, unit_type) in db.unit_types().iter().enumerate() {
            let unit_type_id = UnitTypeId{id: i as i32};
            let text = &format!("{} ({})", unit_type.name, unit_type.cost);
            let button_id = button_manager.add_button(
                Button::new(context, text, pos));
            button_ids.insert(button_id, unit_type_id);
            pos.v.y -= vstep;
        }
        ReinforcementsPopup {
            game_screen_tx: tx,
            button_manager: button_manager,
            button_ids: button_ids,
        }
    }

    fn handle_event_lmb_release(&mut self, context: &mut Context) {
        if !is_tap(context) {
            return;
        }
        if let Some(button_id) = self.button_manager.get_clicked_button_id(context) {
            self.handle_event_button_press(context, button_id);
        } else {
            context.add_command(ScreenCommand::PopPopup);
        }
    }

    fn handle_event_button_press(
        &mut self,
        context: &mut Context,
        button_id: ButtonId
    ) {
        if let Some(&unit_type_id) = self.button_ids.get(&button_id) {
            self.game_screen_tx.send(Some(unit_type_id)).unwrap();
            context.add_command(ScreenCommand::PopPopup);
            return;
        } else {
            panic!("Bad button id: {}", button_id.id);
        }
    }

    fn handle_event_key_press(&mut self, context: &mut Context, key: VirtualKeyCode) {
        match key {
            glutin::VirtualKeyCode::Q
                | glutin::VirtualKeyCode::Escape =>
            {
                context.add_command(ScreenCommand::PopPopup);
            },
            _ => {},
        }
    }
}

impl Screen for ReinforcementsPopup {
    fn tick(&mut self, context: &mut Context, _: Time) {
        context.data.basic_color = [0.0, 0.0, 0.0, 1.0];
        self.button_manager.draw(context);
    }

    fn handle_event(
        &mut self,
        context: &mut Context,
        event: &glutin::Event,
    ) -> EventStatus {
        let mut event_status = EventStatus::Handled;
        match *event {
            Event::MouseMoved(..) => {},
            Event::MouseInput(Released, MouseButton::Left) => {
                self.handle_event_lmb_release(context);
            },
            Event::Touch(glutin::Touch{phase, ..}) => {
                if phase == glutin::TouchPhase::Ended {
                    self.handle_event_lmb_release(context);
                }
            },
            glutin::Event::KeyboardInput(Released, _, Some(key)) => {
                self.handle_event_key_press(context, key);
            },
            _ => event_status = EventStatus::NotHandled,
        }
        event_status
    }
}
