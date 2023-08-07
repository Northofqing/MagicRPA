use crate::rpa_engine::rpa_automation::rpa_element::RPAElement;
pub trait RPAElementAction : RPAElement {
     fn click();
     fn input();
     fn mouse_move();
}
