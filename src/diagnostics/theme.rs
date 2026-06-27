pub struct Theme {
    pub error: &'static str,
    pub warning: &'static str,
    pub lint: &'static str,
    pub info: &'static str,
    pub note: &'static str,
    pub help: &'static str,
    pub fatal: &'static str,
    pub bug: &'static str,
    pub opt_remark: &'static str,
    pub bold: &'static str,
    pub underline: &'static str,
    pub reset: &'static str,
    pub border: &'static str,
    pub primary_label: &'static str,
    pub secondary_label: &'static str,
    pub quote: &'static str,
    pub caret: &'static str,
}

impl Theme {
    pub fn new() -> Self {
        if Self::color_disabled() {
            Theme::plain()
        } else {
            Theme::fancy()
        }
    }

    fn color_disabled() -> bool {
        std::env::var("NO_COLOR").is_ok()
            || std::env::var("TERM").map(|t| t == "dumb").unwrap_or(false)
    }

    pub fn plain() -> Self {
        Theme {
            error: "",
            warning: "",
            lint: "",
            info: "",
            note: "",
            help: "",
            fatal: "",
            bug: "",
            opt_remark: "",
            bold: "",
            underline: "",
            reset: "",
            border: "",
            primary_label: "",
            secondary_label: "",
            quote: "",
            caret: "",
        }
    }

    pub fn fancy() -> Self {
        Theme {
            error: "\x1b[1;31m",           // Bold Red
            warning: "\x1b[1;33m",         // Bold Yellow
            lint: "\x1b[1;35m",            // Bold Magenta
            info: "\x1b[1;36m",            // Bold Cyan
            note: "\x1b[1;34m",            // Bold Blue
            help: "\x1b[1;32m",            // Bold Green
            fatal: "\x1b[1;31m",           // Bold Red
            bug: "\x1b[1;35m",             // Bold Magenta
            opt_remark: "\x1b[1;36m",      // Bold Cyan
            bold: "\x1b[1m",               // Bold
            underline: "\x1b[4m",          // Underline
            reset: "\x1b[0m",              // Reset
            border: "\x1b[38;5;244m",      // Grey border
            primary_label: "\x1b[1;31m",   // Red primary labels
            secondary_label: "\x1b[1;36m", // Cyan secondary labels
            quote: "\x1b[37m",             // White quote
            caret: "\x1b[1;31m",           // Red caret
        }
    }
}
