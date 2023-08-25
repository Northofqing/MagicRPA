use crate::rpa_engine::rpa_core::rect::MagicRect;

use uiautomation::types::UIProperty; 
use uiautomation::UIAutomation;
use uiautomation::UIElement; 

use crate::rpa_engine::rpa_automation::rpa_element::RPAElement;

use log::error;
use std::collections::HashMap;
pub struct WinUIAElement {
    pub automation: UIAutomation,
    pub element: UIElement,
}

impl RPAElement for WinUIAElement {
    fn get_name(&self) -> String {
        match &self.element.get_name() {
            Ok(name) => {
                return name.clone();
            }
            Err(e) => {
                error!("get element name error:{e}");
                return String::from("");
            }
        };
    }
    fn get_classname(&self) -> String {
        match &self.element.get_classname() {
            Ok(classname) => {
                return classname.clone();
            }
            Err(e) => {
                error!("get element classname error:{e}");
                return String::from("");
            }
        };
    }
    fn get_bounding_rectangle(&self) -> MagicRect {
        match &self.element.get_bounding_rectangle() {
            Ok(rect) => {
                return MagicRect {
                    x: rect.get_left(),
                    y: rect.get_top(),
                    width: rect.get_width(),
                    height: rect.get_height(),
                };
            }
            Err(e) => {
                error!("get element bounding rectangle error:{e}");
                return MagicRect {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                };
            }
        };
    }
    fn get_control_type(&self) -> String {
        match &self.element.get_control_type() {
            Ok(controltype) => {
                return format!("{:?}", controltype);
            }
            Err(_) => {
                return String::from("");
            }
        };
    }

    fn is_enabled(&self) -> bool {
        match &self.element.is_enabled() {
            Ok(enable) => {
                return enable.clone();
            }
            Err(_) => {
                return bool::from(false);
            }
        }
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

        let vec_attributes: Vec<UIProperty> = vec![
            UIProperty::RuntimeId,
            UIProperty::BoundingRectangle,
            UIProperty::ProcessId,
            UIProperty::ControlType,
            UIProperty::LocalizedControlType,
            UIProperty::Name,
            UIProperty::AcceleratorKey,
            UIProperty::AccessKey,
            UIProperty::HasKeyboardFocus,
            UIProperty::IsKeyboardFocusable,
            UIProperty::IsEnabled,
            UIProperty::AutomationId,
            UIProperty::ClassName,
            UIProperty::HelpText,
            UIProperty::ClickablePoint,
            UIProperty::Culture,
            UIProperty::IsControlElement,
            UIProperty::IsContentElement,
            UIProperty::LabeledBy,
            UIProperty::IsPassword,
            UIProperty::NativeWindowHandle,
            UIProperty::ItemType,
            UIProperty::IsOffscreen,
            UIProperty::Orientation,
            UIProperty::FrameworkId,
            UIProperty::IsRequiredForForm,
            UIProperty::ItemStatus,
            UIProperty::IsDockPatternAvailable,
            UIProperty::IsExpandCollapsePatternAvailable,
            UIProperty::IsGridItemPatternAvailable,
            UIProperty::IsGridPatternAvailable,
            UIProperty::IsInvokePatternAvailable,
            UIProperty::IsMultipleViewPatternAvailable,
            UIProperty::IsRangeValuePatternAvailable,
            UIProperty::IsScrollPatternAvailable,
            UIProperty::IsScrollItemPatternAvailable,
            UIProperty::IsSelectionItemPatternAvailable,
            UIProperty::IsSelectionPatternAvailable,
            UIProperty::IsTablePatternAvailable,
            UIProperty::IsTableItemPatternAvailable,
            UIProperty::IsTextPatternAvailable,
            UIProperty::IsTogglePatternAvailable,
            UIProperty::IsTransformPatternAvailable,
            UIProperty::IsValuePatternAvailable,
            UIProperty::IsWindowPatternAvailable,
            UIProperty::ValueValue,
            UIProperty::ValueIsReadOnly,
            UIProperty::RangeValueValue,
            UIProperty::RangeValueIsReadOnly,
            UIProperty::RangeValueMinimum,
            UIProperty::RangeValueMaximum,
            UIProperty::RangeValueLargeChange,
            UIProperty::RangeValueSmallChange,
            UIProperty::ScrollHorizontalScrollPercent,
            UIProperty::ScrollHorizontalViewSize,
            UIProperty::ScrollVerticalScrollPercent,
            UIProperty::ScrollVerticalViewSize,
            UIProperty::ScrollHorizontallyScrollable,
            UIProperty::ScrollVerticallyScrollable,
            UIProperty::SelectionSelection,
            UIProperty::SelectionCanSelectMultiple,
            UIProperty::SelectionIsSelectionRequired,
            UIProperty::GridRowCount,
            UIProperty::GridColumnCount,
            UIProperty::GridItemRow,
            UIProperty::GridItemColumn,
            UIProperty::GridItemRowSpan,
            UIProperty::GridItemColumnSpan,
            UIProperty::GridItemContainingGrid,
            UIProperty::DockDockPosition,
            UIProperty::ExpandCollapseExpandCollapseState,
            UIProperty::MultipleViewCurrentView,
            UIProperty::MultipleViewSupportedViews,
            UIProperty::WindowCanMaximize,
            UIProperty::WindowCanMinimize,
            UIProperty::WindowWindowVisualState,
            UIProperty::WindowWindowInteractionState,
            UIProperty::WindowIsModal,
            UIProperty::WindowIsTopmost,
            UIProperty::SelectionItemIsSelected,
            UIProperty::SelectionItemSelectionContainer,
            UIProperty::TableRowHeaders,
            UIProperty::TableColumnHeaders,
            UIProperty::TableRowOrColumnMajor,
            UIProperty::TableItemRowHeaderItems,
            UIProperty::TableItemColumnHeaderItems,
            UIProperty::ToggleToggleState,
            UIProperty::TransformCanMove,
            UIProperty::TransformCanResize,
            UIProperty::TransformCanRotate,
            UIProperty::IsLegacyIAccessiblePatternAvailable,
            UIProperty::LegacyIAccessibleChildId,
            UIProperty::LegacyIAccessibleName,
            UIProperty::LegacyIAccessibleValue,
            UIProperty::LegacyIAccessibleDescription,
            UIProperty::LegacyIAccessibleRole,
            UIProperty::LegacyIAccessibleState,
            UIProperty::LegacyIAccessibleHelp,
            UIProperty::LegacyIAccessibleKeyboardShortcut,
            UIProperty::LegacyIAccessibleSelection,
            UIProperty::LegacyIAccessibleDefaultAction,
            UIProperty::AriaRole,
            UIProperty::AriaProperties,
            UIProperty::IsDataValidForForm,
            UIProperty::ControllerFor,
            UIProperty::DescribedBy,
            UIProperty::FlowsTo,
            UIProperty::ProviderDescription,
            UIProperty::IsItemContainerPatternAvailable,
            UIProperty::IsVirtualizedItemPatternAvailable,
            UIProperty::IsSynchronizedInputPatternAvailable,
            UIProperty::OptimizeForVisualContent,
            UIProperty::IsObjectModelPatternAvailable,
            UIProperty::AnnotationAnnotationTypeId,
            UIProperty::AnnotationAnnotationTypeName,
            UIProperty::AnnotationAuthor,
            UIProperty::AnnotationDateTime,
            UIProperty::AnnotationTarget,
            UIProperty::IsAnnotationPatternAvailable,
            UIProperty::IsTextPattern2Available,
            UIProperty::StylesStyleId,
            UIProperty::StylesStyleName,
            UIProperty::StylesFillColor,
            UIProperty::StylesFillPatternStyle,
            UIProperty::StylesShape,
            UIProperty::StylesFillPatternColor,
            UIProperty::StylesExtendedProperties,
            UIProperty::IsStylesPatternAvailable,
            UIProperty::IsSpreadsheetPatternAvailable,
            UIProperty::SpreadsheetItemFormula,
            UIProperty::SpreadsheetItemAnnotationObjects,
            UIProperty::SpreadsheetItemAnnotationTypes,
            UIProperty::IsSpreadsheetItemPatternAvailable,
            UIProperty::Transform2CanZoom,
            UIProperty::IsTransformPattern2Available,
            UIProperty::LiveSetting,
            UIProperty::IsTextChildPatternAvailable,
            UIProperty::IsDragPatternAvailable,
            UIProperty::DragIsGrabbed,
            UIProperty::DragDropEffect,
            UIProperty::DragDropEffects,
            UIProperty::IsDropTargetPatternAvailable,
            UIProperty::DropTargetDropTargetEffect,
            UIProperty::DropTargetDropTargetEffects,
            UIProperty::DragGrabbedItems,
            UIProperty::Transform2ZoomLevel,
            UIProperty::Transform2ZoomMinimum,
            UIProperty::Transform2ZoomMaximum,
            UIProperty::FlowsFrom,
            UIProperty::IsTextEditPatternAvailable,
            UIProperty::IsPeripheral,
            UIProperty::IsCustomNavigationPatternAvailable,
            UIProperty::PositionInSet,
            UIProperty::SizeOfSet,
            UIProperty::Level,
            UIProperty::AnnotationTypes,
            UIProperty::AnnotationObjects,
            UIProperty::LandmarkType,
            UIProperty::LocalizedLandmarkType,
            UIProperty::FullDescription,
            UIProperty::FillColor,
            UIProperty::OutlineColor,
            UIProperty::FillType,
            UIProperty::VisualEffects,
            UIProperty::OutlineThickness,
            UIProperty::CenterPoint,
            UIProperty::Rotation,
            UIProperty::Size,
            UIProperty::IsSelectionPattern2Available,
            UIProperty::Selection2FirstSelectedItem,
            UIProperty::Selection2LastSelectedItem,
            UIProperty::Selection2CurrentSelectedItem,
            UIProperty::Selection2ItemCount,
            UIProperty::HeadingLevel,
            UIProperty::IsDialog,
        ];

        for _item in vec_attributes {
            match self.element.get_property_value(_item) {
                Ok(_value) => {
                    if _value.is_string() && !_value.is_null() {
                        let v = _value.get_string().unwrap();
                        if v != "" {
                            map_attributes
                                .insert(format!("{:?}", _item), _value.get_string().unwrap());
                        }
                    }
                }
                Err(_) => {}
            }
        }
        return map_attributes;
    }
    fn get_attribute(&self, attribute_name: &str) -> String {
        return String::from("");
    }
    fn set_attribute(&self, attribute_name: &str, attribute_value: &str) -> bool {
        return true;
    }

    fn get_parent(&self) -> Box<dyn RPAElement> {
        let walker = self.automation.get_raw_view_walker().unwrap();
        let parent = match walker.get_parent(&self.element) {
            Ok(element) => element,
            Err(error) => panic!("not found element: {:?}", error),
        };
        let win_uia_element = WinUIAElement {
            automation: self.automation.clone(),
            element: parent.clone(),
        };
        return Box::new(win_uia_element);
    }
    fn get_children(&self) -> Vec<Box<dyn RPAElement>> {
        let walker = self.automation.get_raw_view_walker().unwrap();
        let mut child = match walker.get_first_child(&self.element) {
            Ok(element) => element,
            Err(_) => {
                let result: Vec<Box<dyn RPAElement>> = Vec::new();
                return result;
            }
        };
        match Some(child) {
            Some(ele) => {
                child = ele;
            }
            None => {
                let _ = walker.get_next_sibling(&self.element);
                child = match walker.get_first_child(&self.element) {
                    Ok(element) => element,
                    Err(_) => {
                        let result: Vec<Box<dyn RPAElement>> = Vec::new();
                        return result;
                    }
                }
            }
        }

        let mut children: Vec<Box<dyn RPAElement>> = Vec::new();
        let mut win_elemet = WinUIAElement {
            automation: self.automation.clone(),
            element: child.clone(),
        };
        let mut box_win_element = Box::new(win_elemet);
        children.push(box_win_element);
        loop {
            child = match walker.get_next_sibling(&child) {
                Ok(element) => element,
                Err(_) => break,
            };

            win_elemet = WinUIAElement {
                automation: self.automation.clone(),
                element: child.clone(),
            };
            box_win_element = Box::new(win_elemet);
            children.push(box_win_element);
        }
        return children;
    }
    fn get_descendants(&self) -> Vec<Box<dyn RPAElement>> {
        let mut win_elements = Vec::new();
        let automation = UIAutomation::new();
        let win_element = WinUIAElement {
            automation: self.automation.clone(),
            element: automation.unwrap().get_root_element().unwrap(),
        };
        return win_elements;
    }
    // pub fn new(element: UIElement) -> Self {
    //     WinUIAElement {
    //         element,
    //         automation: UIAutomation::new().unwrap(),
    //     }
    // }

    // pub fn element_from_handle(&mut self, hwnd: uiautomation::types::Handle) ->  WinUIAElement {
    //     let resut = self.automation.element_from_handle(hwnd);
    //     self.element = match resut {
    //         Ok(element) => element,
    //         Err(error) => panic!("not found element: {:?}", error),
    //     };
    //     let win_uia_element = WinUIAElement::new(self.element.clone());
    //     return win_uia_element;
    // }
    // pub fn trace_point(&mut self, point: uiautomation::types::Point) -> WinUIAElement {
    //     let resut = self.element_from_point(point);
    //     let  win_uia_element = WinUIAElement::new(resut);
    //     //win_uia_element = Self::element_traversal(win_uia_element, point);
    //     return win_uia_element;
    // }
    // pub fn element_from_point(&mut self, point: uiautomation::types::Point) -> UIElement {
    //     let resut = self.automation.element_from_point(point);
    //     self.element = match resut {
    //         Ok(element) => element,
    //         Err(error) => panic!("element from point not found element: {:?}", error),
    //     };
    //     return self.element.clone();
    // }
    // // pub fn element_from_point(&mut self, point: uiautomation::types::Point) -> WinUIAElement {
    // //     let resut = self.automation.element_from_point(point);
    // //     self.element = match resut {
    // //         Ok(element) => element,
    // //         Err(error) => panic!("not found element: {:?}", error),
    // //     };
    // //     let mut win_uia_element = WinUIAElement::new(self.element.clone());
    // //     return win_uia_element;
    // // }
    // fn element_traversal(
    //     element: WinUIAElement,
    //     point: uiautomation::types::Point,
    // ) -> WinUIAElement {
    //     let children = element.get_children();
    //     for child in children {
    //         let _rect = child.get_bounding_rectangle();
    //         let magic_rect = rpa_core::rect::MagicRect::get_rect(&_rect);
    //         let magic_point = rpa_core::point::MagicPoint::get_point(point);
    //         if magic_rect.contain(magic_point) {
    //             return Self::element_traversal(child, point);
    //         }
    //     }
    //     return element;
    // }
    // pub fn get_parent(&self) -> WinUIAElement {
    //     let walker = self.automation.get_raw_view_walker().unwrap();
    //     let parent = match walker.get_parent(&self.element) {
    //         Ok(element) => element,
    //         Err(error) => panic!("not found element: {:?}", error),
    //     };
    //     let win_uia_element = WinUIAElement::new(parent.clone());
    //     return win_uia_element;
    // }
    // pub fn get_children(&self) -> Vec<WinUIAElement> {
    //     let walker = self.automation.get_raw_view_walker().unwrap();
    //     let mut child = match walker.get_first_child(&self.element) {
    //         Ok(element) => element,
    //         Err(error) => panic!("get children not found element: {:?}", error),
    //     };
    //     let mut children = Vec::new();
    //     children.push(WinUIAElement::new(child.clone()));
    //     loop {
    //         child = match walker.get_next_sibling(&child) {
    //             Ok(element) => element,
    //             Err(error) => panic!("get next sibling not found element: {:?}", error),
    //         };
    //         break;
    //     }
    //     children.push(WinUIAElement::new(child.clone()));
    //     return children;
    // }
    // pub fn get_name(&self) -> String {
    //     let result = &self.element.get_name().unwrap();
    //     return result.clone();
    // }
    // pub fn get_classname(&self) -> String {
    //     let result = &self.element.get_classname().unwrap();
    //     return result.clone();
    // }
    // pub fn get_bounding_rectangle(&self) -> Rect {
    //     let result = &self.element.get_bounding_rectangle().unwrap();
    //     return result.clone();
    // }
    // pub fn get_control_type(&self) -> ControlType {
    //     let result = &self.element.get_control_type().unwrap();
    //     return result.clone();
    // }
    // pub fn is_enabled(&self) -> bool {
    //     let result = &self.element.is_enabled().unwrap();
    //     return result.clone();
    // }

    // // pub fn is_focus(&self) -> bool {

    // // }
    // // pub fn is_edit(&self) -> bool {
    // //     return true;
    // // }
    // // pub fn is_checked(&self) -> bool {
    // //     return true;
    // // }
    // // pub fn set_check(&self) -> bool {
    // //     return true;
    // // }

    // // pub fn get_text(&self) -> &Result<String, uiautomation::Error>{}
    // // pub fn set_text(&self) -> &Result<String, uiautomation::Error>{}
    // // pub fn get_index(&self) -> i32 {
    // //     return  1;
    // // }

    // // pub fn get_select(&self) -> Vect<&str> {

    // // }
    // // pub fn set_select(&self) -> Vect<&str> {

    // // }
    // // pub fn click(&self) -> bool {
    // //     &self.element.click();
    // //     return true;
    // // }
    // // pub fn get_attribute(&self, name: &str) -> Vect<&str> {
    // //     return &self.element.get_attribute();
    // // }
}
