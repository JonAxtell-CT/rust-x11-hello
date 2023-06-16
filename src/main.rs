extern crate x11;

use std::ffi::CString;
use std::mem::{self, MaybeUninit};
use std::os::raw::c_char;
use std::os::raw::*;
use std::ptr::{self, null_mut};
use x11::xlib;

#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
enum GraphicContextFunctions {
    GXclear = 0x0,
    GXand = 0x1,
    GXandReverse = 0x2,
    GXcopy = 0x3,
    GXandInverted = 0x4,
    GXnoop = 0x5,
    GXxor = 0x6,
    GXor = 0x7,
    GXnor = 0x8,
    GXequiv = 0x9,
    GXinvert = 0xa,
    GXorReverse = 0xb,
    GXcopyInverted = 0xc,
    GXorInverted = 0xd,
    GXnand = 0xe,
    GXset = 0xf,
}

// LineStyle
#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
enum GraphicContextLineStyles {
    LineSolid = 0,
    LineOnOffDash = 1,
    LineDoubleDash = 2,
}

// capStyle
#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
enum GraphicContextCapStyles {
    CapNotLast = 0,
    CapButt = 1,
    CapRound = 2,
    CapProjecting = 3,
}

// joinStyle
#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
enum GraphicContextJoinStyles {
    JoinMiter = 0,
    JoinRound = 1,
    JoinBevel = 2,
}

// fillStyle
#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
enum GraphicContextFillStyles {
    FillSolid = 0,
    FillTiled = 1,
    FillStippled = 2,
    FillOpaqueStippled = 3,
}

// fillRule
#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
enum GraphicContextFillRules {
    EvenOddRule = 0,
    WindingRule = 1,
}

// Arc modes for PolyFillArc
#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
enum GraphicContextArcModes {
    ArcChord = 0,
    ArcPieSlice = 1,
}

// subwindow mode
#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
enum GraphicContextSubWindowModes {
    ClipByChildren = 0,
    IncludeInferiors = 1,
}

// Graphic exposure
#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
enum GraphicContextGraphicExposure {
    CopyArea = 0,
    CopyPlane = 1,
}

// window classes
#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
enum WindowClasses {
    InputOutput = 1,
    InputOnly = 2,
}

/// Structure that maps to the C structure in X11 as closely as possible
#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
struct GraphicContextComponents {
    /// Logical function
    pub function: c_int,

    /// Plane mask
    pub plane_mask: c_ulong,

    /// Foreground pixel colour
    pub foreground: c_ulong,

    /// Background pixel colour
    pub background: c_ulong,

    /// Line width
    pub line_width: c_int,

    /// Line style (solid, on-off dash, double dash)
    pub line_style: c_int,

    /// Cap (end of line) style (not last, butt, round, projecting)
    pub cap_style: c_int,

    /// Line join style (miter, round, bevel)
    pub join_style: c_int,

    /// Fill style (solid, tiled, stuppled, opaque-stippled)
    pub fill_style: c_int,

    /// Fill rule (even-odd, winding)
    pub fill_rule: c_int,

    /// Arc mode (chord, pie-slice)
    pub arc_mode: c_int,

    /// Pixmap to use for tiling operations
    pub tile: xlib::Pixmap,

    /// Pixmap of depth 1 to use for stippling
    pub stipple: xlib::Pixmap,

    /// Tile/stipple origin
    pub ts_x_origin: c_int,
    pub ts_y_origin: c_int,

    /// Font for text operations (except XDrawText)
    pub font: xlib::Font,

    /// Subwindow mode (clip-by-children, include-inferiors)
    pub subwindow_mode: c_int,

    /// Graphic exposure (copy-area, copy-plane)
    pub graphics_exposures: bool,

    /// Clip origin
    pub clip_x_origin: c_int,
    pub clip_y_origin: c_int,

    /// Pixmap to use for clipping
    pub clip_mask: xlib::Pixmap,

    /// Dash information (patterned, dashed)
    pub dash_offset: c_int,

    /// Dash pattern
    pub dashes: c_char,
}

#[derive(Copy, Clone, PartialEq)]
#[allow(dead_code)]
struct GraphicContextBuilder {
    components: GraphicContextComponents,
    used: u32,
}

impl Default for GraphicContextBuilder {
    fn default() -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                function: xlib::GXcopy,
                plane_mask: 0xFFFFFFFF,
                foreground: 0,
                background: 1,
                line_width: 0,
                line_style: xlib::LineSolid,
                cap_style: xlib::CapButt,
                join_style: xlib::JoinMiter,
                fill_style: xlib::FillSolid,
                fill_rule: xlib::EvenOddRule,
                arc_mode: xlib::ArcPieSlice,
                tile: 0,
                stipple: 0,
                ts_x_origin: 0,
                ts_y_origin: 0,
                font: 0,
                subwindow_mode: xlib::ClipByChildren,
                graphics_exposures: true,
                clip_x_origin: 0,
                clip_y_origin: 0,
                clip_mask: 0,
                dash_offset: 0,
                dashes: 4,
            },
            used: 0,
        }
    }
}

#[allow(dead_code)]
impl GraphicContextBuilder {
    pub fn set_function(self, function: GraphicContextFunctions) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                function: function as i32,
                ..self.components
            },
            used: self.used | xlib::GCFunction,
            ..self
        }
    }

    pub fn set_foreground_colour(self, foreground: c_ulong) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                foreground,
                ..self.components
            },
            used: self.used | xlib::GCForeground,
            ..self
        }
    }

    pub fn set_background_colour(self, background: c_ulong) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                background,
                ..self.components
            },
            used: self.used | xlib::GCBackground,
            ..self
        }
    }

    pub fn set_line_width(self, line_width: c_int) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                line_width,
                ..self.components
            },
            used: self.used | xlib::GCBackground,
            ..self
        }
    }

    pub fn set_line_style(self, line_style: GraphicContextLineStyles) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                line_style: line_style as i32,
                ..self.components
            },
            used: self.used | xlib::GCBackground,
            ..self
        }
    }

    pub fn set_cap_style(self, cap_style: GraphicContextCapStyles) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                cap_style: cap_style as i32,
                ..self.components
            },
            used: self.used | xlib::GCBackground,
            ..self
        }
    }

    pub fn set_join_style(self, join_style: GraphicContextJoinStyles) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                join_style: join_style as i32,
                ..self.components
            },
            used: self.used | xlib::GCBackground,
            ..self
        }
    }

    pub fn set_fill_style(self, fill_style: GraphicContextFillStyles) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                fill_style: fill_style as i32,
                ..self.components
            },
            used: self.used | xlib::GCBackground,
            ..self
        }
    }

    pub fn set_fill_rule(self, fill_rule: GraphicContextFillStyles) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                fill_rule: fill_rule as i32,
                ..self.components
            },
            used: self.used | xlib::GCBackground,
            ..self
        }
    }

    pub fn set_arc_mode(self, arc_mode: GraphicContextArcModes) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                arc_mode: arc_mode as i32,
                ..self.components
            },
            used: self.used | xlib::GCBackground,
            ..self
        }
    }

    pub fn set_tile(self, tile: u64) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                tile: tile,
                ..self.components
            },
            used: self.used | xlib::GCBackground,
            ..self
        }
    }

    pub fn set_stipple(self, stipple: u64) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                stipple: stipple,
                ..self.components
            },
            used: self.used | xlib::GCBackground,
            ..self
        }
    }

    pub fn set_tile_stipple_origin(self, x: i32, y: i32) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                ts_x_origin: x,
                ts_y_origin: y,
                ..self.components
            },
            used: self.used | xlib::GCBackground,
            ..self
        }
    }

    pub fn set_font_from_string(self, display: *mut xlib::Display, name: &str) -> Self {
        unsafe {
            let name = CString::new(name).unwrap();
            let font = xlib::XLoadFont(display, name.as_ptr());
            GraphicContextBuilder {
                components: GraphicContextComponents {
                    font,
                    ..self.components
                },
                used: self.used | xlib::GCFont,
                ..self
            }
        }
    }

    pub fn set_font_from_id(self, font: xlib::Font) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                font,
                ..self.components
            },
            used: self.used | xlib::GCFont,
            ..self
        }
    }

    pub fn set_sub_window_mode(self, subwindow_mode: GraphicContextSubWindowModes) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                subwindow_mode: subwindow_mode as i32,
                ..self.components
            },
            used: self.used | xlib::GCBackground,
            ..self
        }
    }

    pub fn set_graphic_exposure(self, graphic_exposure: GraphicContextGraphicExposure) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                graphics_exposures: match graphic_exposure {
                    GraphicContextGraphicExposure::CopyArea => false,
                    GraphicContextGraphicExposure::CopyPlane => true,
                },
                ..self.components
            },
            used: self.used | xlib::GCBackground,
            ..self
        }
    }

    pub fn set_clip_origin(self, x: i32, y: i32) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                clip_x_origin: x,
                clip_y_origin: y,
                ..self.components
            },
            used: self.used | xlib::GCBackground,
            ..self
        }
    }

    pub fn set_clip_mask(self, clip_mask: u64) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                clip_mask,
                ..self.components
            },
            used: self.used | xlib::GCBackground,
            ..self
        }
    }

    pub fn set_dash_offset(self, dash_offset: c_int) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                dash_offset,
                ..self.components
            },
            used: self.used | xlib::GCBackground,
            ..self
        }
    }

    pub fn set_dashes(self, dashes: c_char) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                dashes,
                ..self.components
            },
            used: self.used | xlib::GCBackground,
            ..self
        }
    }

    pub fn create(self, display: *mut xlib::Display, window: xlib::Window) -> xlib::GC {
        unsafe {
            let xgcvalue: MaybeUninit<xlib::XGCValues> = MaybeUninit::uninit();
            let mut xgcvalue = xgcvalue.assume_init();

            xgcvalue.function = self.components.function;
            xgcvalue.plane_mask = self.components.plane_mask;
            xgcvalue.foreground = self.components.foreground;
            xgcvalue.background = self.components.background;
            xgcvalue.line_width = self.components.line_width;
            xgcvalue.line_style = self.components.line_style;
            xgcvalue.cap_style = self.components.cap_style;
            xgcvalue.join_style = self.components.join_style;
            xgcvalue.fill_style = self.components.fill_style;
            xgcvalue.fill_rule = self.components.fill_rule;
            xgcvalue.arc_mode = self.components.arc_mode;
            xgcvalue.tile = self.components.tile;
            xgcvalue.stipple = self.components.stipple;
            xgcvalue.ts_x_origin = self.components.ts_x_origin;
            xgcvalue.ts_y_origin = self.components.ts_y_origin;
            xgcvalue.font = self.components.font;
            xgcvalue.subwindow_mode = self.components.subwindow_mode;
            xgcvalue.graphics_exposures = self.components.graphics_exposures.into();
            xgcvalue.clip_x_origin = self.components.clip_x_origin;
            xgcvalue.clip_y_origin = self.components.clip_y_origin;
            xgcvalue.clip_mask = self.components.clip_mask;
            xgcvalue.dash_offset = self.components.dash_offset;
            xgcvalue.dashes = self.components.dashes;

            let gc = xlib::XCreateGC(
                display,
                window,
                self.used.try_into().unwrap(),
                &mut xgcvalue,
            );
            gc
        }
    }
}

#[derive(Clone, PartialEq)]
#[allow(dead_code)]
struct WindowBuilder {
    display: *mut xlib::Display,
    root: c_ulong,
    x: c_int,
    y: c_int,
    w: c_uint,
    h: c_uint,
    b: c_uint,
    d: c_int,
    class: c_int,
    visual: *mut xlib::Visual,
    value_mask: c_ulong,
    attributes: *mut xlib::XSetWindowAttributes,
    title: String,
}

impl Default for WindowBuilder {
    fn default() -> Self {
        WindowBuilder {
            display: null_mut(),
            root: 0,
            x: 0,
            y: 0,
            w: 0,
            h: 0,
            b: 5,
            d: 0,
            class: xlib::InputOutput,
            visual: null_mut(),
            value_mask: 0,
            attributes: null_mut(),
            title: String::new(),
        }
    }
}

#[allow(dead_code)]
impl WindowBuilder {
    pub fn set_origin(self, x: c_int, y: c_int) -> Self {
        WindowBuilder { x, y, ..self }
    }

    pub fn set_size(self, w: c_uint, h: c_uint) -> Self {
        WindowBuilder { w, h, ..self }
    }

    pub fn set_border(self, b: c_uint) -> Self {
        WindowBuilder { b, ..self }
    }

    pub fn set_depth(self, d: c_int) -> Self {
        WindowBuilder { d, ..self }
    }

    pub fn set_class(self, class: WindowClasses) -> Self {
        WindowBuilder { class:class  as c_int, ..self }
    }

    pub fn set_visual(self, visual: *mut xlib::Visual) -> Self {
        WindowBuilder { visual, ..self }
    }

    pub fn set_value_mask(self, value_mask: c_ulong) -> Self {
        WindowBuilder { value_mask, ..self }
    }

    pub fn set_attributes(self, attributes: *mut xlib::XSetWindowAttributes ) -> Self {
        WindowBuilder { attributes, ..self }
    }

    pub fn set_title(self, title: String) -> Self {
        WindowBuilder { title, ..self }
    }

    pub fn create(self, display: *mut xlib::Display, root: c_ulong) -> c_ulong {
        unsafe {
            let window: c_ulong = xlib::XCreateWindow(
                display,              // Display
                root,                 // Root window
                self.x,               // x
                self.y,               // y
                self.w,               // w
                self.h,               // h
                self.b,               // ?
                self.d,               // depth?
                self.class as c_uint, // Input only or input/output
                self.visual,          // ?
                self.value_mask,      // ?
                self.attributes,      // ?
            );

            if !self.title.is_empty() {
                // let title_str = CString::new("hello-world").unwrap();
                xlib::XStoreName(display, window, self.title.as_ptr() as *mut c_char);
            }
            window
        }
    }
}

fn pixel_value_for_colour(display: *mut xlib::Display, screen: c_int, color: &str) -> c_ulong {
    let mut xcolour: xlib::XColor = xlib::XColor {
        pixel: 0,
        red: 0,
        green: 0,
        blue: 0,
        flags: 0,
        pad: 0,
    };
    unsafe {
        let color_map = xlib::XDefaultColormap(display, screen);
        let str = CString::new(color.as_bytes()).unwrap();
        xlib::XAllocNamedColor(display, color_map, str.as_ptr(), &mut xcolour, &mut xcolour);
    }

    xcolour.pixel
}

struct XDisplay {
    d:*mut xlib::Display,
}

impl XDisplay {
    fn new() -> Self {
        println!("Display opened");
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());
            if display.is_null() {
                panic!("XOpenDisplay failed");
            }
            XDisplay {
                d:display
            }
        }
    }

    fn display(&mut self) -> *mut xlib::Display {
        self.d
    }
}

impl Drop for XDisplay {
    fn drop(&mut self) {
        unsafe {
            println!("Display closed");
            xlib::XCloseDisplay(self.d);
        }
    }
}

fn main() {
    unsafe {
        // Open display connection.
        // let display = xlib::XOpenDisplay(ptr::null());
        // if display.is_null() {
        //     panic!("XOpenDisplay failed");
        // }

        let mut display = XDisplay::new();

        println!(
            "PRIMARY 0x{:02X}",
            xlib::XGetSelectionOwner(
                display.display(),
                xlib::XInternAtom(
                    display.display(),
                    CString::new("PRIMARY").unwrap().to_owned().as_ptr(),
                    0
                )
            )
        );
        println!(
            "SECONDARY 0x{:02X}",
            xlib::XGetSelectionOwner(
                display.display(),
                xlib::XInternAtom(
                    display.display(),
                    CString::new("SECONDARY").unwrap().to_owned().as_ptr(),
                    0
                )
            )
        );
        println!(
            "CLIPBOARD 0x{:02X}",
            xlib::XGetSelectionOwner(
                display.display(),
                xlib::XInternAtom(
                    display.display(),
                    CString::new("CLIPBOARD").unwrap().to_owned().as_ptr(),
                    0
                )
            )
        );
        println!(
            "FOOBAR 0x{:02X}",
            xlib::XGetSelectionOwner(
                display.display(),
                xlib::XInternAtom(
                    display.display(),
                    CString::new("FOOBAR").unwrap().to_owned().as_ptr(),
                    0
                )
            )
        );

        // Get x11 windows
        let screen = xlib::XDefaultScreen(display.display());
        let root = xlib::XRootWindow(display.display(), screen);
        let depth = xlib::XDefaultDepth(display.display(), screen);

        println!("Screen={}, Root={}", screen, root);
        println!(
            "{} x {}",
            xlib::XDisplayWidth(display.display(), screen),
            xlib::XDisplayHeight(display.display(), screen)
        );
        println!(
            "{} x {}",
            xlib::XDisplayWidthMM(display.display(), screen),
            xlib::XDisplayHeightMM(display.display(), screen)
        );

        // Set background to white. Requires uninit since the attribute structure is not initialised except for the background.
        let mut attributes: MaybeUninit<xlib::XSetWindowAttributes> =
            MaybeUninit::uninit().assume_init();
        let attr_ptr = attributes.as_mut_ptr();
        std::ptr::addr_of_mut!((*attr_ptr).background_pixel)
            .write(xlib::XWhitePixel(display.display(), screen));
        let mut attributes = attributes.assume_init();

        // Create the window
        let window = WindowBuilder::default()
            .set_origin(0, 0)
            .set_size(400, 300)
            .set_border(5)
            .set_depth(depth)
            .set_class(WindowClasses::InputOutput)
            .set_visual(null_mut())
            .set_value_mask(xlib::CWBackPixel)
            .set_attributes(&mut attributes)
            .set_title("Hello World".to_string())
            .create(display.display(), root);

        // Hook close requests.
        let wm_protocols_str = CString::new("WM_PROTOCOLS").unwrap();
        let wm_delete_window_str = CString::new("WM_DELETE_WINDOW").unwrap();

        let wm_protocols = xlib::XInternAtom(display.display(), wm_protocols_str.as_ptr(), xlib::False);
        let wm_delete_window =
            xlib::XInternAtom(display.display(), wm_delete_window_str.as_ptr(), xlib::False);

        let mut protocols = [wm_delete_window];

        xlib::XSetWMProtocols(
            display.display(),
            window,
            protocols.as_mut_ptr(),
            protocols.len() as c_int,
        );

        let mut countret: c_int = 0;
        let fontpat: CString = CString::new("*").unwrap();
        let cfontlist: *mut *mut i8 =
            xlib::XListFonts(display.display(), fontpat.as_ptr(), 10000, &mut countret);
        println!("Total number of fonts={}", countret);
        // let fontslice = std::slice::from_raw_parts(cfontlist, countret as usize);
        // for part in fontslice {
        //     let f = CStr::from_ptr(*part);
        //     println!("{:?}", f);
        // }
        xlib::XFreeFontNames(cfontlist);

        // let font = xlib::XLoadQueryFont(display, "-bitstream-courier 10 pitch-bold-r-normal--0-0-200-200-m-0-iso8859-1".as_ptr() as *const i8);
        // let fontstr = CString::new("Monospaced").unwrap();
        // let font = xlib::XLoadQueryFont(display, fontstr.as_ptr());
        let fontname = CString::new("lucidasanstypewriter-bold-24").unwrap();
        let fontid = xlib::XLoadFont(display.display(), fontname.as_ptr());
        let font = xlib::XQueryFont(display.display(), fontid);
        println!(
            "FontID=0x{:02X} Properties=0x{:02X?}",
            (*font).fid,
            (*font).n_properties
        );
        if font.is_null() {
            println!("*** No font ***\n");
            panic!();
        }

        // Setup some graphic contexts for different colours

        let red_gc = GraphicContextBuilder::default()
            .set_background_colour(xlib::XWhitePixel(display.display(), screen))
            .set_foreground_colour(pixel_value_for_colour(display.display(), screen, "red"))
            .set_font_from_string(display.display(), "lucidasanstypewriter-bold-24")
            .create(display.display(), window);

        let green_gc = GraphicContextBuilder::default()
            .set_background_colour(xlib::XWhitePixel(display.display(), screen))
            .set_foreground_colour(pixel_value_for_colour(display.display(), screen, "green"))
            .set_font_from_string(display.display(), "lucidasanstypewriter-bold-24")
            .create(display.display(), window);

        let blue_gc = GraphicContextBuilder::default()
            .set_background_colour(xlib::XWhitePixel(display.display(), screen))
            .set_foreground_colour(pixel_value_for_colour(display.display(), screen, "purple"))
            .set_font_from_string(display.display(), "lucidasanstypewriter-bold-24")
            .create(display.display(), window);

        let white_gc = GraphicContextBuilder::default()
            .set_background_colour(xlib::XWhitePixel(display.display(), screen))
            .set_foreground_colour(xlib::XWhitePixel(display.display(), screen))
            .set_font_from_string(display.display(), "lucidasanstypewriter-bold-24")
            .create(display.display(), window);

        let black_gc = GraphicContextBuilder::default()
            .set_background_colour(xlib::XWhitePixel(display.display(), screen))
            .set_foreground_colour(xlib::XBlackPixel(display.display(), screen))
            .set_font_from_string(display.display(), "lucidasanstypewriter-bold-24")
            .create(display.display(), window);

        let mut gc = black_gc;

        // Show window.
        xlib::XMapWindow(display.display(), window);
        xlib::XSelectInput(
            display.display(),
            window,
            xlib::ExposureMask
                | xlib::KeyPressMask
                | xlib::KeyReleaseMask
                | xlib::ButtonPressMask
                | xlib::ButtonReleaseMask
                | xlib::PointerMotionMask,
        );

        // Main loop.
        let mut x: i32 = -1;
        let mut y: i32 = -1;
        let mut b: bool = false;
        let mut k: bool = false;
        let mut event: xlib::XEvent = mem::MaybeUninit::uninit().assume_init();
        loop {
            xlib::XNextEvent(display.display(), &mut event);

            let xtype: c_int = event.get_type();
            match xtype {
                xlib::ClientMessage => {
                    let xclient = xlib::XClientMessageEvent::from(event);

                    if xclient.message_type == wm_protocols && xclient.format == 32 {
                        let protocol = xclient.data.get_long(0) as xlib::Atom;

                        if protocol == wm_delete_window {
                            break;
                        }
                    } else {
                        println!("Other msg");
                    }
                }
                xlib::KeyPress => {
                    let xkey = xlib::XKeyEvent::from(event);
                    let xsym = xlib::XKeycodeToKeysym(display.display(), xkey.keycode.try_into().unwrap(), 0);
                    let xstr = std::ffi::CStr::from_ptr(xlib::XKeysymToString(xsym))
                        .to_owned()
                        .into_string()
                        .unwrap();
                    if xsym != xlib::NoSymbol.try_into().unwrap() {
                        println!("Key {} {} {}", xkey.keycode, xsym, xstr);
                        if xsym == 32 {
                            k = true;
                        }
                    }
                }
                xlib::KeyRelease => {
                    let xkey = xlib::XKeyEvent::from(event);
                    let xsym = xlib::XKeycodeToKeysym(display.display(), xkey.keycode.try_into().unwrap(), 0);
                    if xsym != xlib::NoSymbol.try_into().unwrap() {
                        if xsym == 113 {
                            return;
                        } else if xsym == 32 {
                            k = false;
                        }
                    }
                }
                xlib::ButtonPress => {
                    if x != -1 && y != -1 {
                        xlib::XDrawString(
                            display.display(),
                            window,
                            white_gc,
                            x,
                            y,
                            CString::new("Hello")
                                .unwrap()
                                .into_bytes_with_nul()
                                .as_ptr() as *const i8,
                            5,
                        );
                    }
                    let xbutton = xlib::XButtonEvent::from(event);
                    if xbutton.button == 1 {
                        gc = red_gc;
                    } else if xbutton.button == 3 {
                        gc = green_gc;
                    } else {
                        if gc == blue_gc {
                            gc = black_gc;
                        } else {
                            gc = blue_gc;
                        }
                    }

                    x = xbutton.x;
                    y = xbutton.y;
                    b = true;
                    println!("Button {} {} {}", xbutton.button, x, y);
                    xlib::XDrawString(
                        display.display(),
                        window,
                        gc,
                        x,
                        y,
                        CString::new("Hello")
                            .unwrap()
                            .into_bytes_with_nul()
                            .as_ptr() as *const i8,
                        5,
                    );
                }
                xlib::ButtonRelease => {
                    b = false;
                    xlib::XDrawString(
                        display.display(),
                        window,
                        white_gc,
                        x,
                        y,
                        CString::new("Hello")
                            .unwrap()
                            .into_bytes_with_nul()
                            .as_ptr() as *const i8,
                        5,
                    );
                    x = -1;
                    y = -1;
                    // println!(" clicked");
                }
                xlib::MotionNotify => {
                    let xmotion = xlib::XMotionEvent::from(event);
                    if b {
                        if x != -1 && y != -1 {
                            xlib::XDrawString(
                                display.display(),
                                window,
                                white_gc,
                                x,
                                y,
                                CString::new("Hello")
                                    .unwrap()
                                    .into_bytes_with_nul()
                                    .as_ptr() as *const i8,
                                5,
                            );
                        }
                        x = xmotion.x;
                        y = xmotion.y;
                        xlib::XDrawString(
                            display.display(),
                            window,
                            gc,
                            x,
                            y,
                            CString::new("Hello")
                                .unwrap()
                                .into_bytes_with_nul()
                                .as_ptr() as *const i8,
                            5,
                        );
                    }
                    if k {
                        xlib::XDrawPoint(display.display(), window, black_gc, x, y);
                    } else {
                        xlib::XDrawPoint(display.display(), window, white_gc, x, y);
                    }
                }
                xlib::Expose => {
                    println!("Exposed");
                }
                _ => {
                    println!("{}", xtype);
                }
            }
        }

        // Shut down.
        // xlib::XCloseDisplay(display.display());
    }
}
