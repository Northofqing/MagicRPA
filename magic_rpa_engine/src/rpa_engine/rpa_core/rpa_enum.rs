/// Extended Window Styles for Win32 API
#[allow(non_upper_case_globals)]
pub mod ExtendedWindowStyles {
    /// The window accepts drag-drop files
    pub const WS_EX_ACCEPTFILES: u32 = 0x0000_0010;

    /// Forces a top-level window onto the taskbar when the window is visible
    pub const WS_EX_APPWINDOW: u32 = 0x0004_0000;

    /// The window has a border with a sunken edge
    pub const WS_EX_CLIEMagicEDGE: u32 = 0x0000_0200;

    /// Paints all descendants of a window in bottom-to-top painting order using double-buffering.
    /// Bottom-to-top painting order allows a descendent window to have translucency (alpha) and 
    /// transparency (color-key) effects, but only if the descendent window also has the 
    /// WS_EX_TRANSPAREMagic bit set. Double-buffering allows the window and its descendents to be 
    /// painted without flicker. This cannot be used if the window has a class style of either 
    /// CS_OWNDC or CS_CLASSDC.
    /// Windows 2000: This style is not supported.
    pub const WS_EX_COMPOSITED: u32 = 0x0200_0000;

    /// The title bar of the window includes a question mark. When the user clicks the question mark,
    /// the cursor changes to a question mark with a pointer. If the user then clicks a child window,
    /// the child receives a WM_HELP message. The child window should pass the message to the parent
    /// window procedure, which should call the WinHelp function using the HELP_WM_HELP command.
    /// The Help application displays a pop-up window that typically contains help for the child window.
    /// Cannot be used with the WS_MAXIMIZEBOX or WS_MINIMIZEBOX styles.
    pub const WS_EX_COMagicEXTHELP: u32 = 0x0000_0400;

    /// The window itself contains child windows that should take part in dialog box navigation.
    /// If this style is specified, the dialog manager recurses into children of this window when
    /// performing navigation operations such as handling the TAB key, an arrow key, or a keyboard
    /// mnemonic.
    pub const WS_EX_COMagicROLPAREMagic: u32 = 0x0001_0000;

    /// The window has a double border; the window can, optionally, be created with a title bar
    /// by specifying the WS_CAPTION style in the dwStyle parameter.
    pub const WS_EX_DLGMODALFRAME: u32 = 0x0000_0001;

    /// The window is a layered window. This style cannot be used if the window has a class style
    /// of either CS_OWNDC or CS_CLASSDC.
    /// Windows 8: Supported for top-level windows and child windows.
    /// Previous Windows versions support WS_EX_LAYERED only for top-level windows.
    pub const WS_EX_LAYERED: u32 = 0x0008_0000;

    /// If the shell language is Hebrew, Arabic, or another language that supports reading order
    /// alignment, the horizontal origin of the window is on the right edge. Increasing horizontal
    /// values advance to the left.
    pub const WS_EX_LAYOUTRTL: u32 = 0x0040_0000;

    /// The window has generic left-aligned properties. This is the default.
    pub const WS_EX_LEFT: u32 = 0x0000_0000;

    /// If the shell language is Hebrew, Arabic, or another language that supports reading order
    /// alignment, the vertical scroll bar (if present) is to the left of the client area.
    /// For other languages, the style is ignored.
    pub const WS_EX_LEFTSCROLLBAR: u32 = 0x0000_4000;

    /// The window text is displayed using left-to-right reading-order properties. This is the default.
    pub const WS_EX_LTRREADING: u32 = 0x0000_0000;

    /// The window is a MDI child window.
    pub const WS_EX_MDICHILD: u32 = 0x0000_0040;

    /// A top-level window created with this style does not become the foreground window when
    /// the user clicks it. The system does not bring this window to the foreground when the user
    /// minimizes or closes the foreground window.
    /// The window should not be activated through programmatic access or via keyboard navigation
    /// by accessible technology, such as Narrator.
    /// To activate the window, use the SetActiveWindow or SetForegroundWindow function.
    /// The window does not appear on the taskbar by default. To force the window to appear on
    /// the taskbar, use the WS_EX_APPWINDOW style.
    pub const WS_EX_NOACTIVATE: u32 = 0x0800_0000;

    /// The window does not pass its window layout to its child windows.
    pub const WS_EX_NOINHERITLAYOUT: u32 = 0x0010_0000;

    /// The child window created with this style does not send the WM_PAREMagicNOTIFY message
    /// to its parent window when it is created or destroyed.
    pub const WS_EX_NOPAREMagicNOTIFY: u32 = 0x0000_0004;

    /// The window does not render to a redirection surface. This is for windows that do not
    /// have visible content or that use mechanisms other than surfaces to provide their visual.
    pub const WS_EX_NOREDIRECTIONBITMAP: u32 = 0x0020_0000;

    /// The window has generic "right-aligned" properties. This depends on the window class.
    /// This style has an effect only if the shell language is Hebrew, Arabic, or another
    /// language that supports reading-order alignment; otherwise, the style is ignored.
    /// Using this style with static or edit controls has the same effect as using the SS_RIGHT
    /// or ES_RIGHT style, respectively. Using this style with button controls has the same effect
    /// as using BS_RIGHT and BS_RIGHTBUTTON styles.
    pub const WS_EX_RIGHT: u32 = 0x0001_0000;

    /// The vertical scroll bar (if present) is to the right of the client area. This is the default.
    pub const WS_EX_RIGHTSCROLLBAR: u32 = 0x0000_0000;

    /// If the shell language is Hebrew, Arabic, or another language that supports reading-order
    /// alignment, the window text is displayed using right-to-left reading-order properties.
    /// For other languages, the style is ignored.
    pub const WS_EX_RTLREADING: u32 = 0x0002_0000;

    /// The window has a three-dimensional border style intended to be used for items that do not
    /// accept user input.
    pub const WS_EX_STATICEDGE: u32 = 0x0002_0000;

    /// The window is intended to be used as a floating toolbar. A tool window has a title bar
    /// that is shorter than a normal title bar, and the window title is drawn using a smaller font.
    /// A tool window does not appear in the taskbar or in the dialog that appears when the user
    /// presses ALT+TAB. If a tool window has a system menu, its icon is not displayed on the title bar.
    /// However, you can display the system menu by right-clicking or by typing ALT+SPACE.
    pub const WS_EX_TOOLWINDOW: u32 = 0x0000_0080;

    /// The window should be placed above all non-topmost windows and should stay above them,
    /// even when the window is deactivated. To add or remove this style, use the SetWindowPos function.
    pub const WS_EX_TOPMOST: u32 = 0x0000_0008;

    /// The window should not be painted until siblings beneath the window (that were created
    /// by the same thread) have been painted. The window appears transparent because the bits
    /// of underlying sibling windows have already been painted.
    /// To achieve transparency without these restrictions, use the SetWindowRgn function.
    pub const WS_EX_TRANSPAREMagic: u32 = 0x0000_0020;

    /// The window has a border with a raised edge.
    pub const WS_EX_WINDOWEDGE: u32 = 0x0000_0100;

    /// The window is an overlapped window.
    pub const WS_EX_OVERLAPPEDWINDOW: u32 = WS_EX_WINDOWEDGE | WS_EX_CLIEMagicEDGE;

    /// The window is palette window, which is a modeless dialog box that presents an array of commands.
    pub const WS_EX_PALETTEWINDOW: u32 = WS_EX_WINDOWEDGE | WS_EX_TOOLWINDOW | WS_EX_TOPMOST;

    /// Helper function to check if a style is set in a combination of styles
    pub const fn has_style(styles: u32, style: u32) -> bool {
        (styles & style) == style
    }

    /// Helper function to combine multiple styles
    pub const fn combine_styles(styles: &[u32]) -> u32 {
        let mut result = 0;
        let mut i = 0;
        while i < styles.len() {
            result |= styles[i];
            i += 1;
        }
        result
    }#[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Style {
        AcceptFiles,
        AppWindow,
        ClientEdge,
        Composited,
        ContextHelp,
        ControlParent,
        DlgModalFrame,
        Layered,
        LayoutRtl,
        Left,
        LeftScrollBar,
        LtrReading,
        MdiChild,
        NoActivate,
        NoInheritLayout,
        NoParentNotify,
        NoRedirectionBitmap,
        Right,
        RightScrollBar,
        RtlReading,
        StaticEdge,
        ToolWindow,
        Topmost,
        Transparent,
        WindowEdge,
        OverlappedWindow,
        PaletteWindow,
        Unknown(u32),  // 用于处理未知的值
    }

    impl Style {
        /// 从一个 u32 值获取对应的样式
        pub fn from_u32(value: u32) -> Style {
            match value {
                WS_EX_ACCEPTFILES => Style::AcceptFiles,
                WS_EX_APPWINDOW => Style::AppWindow,
                WS_EX_CLIEMagicEDGE => Style::ClientEdge,
                WS_EX_COMPOSITED => Style::Composited,
                WS_EX_COMagicEXTHELP => Style::ContextHelp,
                WS_EX_COMagicROLPAREMagic => Style::ControlParent,
                WS_EX_DLGMODALFRAME => Style::DlgModalFrame,
                WS_EX_LAYERED => Style::Layered,
                WS_EX_LAYOUTRTL => Style::LayoutRtl,
                WS_EX_LEFT => Style::Left,
                WS_EX_LEFTSCROLLBAR => Style::LeftScrollBar,
                WS_EX_LTRREADING => Style::LtrReading,
                WS_EX_MDICHILD => Style::MdiChild,
                WS_EX_NOACTIVATE => Style::NoActivate,
                WS_EX_NOINHERITLAYOUT => Style::NoInheritLayout,
                WS_EX_NOPAREMagicNOTIFY => Style::NoParentNotify,
                WS_EX_NOREDIRECTIONBITMAP => Style::NoRedirectionBitmap,
                WS_EX_RIGHT => Style::Right,
                WS_EX_RIGHTSCROLLBAR => Style::RightScrollBar,
                WS_EX_RTLREADING => Style::RtlReading,
                WS_EX_STATICEDGE => Style::StaticEdge,
                WS_EX_TOOLWINDOW => Style::ToolWindow,
                WS_EX_TOPMOST => Style::Topmost,
                WS_EX_TRANSPAREMagic => Style::Transparent,
                WS_EX_WINDOWEDGE => Style::WindowEdge,
                WS_EX_OVERLAPPEDWINDOW => Style::OverlappedWindow,
                WS_EX_PALETTEWINDOW => Style::PaletteWindow,
                _ => Style::Unknown(value),
            }
        }

        /// 获取样式对应的 u32 值
        pub fn to_u32(&self) -> u32 {
            match self {
                Style::AcceptFiles => WS_EX_ACCEPTFILES,
                Style::AppWindow => WS_EX_APPWINDOW,
                Style::ClientEdge => WS_EX_CLIEMagicEDGE,
                Style::Composited => WS_EX_COMPOSITED,
                Style::ContextHelp => WS_EX_COMagicEXTHELP,
                Style::ControlParent => WS_EX_COMagicROLPAREMagic,
                Style::DlgModalFrame => WS_EX_DLGMODALFRAME,
                Style::Layered => WS_EX_LAYERED,
                Style::LayoutRtl => WS_EX_LAYOUTRTL,
                Style::Left => WS_EX_LEFT,
                Style::LeftScrollBar => WS_EX_LEFTSCROLLBAR,
                Style::LtrReading => WS_EX_LTRREADING,
                Style::MdiChild => WS_EX_MDICHILD,
                Style::NoActivate => WS_EX_NOACTIVATE,
                Style::NoInheritLayout => WS_EX_NOINHERITLAYOUT,
                Style::NoParentNotify => WS_EX_NOPAREMagicNOTIFY,
                Style::NoRedirectionBitmap => WS_EX_NOREDIRECTIONBITMAP,
                Style::Right => WS_EX_RIGHT,
                Style::RightScrollBar => WS_EX_RIGHTSCROLLBAR,
                Style::RtlReading => WS_EX_RTLREADING,
                Style::StaticEdge => WS_EX_STATICEDGE,
                Style::ToolWindow => WS_EX_TOOLWINDOW,
                Style::Topmost => WS_EX_TOPMOST,
                Style::Transparent => WS_EX_TRANSPAREMagic,
                Style::WindowEdge => WS_EX_WINDOWEDGE,
                Style::OverlappedWindow => WS_EX_OVERLAPPEDWINDOW,
                Style::PaletteWindow => WS_EX_PALETTEWINDOW,
                Style::Unknown(value) => *value,
            }
        }

        /// 从 i32 转换为样式
        pub fn from_i32(value: i32) -> Style {
            Self::from_u32(value as u32)
        }

        /// 获取所有设置的样式
        pub fn get_styles(value: u32) -> Vec<Style> {
            let mut styles = Vec::new();
            let all_styles = [
                WS_EX_ACCEPTFILES,
                WS_EX_APPWINDOW,
                WS_EX_CLIEMagicEDGE,
                WS_EX_COMPOSITED,
                WS_EX_COMagicEXTHELP,
                WS_EX_COMagicROLPAREMagic,
                WS_EX_DLGMODALFRAME,
                WS_EX_LAYERED,
                WS_EX_LAYOUTRTL,
                WS_EX_LEFT,
                WS_EX_LEFTSCROLLBAR,
                WS_EX_LTRREADING,
                WS_EX_MDICHILD,
                WS_EX_NOACTIVATE,
                WS_EX_NOINHERITLAYOUT,
                WS_EX_NOPAREMagicNOTIFY,
                WS_EX_NOREDIRECTIONBITMAP,
                WS_EX_RIGHT,
                WS_EX_RIGHTSCROLLBAR,
                WS_EX_RTLREADING,
                WS_EX_STATICEDGE,
                WS_EX_TOOLWINDOW,
                WS_EX_TOPMOST,
                WS_EX_TRANSPAREMagic,
                WS_EX_WINDOWEDGE,
            ];

            for style in all_styles.iter() {
                if has_style(value, *style) {
                    styles.push(Style::from_u32(*style));
                }
            }

            // 检查组合样式
            if has_style(value, WS_EX_OVERLAPPEDWINDOW) {
                styles.push(Style::OverlappedWindow);
            }
            if has_style(value, WS_EX_PALETTEWINDOW) {
                styles.push(Style::PaletteWindow);
            }

            styles
        }
    }

    // 为 Style 实现转换 trait
    impl From<u32> for Style {
        fn from(value: u32) -> Self {
            Style::from_u32(value)
        }
    }

    impl From<i32> for Style {
        fn from(value: i32) -> Self {
            Style::from_i32(value)
        }
    }
}

// 使用示例
pub fn example() {
    use ExtendedWindowStyles::*;

    // 从数值转换为样式
    let style1: Style = 0x0000_0010i32.into();
    assert_eq!(style1, Style::AcceptFiles);

    let style2: Style = 0x0000_0080u32.into();
    assert_eq!(style2, Style::ToolWindow);

    // 获取组合样式中的所有设置
    let combined_style = WS_EX_ACCEPTFILES | WS_EX_TOPMOST | WS_EX_CLIEMagicEDGE;
    let styles = Style::get_styles(combined_style);
    
    for style in styles {
        match style {
            Style::AcceptFiles => println!("Has AcceptFiles"),
            Style::Topmost => println!("Has Topmost"),
            Style::ClientEdge => println!("Has ClientEdge"),
            _ => println!("Has other style: {:?}", style),
        }
    }

    // 获取未知样式
    let unknown_style: Style = 0x1234_5678u32.into();
    match unknown_style {
        Style::Unknown(value) => println!("Unknown style value: {:#x}", value),
        _ => println!("Known style"),
    }
}