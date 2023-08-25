use crate::rpa_engine::rpa_automation::rpa_element::RPAElement;
use crate::rpa_engine::rpa_core::rect::MagicRect;

use windows::Win32::System::Variant::VARIANT;
use windows::Win32::UI::Accessibility::IAccessible;

use log::error;
use std::collections::HashMap;

pub struct WinMSAAElement {
    pub automation: IAccessible,
    pub element: IAccessible,
    pub variant: VARIANT,
}

impl RPAElement for WinMSAAElement {
    fn get_name(&self) -> String {
        unsafe {
            match &self.element.get_accName(self.variant.clone()) {
                Ok(name) => {
                    return name.to_string();
                }
                Err(e) => {
                    error!("get element name error:{e}");
                    return String::from("");
                }
            };
        }
    }
    fn get_classname(&self) -> String {
        return String::from("");
    }
    fn get_bounding_rectangle(&self) -> MagicRect {
        return MagicRect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        };
    }
    fn get_control_type(&self) -> String {
        return String::from("");
    }

    fn is_enabled(&self) -> bool {
        return bool::from(false);
    }
    fn is_focus(&self) -> bool {
        return bool::from(true);
    }
    fn is_edit(&self) -> bool {
        return bool::from(true);
    }
    fn get_checked(&self) -> bool {
        return bool::from(true);
    }
    fn set_check(&self) -> bool {
        return bool::from(true);
    }
    fn get_text(&self) -> String {
        return String::from("");
    }
    fn set_text(&self) -> String {
        return String::from("");
    }

    fn get_select(&self) -> Vec<String> {
        return vec![String::from("")];
    }
    fn set_select(&self) -> Vec<String> {
        return vec![String::from("")];
    }
    fn event_click(&self) -> bool {
        return bool::from(true);
    }
    fn get_attributes(&self) -> HashMap<String, String> {
        let mut map_attributes: HashMap<String, String> = HashMap::new();

        return map_attributes;
    }
    fn get_attribute(&self, attribute_name: &str) -> String {
        return String::from("");
    }
    fn set_attribute(&self, attribute_name: &str, attribute_value: &str) -> bool {
        return true;
    }

    fn get_parent(&self) -> Box<dyn RPAElement> {
        let win_uia_element = WinMSAAElement {
            automation: self.automation.clone(),
            element: self.element.clone(),
            variant: self.variant.clone(),
        };
        return Box::new(win_uia_element);
    }
    fn get_children(&self) -> Vec<Box<dyn RPAElement>> {
        let mut children: Vec<Box<dyn RPAElement>> = Vec::new();
        let mut win_elemet = WinMSAAElement {
            automation: self.automation.clone(),
            element: self.element.clone(),
            variant: self.variant.clone(),
        };
        let mut box_win_element = Box::new(win_elemet);
        children.push(box_win_element);

        win_elemet = WinMSAAElement {
            automation: self.automation.clone(),
            element: self.element.clone(),
            variant: self.variant.clone(),
        };
        box_win_element = Box::new(win_elemet);
        children.push(box_win_element);

        return children;
    }
    fn get_descendants(&self) -> Vec<Box<dyn RPAElement>> {
        let mut children: Vec<Box<dyn RPAElement>> = Vec::new();
        let mut win_elemet = WinMSAAElement {
            automation: self.automation.clone(),
            element: self.element.clone(),
            variant: self.variant.clone(),
        };
        let mut box_win_element = Box::new(win_elemet);
        children.push(box_win_element);

        win_elemet = WinMSAAElement {
            automation: self.automation.clone(),
            element: self.element.clone(),
            variant: self.variant.clone(),
        };
        box_win_element = Box::new(win_elemet);
        children.push(box_win_element);

        return children;
    }
}
