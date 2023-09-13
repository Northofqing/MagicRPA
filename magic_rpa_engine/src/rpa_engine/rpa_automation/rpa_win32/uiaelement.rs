use crate::rpa_engine::rpa_core::rect::MagicRect;

use crate::rpa_engine::rpa_automation::rpa_element::Element;
use uiautomation::types::UIProperty;
use uiautomation::UIAutomation;
use uiautomation::UIElement;

use log::error;
use std::collections::HashMap;

use crate::rpa_engine::rpa_core::error::Result;

#[derive(Debug, Clone)]
pub struct WinUIAElement {
    automation: UIAutomation,
    element: UIElement,
}
impl WinUIAElement {
    pub fn new() -> WinUIAElement {
        let automation = UIAutomation::new().unwrap();
        Self {
            automation: automation.clone(),
            element: automation.get_root_element().unwrap(),
        }
    }
    pub fn update_element(element: UIElement) -> WinUIAElement {
        let automation = UIAutomation::new().unwrap();
        Self {
            automation: automation.clone(),
            element: element,
        }
    }
    pub fn wrap_box(&self, element: UIElement) -> Box<dyn Element> {
        let mut _win_elemet = WinUIAElement {
            automation: self.automation.clone(),
            element: element.clone(),
        };
        return Box::new(_win_elemet);
    }
    pub fn is_same(&self, element: WinUIAElement) -> Result<bool> {
        match &self
            .automation
            .compare_elements(&self.element, &element.element)
        {
            Ok(result) => Ok(*result),
            Err(_) => Ok(false),
        }
    }

    // pub fn is_null(&self) ->  bool {
    //     true
    // }
    // pub fn get_legacy_iaccessible(self) ->  uiautomation::variants::Variant {
    //     match self
    //         .element
    //         .get_property_value(UIProperty::IsLegacyIAccessiblePatternAvailable)
    //     {
    //         Ok(_islegacy) => {

    //             let p = UIPatternType::LegacyIAccessible;

    //           match self.element.get_pattern( ) {
    //               Ok(legacy)=>{
    //                 legacy
    //               },
    //               Err(_)=>{
    //                 panic!("")
    //               }
    //           }
    //         },
    //         Err(_) => {panic!("")}
    //     }
    // }
}

impl Element for WinUIAElement {
    fn get_name(&self) -> Result<String> {
        match self.element.get_name() {
            Ok(name) => {
                return Ok(name);
            }
            Err(e) => {
                return Ok(String::new());
            }
        };
    }
    fn get_classname(&self) -> Result<String> {
        match &self.element.get_classname() {
            Ok(classname) => {
                return Ok(classname.clone());
            }
            Err(e) => {
                error!("get element classname error:{e}");
                return Ok(String::from(""));
            }
        };
    }
    fn get_bounding_rectangle(&self) -> Result<MagicRect> {
        match &self.element.get_bounding_rectangle() {
            Ok(rect) => {
                return Ok(MagicRect {
                    x: rect.get_left(),
                    y: rect.get_top(),
                    width: rect.get_width(),
                    height: rect.get_height(),
                });
            }
            Err(e) => {
                error!("get element bounding rectangle error:{e}");
                return Ok(MagicRect {
                    x: 0,
                    y: 0,
                    width: 0,
                    height: 0,
                });
            }
        };
    }
    fn get_control_type(&self) -> Result<String> {
        match &self.element.get_control_type() {
            Ok(controltype) => {
                return Ok(format!("{:?}", controltype));
            }
            Err(e) => {
                error!("get control type error:{}", e);
                return Ok(String::from(""));
            }
        };
    }

    fn get_index(&self) -> Result<i32> {
        let mut index = -1;
        let Ok(parent) = self.get_parent() else {
            return Ok(index);
        };
        let Ok(children) = parent.get_children() else {
            return Ok(index);
        };
        for child in children {
            index += 1;
            // let Ok(result) = WinUIAElement::is_same(&self, );
            // if result {
            //     return Ok(index);
            // }
        }
        return Ok(index);
    }
    fn is_enabled(&self) -> Result<bool> {
        match &self.element.is_enabled() {
            Ok(enable) => {
                return Ok(enable.clone());
            }
            Err(_) => {
                return Ok(bool::from(false));
            }
        }
    }
    fn is_focus(&self) -> Result<bool> {
        return Ok(bool::from(true));
    }
    fn is_edit(&self) -> Result<bool> {
        return Ok(bool::from(true));
    }
    fn input(&self) -> Result<bool> {
        return Ok(bool::from(true));
    }
    fn get_checked(&self) -> Result<bool> {
        return Ok(bool::from(true));
    }
    fn set_check(&self) -> Result<bool> {
        return Ok(bool::from(true));
    }
    fn get_text(&self) -> Result<String> {
        let mut result = String::new();

        match self
            .element
            .get_pattern::<uiautomation::patterns::UITextPattern>()
        {
            Ok(_text_pattern) => match _text_pattern.get_document_range() {
                Ok(_document_range) => match _document_range.get_text(-1) {
                    Ok(_text) => {
                        result = _text;
                    }
                    Err(_) => {}
                },
                Err(_) => {}
            },
            Err(_) => {}
        }
        return Ok(result);
    }
    fn set_text(&self) -> Result<bool> {
        return Ok(true);
    }
    fn get_value(&self) -> Result<String> {
        let mut result = String::from("");

        match self
            .element
            .get_pattern::<uiautomation::patterns::UIRangeValuePattern>()
        {
            Ok(_value_pattern) => match _value_pattern.get_value() {
                Ok(value) => result = value.to_string(),
                Err(_) => result = String::from(""),
            },
            Err(_) => result = String::from(""),
        };
        return Ok(result);
    }
    fn get_select(&self) -> Result<Vec<String>> {
        let mut map_selectitem: HashMap<String, Box<dyn Element>> = HashMap::new();
        match self
            .element
            .get_pattern::<uiautomation::patterns::UIScrollPattern>()
        {
            Ok(_vertical_scroll_percent) => {
                if _vertical_scroll_percent
                    .get_vertical_scroll_percent()
                    .unwrap()
                    == 0.0
                {
                    let element = WinUIAElement::update_element(self.element.clone());
                    for ele in element.get_children().unwrap() {
                        map_selectitem.insert(ele.get_name().unwrap(), ele);
                    }
                }
            }
            Err(_) => {}
        }

        return Ok(vec![String::from("")]);
    }
    fn set_select(&self) -> Result<bool> {
        return Ok(true);
    }
    fn event_click(&self) -> Result<bool> {
        let _ = &self.element.click();
        return Ok(bool::from(true));
    }
    fn get_attributes(&self) -> Result<HashMap<String, String>> {
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
        return Ok(map_attributes);
    }
    fn get_attribute(&self, _attribute_name: &str) -> Result<String> {
        let attrs = self.get_attributes().unwrap();
        if attrs.contains_key(_attribute_name) {
            return Ok(attrs[_attribute_name].clone());
        }
        return Ok(String::from(""));
    }
    fn set_attribute(&self, _attribute_name: &str, _attribute_value: &str) -> Result<bool> {
        return Ok(true);
    }

    fn get_parent(&self) -> Result<Box<dyn Element>> {
        let walker = self.automation.get_raw_view_walker().unwrap();

        match walker.get_parent(&self.element) {
            Ok(element) => {
                let win_uia_element = WinUIAElement {
                    automation: self.automation.clone(),
                    element: element.clone(),
                };
                return Ok(Box::new(win_uia_element));
            }
            Err(e) => return Err(e.into()),
        };
    }
    fn get_children(&self) -> Result<Vec<Box<dyn Element>>> {
        let walker = self.automation.get_raw_view_walker().unwrap();
        let mut children: Vec<Box<dyn Element>> = Vec::new();
        if let Ok(child) = walker.get_first_child(&self.element) {
            let _box_win_element = WinUIAElement::wrap_box(&self, child.clone());
            children.push(_box_win_element);
            let mut next = child;
            while let Ok(sibling) = walker.get_next_sibling(&next) {
                next = sibling;
                let _box_win_element = WinUIAElement::wrap_box(&self, next.clone());
                children.push(_box_win_element);
            }
        }
        // let mut child = match walker.get_first_child(&self.element) {
        //     Ok(element) => element,
        //     Err(_) => {
        //         let result: Vec<Box<dyn Element>> = Vec::new();
        //         return Ok(result);
        //     }
        // };
        // match Some(child) {
        //     Some(ele) => {
        //         child = ele;
        //     }
        //     None => {
        //         let _ = walker.get_next_sibling(&self.element);
        //         child = match walker.get_first_child(&self.element) {
        //             Ok(element) => element,
        //             Err(_) => {
        //                 let result: Vec<Box<dyn Element>> = Vec::new();
        //                 return Ok(result);
        //             }
        //         }
        //     }
        // }

        // let mut children: Vec<Box<dyn Element>> = Vec::new();
        // let mut win_elemet = WinUIAElement {
        //     automation: self.automation.clone(),
        //     element: child.clone(),
        // };
        // let mut box_win_element = Box::new(win_elemet);
        // children.push(box_win_element);
        // loop {
        //     child = match walker.get_next_sibling(&child) {
        //         Ok(element) => element,
        //         Err(_) => break,
        //     };

        //     win_elemet = WinUIAElement {
        //         automation: self.automation.clone(),
        //         element: child.clone(),
        //     };
        //     box_win_element = Box::new(win_elemet);
        //     children.push(box_win_element);
        // }
        return Ok(children);
    }
    fn get_descendants(&self) -> Result<Vec<Box<dyn Element>>> {
        let mut _descendants: Vec<Box<dyn Element>> = Vec::new();
        match &self.element.get_described_by() {
            Ok(elements) => {
                for element in elements {
                    let win_elemet = WinUIAElement {
                        automation: self.automation.clone(),
                        element: element.clone(),
                    };
                    let box_win_element = Box::new(win_elemet);
                    _descendants.push(box_win_element);
                }
                return Ok(_descendants);
            }
            Err(_) => {
                return Ok(_descendants);
            }
        };
    }
}
