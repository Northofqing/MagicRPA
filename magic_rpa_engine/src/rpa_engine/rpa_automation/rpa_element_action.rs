use crate::rpa_engine::rpa_automation::rpa_element::Element;
pub trait ElementAction : Element {
     fn click();
     fn input();
     fn mouse_move();
}
