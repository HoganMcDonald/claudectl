#![allow(dead_code)]

use owo_colors::Rgb;

// Catppuccin Mocha palette
pub struct CatppuccinColors {
    pub rosewater: Rgb,
    pub flamingo: Rgb,
    pub pink: Rgb,
    pub mauve: Rgb,
    pub red: Rgb,
    pub maroon: Rgb,
    pub peach: Rgb,
    pub yellow: Rgb,
    pub green: Rgb,
    pub teal: Rgb,
    pub sky: Rgb,
    pub sapphire: Rgb,
    pub blue: Rgb,
    pub lavender: Rgb,
    pub text: Rgb,
    pub subtext1: Rgb,
    pub subtext0: Rgb,
    pub overlay2: Rgb,
    pub overlay1: Rgb,
    pub overlay0: Rgb,
    pub surface2: Rgb,
    pub surface1: Rgb,
    pub surface0: Rgb,
    pub base: Rgb,
    pub mantle: Rgb,
    pub crust: Rgb,
}

pub struct ThemeColors {
    pub success: Rgb,
    pub error: Rgb,
    pub warning: Rgb,
    pub info: Rgb,
    pub primary: Rgb,
    pub secondary: Rgb,
    pub text: Rgb,
    pub muted: Rgb,
    pub accent: Rgb,
}

pub const CATPPUCCIN: CatppuccinColors = CatppuccinColors {
    rosewater: Rgb(245, 224, 220), // #f5e0dc
    flamingo: Rgb(242, 205, 205),  // #f2cdcd
    pink: Rgb(245, 194, 231),      // #f5c2e7
    mauve: Rgb(203, 166, 247),     // #cba6f7
    red: Rgb(243, 139, 168),       // #f38ba8
    maroon: Rgb(235, 160, 172),    // #eba0ac
    peach: Rgb(250, 179, 135),     // #fab387
    yellow: Rgb(249, 226, 175),    // #f9e2af
    green: Rgb(166, 227, 161),     // #a6e3a1
    teal: Rgb(148, 226, 213),      // #94e2d5
    sky: Rgb(137, 220, 235),       // #89dceb
    sapphire: Rgb(116, 199, 236),  // #74c7ec
    blue: Rgb(137, 180, 250),      // #89b4fa
    lavender: Rgb(180, 190, 254),  // #b4befe
    text: Rgb(205, 214, 244),      // #cdd6f4
    subtext1: Rgb(186, 194, 222),  // #bac2de
    subtext0: Rgb(166, 173, 200),  // #a6adc8
    overlay2: Rgb(147, 153, 178),  // #9399b2
    overlay1: Rgb(127, 132, 156),  // #7f849c
    overlay0: Rgb(108, 112, 134),  // #6c7086
    surface2: Rgb(88, 91, 112),    // #585b70
    surface1: Rgb(69, 71, 90),     // #45475a
    surface0: Rgb(49, 50, 68),     // #313244
    base: Rgb(30, 30, 46),         // #1e1e2e
    mantle: Rgb(24, 24, 37),       // #181825
    crust: Rgb(17, 17, 27),        // #11111b
};

pub const THEME: ThemeColors = ThemeColors {
    success: CATPPUCCIN.green,
    error: CATPPUCCIN.red,
    warning: CATPPUCCIN.yellow,
    info: CATPPUCCIN.blue,
    primary: CATPPUCCIN.mauve,
    secondary: CATPPUCCIN.lavender,
    text: CATPPUCCIN.text,
    muted: CATPPUCCIN.subtext0,
    accent: CATPPUCCIN.sapphire,
};
