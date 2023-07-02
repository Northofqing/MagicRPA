use std::error::Error;
use uiautomation::UITreeWalker;
use uiautomation::UIAutomation;

use uiautomation::UIElement;
pub struct WinUIAElement {
    element: UIElement,
    automation: UIAutomation,
}
impl WinUIAElement {
    pub fn new(element: UIElement,automation: UIAutomation) -> Self {
        WinUIAElement { element ,automation}
    }
    pub fn get_children(&self){
        let walker =&self.automation.get_raw_view_walker().unwrap();
        let child = walker.get_first_child(&self.element);
    }
    pub fn get_name(&self)   {
        let result = &self.element.get_name().unwrap();
        print!("element name: {} \r\n",result); 
    }
    pub fn get_classname(&self)  {
        let result = &self.element.get_classname().unwrap();
        print!("element classname: {} \r\n",result); 
    }
    pub fn get_bounding_rectangle(&self)   {
        let result = &self.element.get_bounding_rectangle().unwrap();
        print!("element rectangle: {} \r\n",result); 
    }
    pub fn is_enable(&self) -> bool {
        return true;
    }
    pub fn is_focus(&self) -> bool {
        return self.element.try_focus();
    }
    pub fn is_edit(&self) -> bool {
        return true;
    }
    pub fn is_checked(&self) -> bool {
        return true;
    }
    pub fn set_check(&self) -> bool {
        return true;
    }
    
    // pub fn get_text(&self) -> &Result<String, uiautomation::Error>{}
    // pub fn set_text(&self) -> &Result<String, uiautomation::Error>{}
    // pub fn get_index(&self) -> i32 {
    //     return  1;
    // }
   
    // pub fn get_select(&self) -> Vect<&str> {

    // }
    // pub fn set_select(&self) -> Vect<&str> {

    // }
    // pub fn click(&self) -> bool {
    //     &self.element.click();
    //     return true;
    // }
    // pub fn get_attribute(&self, name: &str) -> Vect<&str> {
    //     return &self.element.get_attribute();
    // }
}
