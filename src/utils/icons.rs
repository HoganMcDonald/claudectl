#![allow(dead_code)]

pub struct Status {
    pub success: &'static str,
    pub failure: &'static str,
    pub warning: &'static str,
    pub info: &'static str,
    pub circle: &'static str,
}

pub struct Progress {
    /// e.g., "◐", "◓", "◑", "◒"
    pub unicode: &'static [&'static str],
    /// e.g., Braille spinner frames
    pub braille: &'static [&'static str],
    /// e.g., "○", "◔", "◑", "◕", "●"
    pub circles: &'static [&'static str],
}

pub struct Arrows {
    pub right: &'static str,
    pub left: &'static str,
    pub up: &'static str,
    pub down: &'static str,
    pub dbl_right: &'static str,
    pub dbl_left: &'static str,
    pub dbl_up: &'static str,
    pub dbl_down: &'static str,
    pub thick_right: &'static str,
    pub thick_point: &'static str,
    pub thick_right_boxy: &'static str,
    pub branch_right: &'static str,
    pub hook_right: &'static str,
    pub turn_down_right: &'static str,
}

pub struct Lists {
    pub bullet: &'static str,
    pub bullet_alt: &'static str,
    pub bullet_heavy: &'static str,
    pub caret: &'static str,
    pub box_empty: &'static str,
    pub box_square: &'static str,
    pub box_checked: &'static str,
    pub square: &'static str,
    pub star: &'static str,
    pub star_hollow: &'static str,
    pub star_spark: &'static str,
    pub star_spark_hollow: &'static str,
}

pub struct BoxDraw {
    pub corner_tl: &'static str,
    pub corner_tr: &'static str,
    pub corner_bl: &'static str,
    pub corner_br: &'static str,
    pub horiz: &'static str,
    pub vert: &'static str,
    pub tee_top: &'static str,
    pub tee_bottom: &'static str,
    pub tee_left: &'static str,
    pub tee_right: &'static str,
    pub cross: &'static str,
    pub dbl_corner_tl: &'static str,
    pub dbl_corner_tr: &'static str,
    pub dbl_corner_bl: &'static str,
    pub dbl_corner_br: &'static str,
    pub dbl_horiz: &'static str,
    pub dbl_vert: &'static str,
}

pub struct Misc {
    pub stopwatch: &'static str,
    pub flag: &'static str,
    pub flag_alt: &'static str,
}

pub struct Icons {
    pub status: Status,
    pub progress: Progress,
    pub arrows: Arrows,
    pub lists: Lists,
    pub box_draw: BoxDraw,
    pub misc: Misc,
}

// Spinner frame arrays (exported)
pub const SPINNER_UNICODE: [&str; 4] = ["◐", "◓", "◑", "◒"];
pub const SPINNER_BRAILLE: [&str; 8] = ["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"];
pub const SPINNER_CIRCLES: [&str; 5] = ["○", "◔", "◑", "◕", "●"];

// One global, reusable collection
pub const ICONS: Icons = Icons {
    status: Status {
        success: "✓",
        failure: "✗",
        warning: "⚠",
        info: "ℹ",
        circle: "●",
    },
    progress: Progress {
        unicode: &SPINNER_UNICODE,
        braille: &SPINNER_BRAILLE,
        circles: &SPINNER_CIRCLES,
    },
    arrows: Arrows {
        right: "→",
        left: "←",
        up: "↑",
        down: "↓",
        dbl_right: "⇒",
        dbl_left: "⇐",
        dbl_up: "⇑",
        dbl_down: "⇓",
        thick_right: "➜",
        thick_point: "➤",
        thick_right_boxy: "➡",
        branch_right: "↣",
        hook_right: "↪",
        turn_down_right: "⤷",
    },
    lists: Lists {
        bullet: "•",
        bullet_alt: "◦",
        bullet_heavy: "‣",
        caret: "▸",
        box_empty: "□",
        box_square: "▢",
        box_checked: "▣",
        square: "■",
        star: "★",
        star_hollow: "☆",
        star_spark: "✦",
        star_spark_hollow: "✧",
    },
    box_draw: BoxDraw {
        corner_tl: "┌",
        corner_tr: "┐",
        corner_bl: "└",
        corner_br: "┘",
        horiz: "─",
        vert: "│",
        tee_top: "┬",
        tee_bottom: "┴",
        tee_left: "├",
        tee_right: "┤",
        cross: "┼",
        dbl_corner_tl: "╔",
        dbl_corner_tr: "╗",
        dbl_corner_bl: "╚",
        dbl_corner_br: "╝",
        dbl_horiz: "═",
        dbl_vert: "║",
    },
    misc: Misc {
        stopwatch: "⏱",
        flag: "⚑",
        flag_alt: "⚐",
    },
};
