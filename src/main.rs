extern crate num;
extern crate x11;
#[macro_use]
extern crate num_derive;

use std::ffi::CString;
use std::mem::{self, MaybeUninit};
use std::os::raw::c_char;
use std::os::raw::*;
use std::ptr::{self, null_mut};
use x11::xlib;

//=================================================================================================================================
//
//
// _XDisplay structure from https://xwindow.angelfire.com/page28.html
// struct _XDisplay
// {
// 	XExtData *ext_data;	/* hook for extension to hang data */
// 	struct _XFreeFuncs *free_funcs; /* internal free functions */
// 	int fd;			/* Network socket. */
// 	int conn_checker;         /* ugly thing used by _XEventsQueued */
// 	int proto_major_version;/* maj. version of server's X protocol */
// 	int proto_minor_version;/* minor version of server's X protocol */
// 	char *vendor;		/* vendor of the server hardware */
//         XID resource_base;	/* resource ID base */
// 	XID resource_mask;	/* resource ID mask bits */
// 	XID resource_id;	/* allocator current ID */
// 	int resource_shift;	/* allocator shift to correct bits */
// 	XID (*resource_alloc)(	/* allocator function */
// 		struct _XDisplay*
// 		);
// 	int byte_order;		/* screen byte order, LSBFirst, MSBFirst */
// 	int bitmap_unit;	/* padding and data requirements */
// 	int bitmap_pad;		/* padding requirements on bitmaps */
// 	int bitmap_bit_order;	/* LeastSignificant or MostSignificant */
// 	int nformats;		/* number of pixmap formats in list */
// 	ScreenFormat *pixmap_format;	/* pixmap format list */
// 	int vnumber;		/* Xlib's X protocol version number. */
// 	int release;		/* release of the server */
// 	struct _XSQEvent *head, *tail;	/* Input event queue. */
// 	int qlen;		/* Length of input event queue */
// 	unsigned long last_request_read; /* seq number of last event read */
// 	unsigned long request;	/* sequence number of last request. */
// 	char *last_req;		/* beginning of last request, or dummy */
// 	char *buffer;		/* Output buffer starting address. */
// 	char *bufptr;		/* Output buffer index pointer. */
// 	char *bufmax;		/* Output buffer maximum+1 address. */
// 	unsigned max_request_size; /* maximum number 32 bit words in request*/
// 	struct _XrmHashBucketRec *db;
// 	int (*synchandler)(	/* Synchronization handler */
// 		struct _XDisplay*
// 		);
// 	char *display_name;	/* "host:display" string used on this connect*/
// 	int default_screen;	/* default screen for operations */
// 	int nscreens;		/* number of screens on this server*/
// 	Screen *screens;	/* pointer to list of screens */
// 	unsigned long motion_buffer;	/* size of motion buffer */
// 	unsigned long flags;	   /* internal connection flags */
// 	int min_keycode;	/* minimum defined keycode */
// 	int max_keycode;	/* maximum defined keycode */
// 	KeySym *keysyms;	/* This server's keysyms */
// 	XModifierKeymap *modifiermap;	/* This server's modifier keymap */
// 	int keysyms_per_keycode;/* number of rows */
// 	char *xdefaults;	/* contents of defaults from server */
// 	char *scratch_buffer;	/* place to hang scratch buffer */
// 	unsigned long scratch_length;	/* length of scratch buffer */
// 	int ext_number;		/* extension number on this display */
// 	struct _XExten *ext_procs; /* extensions initialized on this display */
// 	/*
// 	 * the following can be fixed size, as the protocol defines how
// 	 * much address space is available.
// 	 * While this could be done using the extension vector, there
// 	 * may be MANY events processed, so a search through the extension
// 	 * list to find the right procedure for each event might be
// 	 * expensive if many extensions are being used.
// 	 */
// 	Bool (*event_vec[128])(	/* vector for wire to event */
// 		Display *	/* dpy */,
// 		XEvent *	/* re */,
// 		xEvent *	/* event */
// 		);
// 	Status (*wire_vec[128])( /* vector for event to wire */
// 		Display *	/* dpy */,
// 		XEvent *	/* re */,
// 		xEvent *	/* event */
// 		);
// 	KeySym lock_meaning;	   /* for XLookupString */
// 	struct _XLockInfo *lock;   /* multi-thread state, display lock */
// 	struct _XInternalAsync *async_handlers; /* for internal async */
// 	unsigned long bigreq_size; /* max size of big requests */
// 	struct _XLockPtrs *lock_fns; /* pointers to threads functions */
// 	void (*idlist_alloc)(	   /* XID list allocator function */
// 		Display *	/* dpy */,
// 		XID *		/* ids */,
// 		int		/* count */
// 		);
// 	/* things above this line should not move, for binary compatibility */
// 	struct _XKeytrans *key_bindings; /* for XLookupString */
// 	Font cursor_font;	   /* for XCreateFontCursor */
// 	struct _XDisplayAtoms *atoms; /* for XInternAtom */
// 	unsigned int mode_switch;  /* keyboard group modifiers */
// 	unsigned int num_lock;  /* keyboard numlock modifiers */
// 	struct _XContextDB *context_db; /* context database */
// 	Bool (**error_vec)(	/* vector for wire to error */
// 		Display     *	/* display */,
// 		XErrorEvent *	/* he */,
// 		xError      *	/* we */
// 		);
// 	/*
// 	 * Xcms information
// 	 */
// 	struct {
// 	   XPointer defaultCCCs;  /* pointer to an array of default XcmsCCC */
// 	   XPointer clientCmaps;  /* pointer to linked list of XcmsCmapRec */
// 	   XPointer perVisualIntensityMaps;
// 				  /* linked list of XcmsIntensityMap */
// 	} cms;
// 	struct _XIMFilter *im_filters;
// 	struct _XSQEvent *qfree; /* unallocated event queue elements */
// 	unsigned long next_event_serial_num; /* inserted into next queue elt */
// 	struct _XExten *flushes; /* Flush hooks */
// 	struct _XConnectionInfo *im_fd_info; /* _XRegisterInternalConnection */
// 	int im_fd_length;	/* number of im_fd_info */
// 	struct _XConnWatchInfo *conn_watchers; /* XAddConnectionWatch */
// 	int watcher_count;	/* number of conn_watchers */
// 	XPointer filedes;	/* struct pollfd cache for _XWaitForReadable */
// 	int (*savedsynchandler)( /* user synchandler when Xlib usurps */
// 		Display *	/* dpy */
// 		);
// 	XID resource_max;	/* allocator max ID */
// 	int xcmisc_opcode;	/* major opcode for XC-MISC */
// 	struct _XkbInfoRec *xkb_info; /* XKB info */
// 	struct _XtransConnInfo *trans_conn; /* transport connection object */
// };

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
struct XDisplay_cms {
    defaultcccs: *mut c_char, /* pointer to an array of default XcmsCCC */
    clientcmaps: *mut c_char, /* pointer to linked list of XcmsCmapRec */
    pervisualintensitymaps: *mut c_char,
    /* linked list of XcmsIntensityMap */
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
struct XDisplay {
    ext_data: *const c_void,       /* hook for extension to hang data */
    free_funcs: *const c_void,     /* internal free functions */
    fd: c_int,                     /* Network socket. */
    conn_checker: c_int,           /* ugly thing used by _XEventsQueued */
    proto_major_version: c_int,    /* maj. version of server's X protocol */
    proto_minor_version: c_int,    /* minor version of server's X protocol */
    vendor: *const c_char,         /* vendor of the server hardware */
    resource_base: c_ulong,        /* resource ID base */
    resource_mask: c_ulong,        /* resource ID mask bits */
    resource_id: c_ulong,          /* allocator current ID */
    resource_shift: c_int,         /* allocator shift to correct bits */
    resource_alloc: *const c_void, /* allocator function */
    byte_order: c_int,             /* screen byte order, LSBFirst, MSBFirst */
    bitmap_unit: c_int,            /* padding and data requirements */
    bitmap_pad: c_int,             /* padding requirements on bitmaps */
    bitmap_bit_order: c_int,       /* LeastSignificant or MostSignificant */
    nformats: c_int,               /* number of pixmap formats in list */
    pixmap_format: *const c_void,  /* pixmap format list */
    vnumber: c_int,                /* Xlib's X protocol version number. */
    release: c_int,                /* release of the server */
    head: *const c_void,
    tail: *const c_void,        /* Input event queue. */
    qlen: c_int,                /* Length of input event queue */
    last_request_read: c_ulong, /* seq number of last event read */
    request: c_ulong,           /* sequence number of last request. */
    last_req: *const i8,        /* beginning of last request, or dummy */
    buffer: *const i8,          /* Output buffer starting address. */
    bufptr: *const i8,          /* Output buffer index pointer. */
    bufmax: *const i8,          /* Output buffer maximum+1 address. */
    max_request_size: c_uint,   /* maximum number 32 bit words in request*/
    db: *const c_void,
    synchandler: *const c_void,   /* Synchronization handler */
    display_name: *const i8,      /* "host:display" string used on this connect*/
    default_screen: c_int,        /* default screen for operations */
    nscreens: c_int,              /* number of screens on this server*/
    screens: *const xlib::Screen, /* pointer to list of screens */
    motion_buffer: c_ulong,       /* size of motion buffer */
    flags: c_ulong,               /* internal connection flags */
    min_keycode: c_int,           /* minimum defined keycode */
    max_keycode: c_int,           /* maximum defined keycode */
    keysyms: *const c_void,       /* This server's keysyms */
    modifiermap: *const c_void,   /* This server's modifier keymap */
    keysyms_per_keycode: c_int,   /* number of rows */
    xdefaults: *const i8,         /* contents of defaults from server */
    scratch_buffer: *const i8,    /* place to hang scratch buffer */
    scratch_length: c_ulong,      /* length of scratch buffer */
    ext_number: c_int,            /* extension number on this display */
    ext_procs: *const c_void,     /* extensions initialized on this display */
    /*
     * the following can be fixed size, as the protocol defines how
     * much address space is available.
     * While this could be done using the extension vector, there
     * may be MANY events processed, so a search through the extension
     * list to find the right procedure for each event might be
     * expensive if many extensions are being used.
     */
    event_vec: [*const c_void; 128], /* vector for wire to event */
    wire_vec: [*const c_void; 128],  /* vector for event to wire */
    lock_meaning: c_ulong,           /* for XLookupString */
    lock: *const c_void,             /* multi-thread state, display lock */
    async_handlers: *const c_void,   /* for internal async */
    bigreq_size: c_ulong,            /* max size of big requests */
    lock_fns: *const c_void,         /* pointers to threads functions */
    idlist_alloc: *const c_void,     /* XID list allocator function */
    /* things above this line should not move, for binary compatibility */
    key_bindings: *const c_void,     /* for XLookupString */
    cursor_font: c_ulong,            /* for XCreateFontCursor */
    atoms: *const c_void,            /* for XInternAtom */
    mode_switch: c_uint,             /* keyboard group modifiers */
    num_lock: c_uint,                /* keyboard numlock modifiers */
    context_db: *const c_void,       /* context database */
    error_vec: *const *const c_void, /* vector for wire to error */
    /*
     * Xcms information
     */
    cms: XDisplay_cms,
    im_filters: *const c_void,
    qfree: *const c_void,            /* unallocated event queue elements */
    next_event_serial_num: c_ulong,  /* inserted into next queue elt */
    flushes: *const c_void,          /* Flush hooks */
    im_fd_info: *const c_void,       /* _XRegisterInternalConnection */
    im_fd_length: c_int,             /* number of im_fd_info */
    conn_watchers: *const c_void,    /* XAddConnectionWatch */
    watcher_count: c_int,            /* number of conn_watchers */
    filedes: *mut c_char,            /* struct pollfd cache for _XWaitForReadable */
    savedsynchandler: *const c_void, /* user synchandler when Xlib usurps */
    resource_max: c_ulong,           /* allocator max ID */
    xcmisc_opcode: c_int,            /* major opcode for XC-MISC */
    xkb_info: *const c_void,         /* XKB info */
    trans_conn: *const c_void,       /* transport connection object */
}

//=================================================================================================================================
//
// Graphic functions
//
#[derive(Copy, Clone, PartialEq, Debug)]
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

//=================================================================================================================================
//
// LineStyle
//
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(dead_code)]
enum GraphicContextLineStyles {
    LineSolid = 0,
    LineOnOffDash = 1,
    LineDoubleDash = 2,
}

// capStyle
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(dead_code)]
enum GraphicContextCapStyles {
    CapNotLast = 0,
    CapButt = 1,
    CapRound = 2,
    CapProjecting = 3,
}

//=================================================================================================================================
//
// joinStyle
//
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(dead_code)]
enum GraphicContextJoinStyles {
    JoinMiter = 0,
    JoinRound = 1,
    JoinBevel = 2,
}

//=================================================================================================================================
//
// fillStyle
//
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(dead_code)]
enum GraphicContextFillStyles {
    FillSolid = 0,
    FillTiled = 1,
    FillStippled = 2,
    FillOpaqueStippled = 3,
}

//=================================================================================================================================
//
// fillRule
//
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(dead_code)]
enum GraphicContextFillRules {
    EvenOddRule = 0,
    WindingRule = 1,
}

//=================================================================================================================================
//
// Arc modes for PolyFillArc
//
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(dead_code)]
enum GraphicContextArcModes {
    ArcChord = 0,
    ArcPieSlice = 1,
}

//=================================================================================================================================
//
// subwindow mode
//
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(dead_code)]
enum GraphicContextSubWindowModes {
    ClipByChildren = 0,
    IncludeInferiors = 1,
}

//=================================================================================================================================
//
// Graphic exposure
//
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(dead_code)]
enum GraphicContextGraphicExposure {
    CopyArea = 0,
    CopyPlane = 1,
}

//=================================================================================================================================
//
// window classes
//
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(dead_code)]
enum WindowClasses {
    InputOutput = 1,
    InputOnly = 2,
}

//#################################################################################################################################
//
// Error codes
//
// To convert from an integer to the enum use the following trick which makes use of the num and num_derive crates.
//      let error_code:ErrorCodes = num::FromPrimitive::from_i32(xlib::XFunctionThatReturns_i32).unwrap();
//
// The fmt function of the Display trait is implemented to allow the human readable form of the error code to be output.
//      println!("Result={}", error_code);
//
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
#[allow(dead_code)]
enum ErrorCodes {
    Success = 0,
    BadRequest = 1,
    BadValue = 2,
    BadWindow = 3,
    BadPixmap = 4,
    BadAtom = 5,
    BadCursor = 6,
    BadFont = 7,
    BadMatch = 8,
    BadDrawable = 9,
    BadAccess = 10,
    BadAlloc = 11,
    BadColor = 12,
    BadGC = 13,
    BadIDChoice = 14,
    BadName = 15,
    BadLength = 16,
    BadImplementation = 17,
    FirstExtensionError = 128,
    LastExtensionError = 255,
}

impl std::fmt::Display for ErrorCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorCodes::Success => write!(f, "Success"),
            ErrorCodes::BadRequest => write!(f, "BadRequest"),
            ErrorCodes::BadValue => write!(f, "BadValue"),
            ErrorCodes::BadWindow => write!(f, "BadWindow"),
            ErrorCodes::BadPixmap => write!(f, "BadPixmap"),
            ErrorCodes::BadAtom => write!(f, "BadAtom"),
            ErrorCodes::BadCursor => write!(f, "BadCursor"),
            ErrorCodes::BadFont => write!(f, "BadFont"),
            ErrorCodes::BadMatch => write!(f, "BadMatch"),
            ErrorCodes::BadDrawable => write!(f, "BadRequeBadDrawablest"),
            ErrorCodes::BadAccess => write!(f, "BadAccess"),
            ErrorCodes::BadAlloc => write!(f, "BadAlloc"),
            ErrorCodes::BadColor => write!(f, "BadColor"),
            ErrorCodes::BadGC => write!(f, "BadGC"),
            ErrorCodes::BadIDChoice => write!(f, "BadIDChoice"),
            ErrorCodes::BadName => write!(f, "BadName"),
            ErrorCodes::BadLength => write!(f, "BadLength"),
            ErrorCodes::BadImplementation => write!(f, "BadImplementation"),
            _ => write!(f, ""),
        }
    }
}

//#################################################################################################################################
//
// Structure that maps to the C structure in X11 as closely as possible
//
#[derive(Copy, Clone, PartialEq, Debug)]
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

impl Default for GraphicContextComponents {
    fn default() -> Self {
        GraphicContextComponents {
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
        }
    }
}

//#################################################################################################################################
//
// Graphic context builder
//
// Sets up the arguments for the XCreateGC call allowing default values for arguments to be used without having to specify all
// the arguments. Some components must be specified such as the font. The default function is called first and any arguments that
// need to be set can be chained of it. The last function in the chain must be create.
//
// Example:
// The following uses keeps everything at default except the foreground and background colours.
//
//      let black_on_white_gc = GraphicContextBuilder::default()
//          .set_background_colour(xlib::XWhitePixel(display.display(), display.screen()))
//          .set_foreground_colour(xlib::XBlackPixel(display.display(), display.screen()))
//          .set_font(fontinfo.id())
//          .create(display.display(), window);

//
#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(dead_code)]
struct GraphicContextBuilder {
    components: GraphicContextComponents,
    used: u32,
    must_exist: u32,
}

impl Default for GraphicContextBuilder {
    fn default() -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents::default(),
            used: 0,
            must_exist: xlib::GCFont,
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
            used: self.used | xlib::GCLineWidth,
            ..self
        }
    }

    pub fn set_line_style(self, line_style: GraphicContextLineStyles) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                line_style: line_style as i32,
                ..self.components
            },
            used: self.used | xlib::GCLineStyle,
            ..self
        }
    }

    pub fn set_cap_style(self, cap_style: GraphicContextCapStyles) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                cap_style: cap_style as i32,
                ..self.components
            },
            used: self.used | xlib::GCCapStyle,
            ..self
        }
    }

    pub fn set_join_style(self, join_style: GraphicContextJoinStyles) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                join_style: join_style as i32,
                ..self.components
            },
            used: self.used | xlib::GCJoinStyle,
            ..self
        }
    }

    pub fn set_fill_style(self, fill_style: GraphicContextFillStyles) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                fill_style: fill_style as i32,
                ..self.components
            },
            used: self.used | xlib::GCFillStyle,
            ..self
        }
    }

    pub fn set_fill_rule(self, fill_rule: GraphicContextFillStyles) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                fill_rule: fill_rule as i32,
                ..self.components
            },
            used: self.used | xlib::GCFillRule,
            ..self
        }
    }

    pub fn set_arc_mode(self, arc_mode: GraphicContextArcModes) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                arc_mode: arc_mode as i32,
                ..self.components
            },
            used: self.used | xlib::GCArcMode,
            ..self
        }
    }

    pub fn set_tile(self, tile: u64) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                tile: tile,
                ..self.components
            },
            used: self.used | xlib::GCTile,
            ..self
        }
    }

    pub fn set_stipple(self, stipple: u64) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                stipple: stipple,
                ..self.components
            },
            used: self.used | xlib::GCStipple,
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
            used: self.used | xlib::GCTileStipXOrigin | xlib::GCTileStipYOrigin,
            ..self
        }
    }

    pub fn set_font(self, font: xlib::Font) -> Self {
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
            used: self.used | xlib::GCSubwindowMode,
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
            used: self.used | xlib::GCGraphicsExposures,
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
            used: self.used | xlib::GCClipXOrigin | xlib::GCClipYOrigin,
            ..self
        }
    }

    pub fn set_clip_mask(self, clip_mask: u64) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                clip_mask,
                ..self.components
            },
            used: self.used | xlib::GCClipMask,
            ..self
        }
    }

    pub fn set_dash_offset(self, dash_offset: c_int) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                dash_offset,
                ..self.components
            },
            used: self.used | xlib::GCDashOffset,
            ..self
        }
    }

    pub fn set_dashes(self, dashes: c_char) -> Self {
        GraphicContextBuilder {
            components: GraphicContextComponents {
                dashes,
                ..self.components
            },
            used: self.used | xlib::GCDashList,
            ..self
        }
    }

    pub fn create(self, display: *mut xlib::Display, window: xlib::Window) -> xlib::GC {
        if self.used & self.must_exist == 0 {
            panic!("Required graphic context components not specified");
        }
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

            let gc = xlib::XCreateGC(display, window, self.used as u64, &mut xgcvalue);
            gc
        }
    }
}

//#################################################################################################################################
//
// Window attributes and its builder
//
// The attributes are used when a window is created. The XCreateWindow uses two arguments to set the attributes, a flag and a
// structure with the flag indicating which fields in the structure are actually in use. The window attribute builder will set
// the flag and the appropriate field in the structure. The two components will then be passed as seperate arguments when the
// window is created.
//
#[allow(dead_code, non_snake_case)]
pub mod WindowAttributes {
    use std::ffi::c_ulong;

    pub const CWBACK_PIXMAP: c_ulong = 0x0001;
    pub const CWBACK_PIXEL: c_ulong = 0x0002;
    pub const CWBORDER_PIXMAP: c_ulong = 0x0004;
    pub const CWBORDER_PIXEL: c_ulong = 0x0008;
    pub const CWBIT_GRAVITY: c_ulong = 0x0010;
    pub const CWWIN_GRAVITY: c_ulong = 0x0020;
    pub const CWBACKING_STORE: c_ulong = 0x0040;
    pub const CWBACKING_PLANES: c_ulong = 0x0080;
    pub const CWBACKING_PIXEL: c_ulong = 0x0100;
    pub const CWOVERRIDE_REDIRECT: c_ulong = 0x0200;
    pub const CWSAVE_UNDER: c_ulong = 0x0400;
    pub const CWEVENT_MASK: c_ulong = 0x0800;
    pub const CWDONT_PROPAGATE: c_ulong = 0x1000;
    pub const CWCOLORMAP: c_ulong = 0x2000;
    pub const CWCURSOR: c_ulong = 0x4000;
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[allow(dead_code)]
struct WindowAttributeBuilder {
    mask: c_ulong,
    attributes: xlib::XSetWindowAttributes,
}

impl Default for WindowAttributeBuilder {
    fn default() -> Self {
        WindowAttributeBuilder {
            mask: num::FromPrimitive::from_i32(0).unwrap(),
            attributes: xlib::XSetWindowAttributes {
                background_pixmap: 0,
                background_pixel: 0,
                border_pixmap: 0,
                border_pixel: 0,
                bit_gravity: 0,
                win_gravity: 0,
                backing_store: 0,
                backing_planes: 0,
                backing_pixel: 0,
                save_under: 0,
                event_mask: 0,
                do_not_propagate_mask: 0,
                override_redirect: 0,
                colormap: 0,
                cursor: 0,
            },
        }
    }
}

#[allow(dead_code)]
impl WindowAttributeBuilder {
    // let mut attributes: MaybeUninit<xlib::XSetWindowAttributes> =
    // MaybeUninit::uninit().assume_init();
    // let attr_ptr = attributes.as_mut_ptr();
    // std::ptr::addr_of_mut!((*attr_ptr).background_pixel)
    // .write(xlib::XWhitePixel(display.display(), display.screen()));
    // let mut attributes = attributes.assume_init();

    fn back_pixmap(&mut self, back_pixmap: u64) -> Self {
        self.mask = self.mask | WindowAttributes::CWBACK_PIXMAP;
        self.attributes.background_pixmap = back_pixmap;
        *self
    }

    fn back_pixel(&mut self, back_pixel: u64) -> Self {
        self.mask = self.mask | WindowAttributes::CWBACK_PIXEL;
        self.attributes.background_pixel = back_pixel;
        *self
    }

    fn border_pixmap(&mut self, border_pixmap: u64) -> Self {
        self.mask = self.mask | WindowAttributes::CWBORDER_PIXMAP;
        self.attributes.border_pixmap = border_pixmap;
        *self
    }

    fn border_pixel(&mut self, border_pixel: u64) -> Self {
        self.mask = self.mask | WindowAttributes::CWBORDER_PIXEL;
        self.attributes.border_pixel = border_pixel;
        *self
    }

    fn bit_gravity(&mut self, bit_gravity: i32) -> Self {
        self.mask = self.mask | WindowAttributes::CWBIT_GRAVITY;
        self.attributes.bit_gravity = bit_gravity;
        *self
    }

    fn win_gravity(&mut self, win_gravity: i32) -> Self {
        self.mask = self.mask | WindowAttributes::CWWIN_GRAVITY;
        self.attributes.win_gravity = win_gravity;
        *self
    }

    fn backing_store(&mut self, backing_store: i32) -> Self {
        self.mask = self.mask | WindowAttributes::CWBACKING_STORE;
        self.attributes.backing_store = backing_store;
        *self
    }

    fn backing_planes(&mut self, backing_planes: u64) -> Self {
        self.mask = self.mask | WindowAttributes::CWBACKING_PLANES;
        self.attributes.backing_planes = backing_planes;
        *self
    }

    fn backing_pixel(&mut self, backing_pixel: u64) -> Self {
        self.mask = self.mask | WindowAttributes::CWBACKING_PIXEL;
        self.attributes.backing_pixel = backing_pixel;
        *self
    }

    fn save_under(&mut self, save_under: i32) -> Self {
        self.mask = self.mask | WindowAttributes::CWSAVE_UNDER;
        self.attributes.save_under = save_under;
        *self
    }

    fn event_mask(&mut self, event_mask: i64) -> Self {
        self.mask = self.mask | WindowAttributes::CWEVENT_MASK;
        self.attributes.event_mask = event_mask;
        *self
    }

    fn do_not_propagate_mask(&mut self, do_not_propagate_mask: i64) -> Self {
        self.mask = self.mask | WindowAttributes::CWDONT_PROPAGATE;
        self.attributes.do_not_propagate_mask = do_not_propagate_mask;
        *self
    }

    fn override_redirect(&mut self, override_redirect: i32) -> Self {
        self.mask = self.mask | WindowAttributes::CWOVERRIDE_REDIRECT;
        self.attributes.override_redirect = override_redirect;
        *self
    }

    fn color_map(&mut self, color_map: u64) -> Self {
        self.mask = self.mask | WindowAttributes::CWCOLORMAP;
        self.attributes.colormap = color_map;
        *self
    }

    fn cursor(&mut self, cursor: u64) -> Self {
        self.mask = self.mask | WindowAttributes::CWCURSOR;
        self.attributes.cursor = cursor;
        *self
    }
}

//#################################################################################################################################
//
// Window builder
//
// Sets up the arguments for the XCreateWindow call allowing default values for arguments to be used without having to specify all
// the arguments. The default function is called first and any arguments that need to be set can be chained of it. The last function
// in the chain must be create.
//
// Technically, the title is not part of the XCreateWindow call. But it's so closely linked to the creation of a window that its
// appropriate to include it in the builder's methods.
//
// Example:
// The following uses a default border width of 5 and default class of InputOutput.
//
//  let window = WindowBuilder::default()
//     .set_origin(0, 0)
//     .set_size(400, 300)
//     .set_depth(display.depth())
//     .set_visual(null_mut())
//     .set_value_mask(xlib::CWBackPixel)
//     .set_attributes(&mut attributes)
//     .set_title("Hello World".to_string())
//     .create(display.display(), display.root_window());
//
#[derive(Clone, PartialEq, Debug)]
#[allow(dead_code)]
struct WindowBuilder {
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
        WindowBuilder {
            class: class as c_int,
            ..self
        }
    }

    pub fn set_visual(self, visual: *mut xlib::Visual) -> Self {
        WindowBuilder { visual, ..self }
    }

    pub fn set_attributes(mut self, mut attributes: WindowAttributeBuilder) -> Self {
        self.value_mask = attributes.mask;
        self.attributes = &mut attributes.attributes;
        self
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
                xlib::XStoreName(display, window, self.title.as_ptr() as *mut c_char);
            }
            window
        }
    }
}

//#################################################################################################################################
//
// Display maniuplation
//
// Allows opening the display with automatic closing of it when it goes out of scope. Some useful functions tacked on.
//
#[derive(Clone, PartialEq, Debug)]
#[allow(dead_code)]
struct Display {
    display: *mut xlib::Display,
    screen: i32,
    root_window: u64,
    depth: i32,
}

#[allow(dead_code)]
impl Display {
    fn new() -> Self {
        println!("Display opened");
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());
            if display.is_null() {
                panic!("XOpenDisplay failed");
            }

            println!("display={:p}", display);

            let txd = display as *const c_void;
            let xd = txd as *const XDisplay;
            println!(
                "XDisplay vendor is {:?}",
                std::ffi::CStr::from_ptr((*xd).vendor)
            );
            println!(
                "XDisplay display_name is {:?}",
                std::ffi::CStr::from_ptr((*xd).display_name)
            );
            println!("XDisplay default_screen is {:?}", (*xd).default_screen);
            println!("XDisplay nscreens is {:?}", (*xd).nscreens);
            println!(
                "XDisplay screen 0 {} x {}",
                (*(*xd).screens).width,
                (*(*xd).screens).height
            );

            let screen = xlib::XDefaultScreen(display);
            let root_window = xlib::XRootWindow(display, screen);
            let depth = xlib::XDefaultDepth(display, screen);

            Display {
                display,
                screen,
                root_window,
                depth,
            }
        }
    }

    fn display(&mut self) -> *mut xlib::Display {
        self.display
    }

    fn screen(&mut self) -> i32 {
        self.screen
    }

    fn root_window(&mut self) -> u64 {
        self.root_window
    }

    fn depth(&mut self) -> i32 {
        self.depth
    }

    fn print_dimensions(&self) {
        unsafe {
            println!(
                "{} x {}",
                xlib::XDisplayWidth(self.display, self.screen),
                xlib::XDisplayHeight(self.display, self.screen)
            );
            println!(
                "{}mm x {}mm",
                xlib::XDisplayWidthMM(self.display, self.screen),
                xlib::XDisplayHeightMM(self.display, self.screen)
            );
        }
    }

    fn get_selection_owner(&self, selection: String) -> String {
        unsafe {
            let string = format!(
                "{} 0x{:02X}",
                selection,
                xlib::XGetSelectionOwner(
                    self.display,
                    xlib::XInternAtom(
                        self.display,
                        CString::new(selection.as_bytes())
                            .unwrap()
                            .to_owned()
                            .as_ptr(),
                        0
                    )
                )
            );
            string
        }
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe {
            let error_code:ErrorCodes = num::FromPrimitive::from_i32(xlib::XCloseDisplay(self.display)).unwrap();
            if error_code != ErrorCodes::Success {
                panic!("XCloseDisplay returned {}", error_code);
            }
        }
    }
}

//#################################################################################################################################
//
// Font list
//
// Handle the creation and allocation of the font list and implement automatic release of the memory used by the font list when the
// variable goes out of scope.
//
#[derive(Clone, PartialEq, Debug)]
#[allow(dead_code)]
struct FontList {
    cfontlist: *mut *mut i8,
    count: c_int,
}

#[allow(dead_code)]
impl FontList {
    fn new(display: &mut Display) -> Self {
        unsafe {
            let mut count: c_int = 0;
            let fontpat: CString = CString::new("*").unwrap();
            let cfontlist: *mut *mut i8 =
                xlib::XListFonts(display.display(), fontpat.as_ptr(), 10000, &mut count);
            FontList { cfontlist, count }
        }
    }

    fn count(&self) -> i32 {
        self.count
    }

    fn list(&self) -> Vec<String> {
        let mut list = Vec::new();

        unsafe {
            let fontslice = std::slice::from_raw_parts(self.cfontlist, self.count as usize);
            for part in fontslice {
                let name = std::ffi::CStr::from_ptr(*part);
                // println!("{:?}", f);
                list.push(name.to_str().unwrap().to_owned());
            }
        }

        list
    }
}

impl Drop for FontList {
    fn drop(&mut self) {
        unsafe {
            // xlib::XFreeFontNames(self.cfontlist);
            if xlib::XFreeFontNames(self.cfontlist) == 0 {
                panic!("XFreeFontNames returned 0 ");
            }
        }
    }
}

//#################################################################################################################################
//
// Font structure
//
// Handle the creation and allocation of the font structure and implement automatic release of the memory used by it when the
// variable goes out of scope.
//
#[derive(Clone, PartialEq, Debug)]
#[allow(dead_code)]
struct Font {
    display: *mut Display,
    font: *mut xlib::XFontStruct,
}

#[allow(dead_code)]
impl Font {
    fn new(display: &mut Display, name: String) -> Self {
        unsafe {
            let font =
                xlib::XLoadQueryFont(display.display(), name.as_bytes().as_ptr() as *const i8);
            if font.is_null() {
                panic!("*** No font {} found ***\n", name);
            }
            Font { display, font }
        }
    }

    fn font(&self) -> *mut xlib::XFontStruct {
        self.font
    }

    fn id(&self) -> c_ulong {
        unsafe { (*self.font).fid }
    }

    fn properties(&self) -> i32 {
        unsafe { (*self.font).n_properties }
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe {
            if xlib::XFreeFont(
                (*(self.display)).display(),
                self.font,
            ) == 0 {
                panic!("XFreeFont returned 0");
            }
        }
    }
}
//=================================================================================================================================
//
// Get the pixel value for a named colour.
//
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

fn main() {
    unsafe {
        let mut display = Display::new();

        xlib::XSynchronize(display.display(), 0);

        println!("{}", display.get_selection_owner("PRIMARY".to_string()));
        println!("{}", display.get_selection_owner("SECONDARY".to_string()));
        println!("{}", display.get_selection_owner("CLIPBOARD".to_string()));
        println!("{}", display.get_selection_owner("FOOBAR".to_string()));

        display.print_dimensions();

        // Create the window
        let window = WindowBuilder::default()
            .set_origin(0, 0)
            .set_size(400, 300)
            .set_border(5)
            .set_depth(display.depth())
            .set_class(WindowClasses::InputOutput)
            .set_visual(null_mut())
            .set_attributes(
                WindowAttributeBuilder::default()
                    .back_pixel(xlib::XWhitePixel(display.display(), display.screen())),
            )
            .set_title("Hello World".to_string())
            .create(display.display(), display.root_window());

        // Hook close requests.
        let wm_protocols_str = CString::new("WM_PROTOCOLS").unwrap();
        let wm_delete_window_str = CString::new("WM_DELETE_WINDOW").unwrap();

        let wm_protocols =
            xlib::XInternAtom(display.display(), wm_protocols_str.as_ptr(), xlib::False);
        let wm_delete_window = xlib::XInternAtom(
            display.display(),
            wm_delete_window_str.as_ptr(),
            xlib::False,
        );

        let mut protocols = [wm_delete_window];

        xlib::XSetWMProtocols(
            display.display(),
            window,
            protocols.as_mut_ptr(),
            protocols.len() as c_int,
        );

        let fontlist = FontList::new(&mut display);
        println!("Number of fonts installed is {}", fontlist.count());
        // let l = fontlist.list();
        // println!("{:?}", l);

        let fontinfo = Font::new(&mut display, "lucidasanstypewriter-bold-24".into());
        println!("{:?}", fontinfo);
        println!(
            "FontID=0x{:02X} Properties=0x{:02X?}",
            (*(fontinfo.font())).fid,
            (*(fontinfo.font())).n_properties
        );
        // xlib::XUnloadFont(display.display(), fontinfo.id());

        // Setup some graphic contexts for different colours

        // let default_gc = GraphicContextBuilder::default()
        //     .set_font(fontinfo.id())
        //     .create(display.display(), window);

        let red_gc = GraphicContextBuilder::default()
            .set_background_colour(xlib::XWhitePixel(display.display(), display.screen()))
            .set_foreground_colour(pixel_value_for_colour(
                display.display(),
                display.screen(),
                "red",
            ))
            .set_font(fontinfo.id())
            .create(display.display(), window);

        let green_gc = GraphicContextBuilder::default()
            .set_background_colour(xlib::XWhitePixel(display.display(), display.screen()))
            .set_foreground_colour(pixel_value_for_colour(
                display.display(),
                display.screen(),
                "green",
            ))
            .set_font(fontinfo.id())
            .create(display.display(), window);

        let blue_gc = GraphicContextBuilder::default()
            .set_background_colour(xlib::XWhitePixel(display.display(), display.screen()))
            .set_foreground_colour(pixel_value_for_colour(
                display.display(),
                display.screen(),
                "blue",
            ))
            .set_font(fontinfo.id())
            .create(display.display(), window);

        let white_gc = GraphicContextBuilder::default()
            .set_background_colour(xlib::XWhitePixel(display.display(), display.screen()))
            .set_foreground_colour(xlib::XWhitePixel(display.display(), display.screen()))
            .set_font(fontinfo.id())
            .create(display.display(), window);

        let black_gc = GraphicContextBuilder::default()
            .set_background_colour(xlib::XWhitePixel(display.display(), display.screen()))
            .set_foreground_colour(xlib::XBlackPixel(display.display(), display.screen()))
            .set_font(fontinfo.id())
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
                    let xsym = xlib::XKeycodeToKeysym(
                        display.display(),
                        xkey.keycode.try_into().unwrap(),
                        0,
                    );
                    if xsym != xlib::NoSymbol.try_into().unwrap() {
                        // let xstr = std::ffi::CStr::from_ptr(xlib::XKeysymToString(xsym))
                        //     .to_owned()
                        //     .into_string()
                        //     .unwrap();
                        // println!("Key {} {} {}", xkey.keycode, xsym, xstr);
                        if xsym == 32 {
                            k = true;
                        }
                    }
                }
                xlib::KeyRelease => {
                    let xkey = xlib::XKeyEvent::from(event);
                    let xsym = xlib::XKeycodeToKeysym(
                        display.display(),
                        xkey.keycode.try_into().unwrap(),
                        0,
                    );
                    if xsym != xlib::NoSymbol.try_into().unwrap() {
                        if xsym == 113 {
                            break;
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
                    // println!("Button {} {} {}", xbutton.button, x, y);
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

        println!("Shutting down");
    }
}
