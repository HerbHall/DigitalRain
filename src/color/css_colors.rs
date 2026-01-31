//! All 148 CSS Level 4 named colors.
//!
//! Provides a lookup table for CSS color names to RGB values,
//! used by the auto-generated palette system.

/// A CSS named color with its RGB components.
pub struct CssColor {
    pub name: &'static str,
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

/// All 148 CSS Level 4 named colors, sorted alphabetically.
pub const CSS_COLORS: &[CssColor] = &[
    CssColor {
        name: "aliceblue",
        r: 240,
        g: 248,
        b: 255,
    },
    CssColor {
        name: "antiquewhite",
        r: 250,
        g: 235,
        b: 215,
    },
    CssColor {
        name: "aqua",
        r: 0,
        g: 255,
        b: 255,
    },
    CssColor {
        name: "aquamarine",
        r: 127,
        g: 255,
        b: 212,
    },
    CssColor {
        name: "azure",
        r: 240,
        g: 255,
        b: 255,
    },
    CssColor {
        name: "beige",
        r: 245,
        g: 245,
        b: 220,
    },
    CssColor {
        name: "bisque",
        r: 255,
        g: 228,
        b: 196,
    },
    CssColor {
        name: "black",
        r: 0,
        g: 0,
        b: 0,
    },
    CssColor {
        name: "blanchedalmond",
        r: 255,
        g: 235,
        b: 205,
    },
    CssColor {
        name: "blue",
        r: 0,
        g: 0,
        b: 255,
    },
    CssColor {
        name: "blueviolet",
        r: 138,
        g: 43,
        b: 226,
    },
    CssColor {
        name: "brown",
        r: 165,
        g: 42,
        b: 42,
    },
    CssColor {
        name: "burlywood",
        r: 222,
        g: 184,
        b: 135,
    },
    CssColor {
        name: "cadetblue",
        r: 95,
        g: 158,
        b: 160,
    },
    CssColor {
        name: "chartreuse",
        r: 127,
        g: 255,
        b: 0,
    },
    CssColor {
        name: "chocolate",
        r: 210,
        g: 105,
        b: 30,
    },
    CssColor {
        name: "coral",
        r: 255,
        g: 127,
        b: 80,
    },
    CssColor {
        name: "cornflowerblue",
        r: 100,
        g: 149,
        b: 237,
    },
    CssColor {
        name: "cornsilk",
        r: 255,
        g: 248,
        b: 220,
    },
    CssColor {
        name: "crimson",
        r: 220,
        g: 20,
        b: 60,
    },
    CssColor {
        name: "cyan",
        r: 0,
        g: 255,
        b: 255,
    },
    CssColor {
        name: "darkblue",
        r: 0,
        g: 0,
        b: 139,
    },
    CssColor {
        name: "darkcyan",
        r: 0,
        g: 139,
        b: 139,
    },
    CssColor {
        name: "darkgoldenrod",
        r: 184,
        g: 134,
        b: 11,
    },
    CssColor {
        name: "darkgray",
        r: 169,
        g: 169,
        b: 169,
    },
    CssColor {
        name: "darkgreen",
        r: 0,
        g: 100,
        b: 0,
    },
    CssColor {
        name: "darkgrey",
        r: 169,
        g: 169,
        b: 169,
    },
    CssColor {
        name: "darkkhaki",
        r: 189,
        g: 183,
        b: 107,
    },
    CssColor {
        name: "darkmagenta",
        r: 139,
        g: 0,
        b: 139,
    },
    CssColor {
        name: "darkolivegreen",
        r: 85,
        g: 107,
        b: 47,
    },
    CssColor {
        name: "darkorange",
        r: 255,
        g: 140,
        b: 0,
    },
    CssColor {
        name: "darkorchid",
        r: 153,
        g: 50,
        b: 204,
    },
    CssColor {
        name: "darkred",
        r: 139,
        g: 0,
        b: 0,
    },
    CssColor {
        name: "darksalmon",
        r: 233,
        g: 150,
        b: 122,
    },
    CssColor {
        name: "darkseagreen",
        r: 143,
        g: 188,
        b: 143,
    },
    CssColor {
        name: "darkslateblue",
        r: 72,
        g: 61,
        b: 139,
    },
    CssColor {
        name: "darkslategray",
        r: 47,
        g: 79,
        b: 79,
    },
    CssColor {
        name: "darkslategrey",
        r: 47,
        g: 79,
        b: 79,
    },
    CssColor {
        name: "darkturquoise",
        r: 0,
        g: 206,
        b: 209,
    },
    CssColor {
        name: "darkviolet",
        r: 148,
        g: 0,
        b: 211,
    },
    CssColor {
        name: "deeppink",
        r: 255,
        g: 20,
        b: 147,
    },
    CssColor {
        name: "deepskyblue",
        r: 0,
        g: 191,
        b: 255,
    },
    CssColor {
        name: "dimgray",
        r: 105,
        g: 105,
        b: 105,
    },
    CssColor {
        name: "dimgrey",
        r: 105,
        g: 105,
        b: 105,
    },
    CssColor {
        name: "dodgerblue",
        r: 30,
        g: 144,
        b: 255,
    },
    CssColor {
        name: "firebrick",
        r: 178,
        g: 34,
        b: 34,
    },
    CssColor {
        name: "floralwhite",
        r: 255,
        g: 250,
        b: 240,
    },
    CssColor {
        name: "forestgreen",
        r: 34,
        g: 139,
        b: 34,
    },
    CssColor {
        name: "fuchsia",
        r: 255,
        g: 0,
        b: 255,
    },
    CssColor {
        name: "gainsboro",
        r: 220,
        g: 220,
        b: 220,
    },
    CssColor {
        name: "ghostwhite",
        r: 248,
        g: 248,
        b: 255,
    },
    CssColor {
        name: "gold",
        r: 255,
        g: 215,
        b: 0,
    },
    CssColor {
        name: "goldenrod",
        r: 218,
        g: 165,
        b: 32,
    },
    CssColor {
        name: "gray",
        r: 128,
        g: 128,
        b: 128,
    },
    CssColor {
        name: "green",
        r: 0,
        g: 128,
        b: 0,
    },
    CssColor {
        name: "greenyellow",
        r: 173,
        g: 255,
        b: 47,
    },
    CssColor {
        name: "grey",
        r: 128,
        g: 128,
        b: 128,
    },
    CssColor {
        name: "honeydew",
        r: 240,
        g: 255,
        b: 240,
    },
    CssColor {
        name: "hotpink",
        r: 255,
        g: 105,
        b: 180,
    },
    CssColor {
        name: "indianred",
        r: 205,
        g: 92,
        b: 92,
    },
    CssColor {
        name: "indigo",
        r: 75,
        g: 0,
        b: 130,
    },
    CssColor {
        name: "ivory",
        r: 255,
        g: 255,
        b: 240,
    },
    CssColor {
        name: "khaki",
        r: 240,
        g: 230,
        b: 140,
    },
    CssColor {
        name: "lavender",
        r: 230,
        g: 230,
        b: 250,
    },
    CssColor {
        name: "lavenderblush",
        r: 255,
        g: 240,
        b: 245,
    },
    CssColor {
        name: "lawngreen",
        r: 124,
        g: 252,
        b: 0,
    },
    CssColor {
        name: "lemonchiffon",
        r: 255,
        g: 250,
        b: 205,
    },
    CssColor {
        name: "lightblue",
        r: 173,
        g: 216,
        b: 230,
    },
    CssColor {
        name: "lightcoral",
        r: 240,
        g: 128,
        b: 128,
    },
    CssColor {
        name: "lightcyan",
        r: 224,
        g: 255,
        b: 255,
    },
    CssColor {
        name: "lightgoldenrodyellow",
        r: 250,
        g: 250,
        b: 210,
    },
    CssColor {
        name: "lightgray",
        r: 211,
        g: 211,
        b: 211,
    },
    CssColor {
        name: "lightgreen",
        r: 144,
        g: 238,
        b: 144,
    },
    CssColor {
        name: "lightgrey",
        r: 211,
        g: 211,
        b: 211,
    },
    CssColor {
        name: "lightpink",
        r: 255,
        g: 182,
        b: 193,
    },
    CssColor {
        name: "lightsalmon",
        r: 255,
        g: 160,
        b: 122,
    },
    CssColor {
        name: "lightseagreen",
        r: 32,
        g: 178,
        b: 170,
    },
    CssColor {
        name: "lightskyblue",
        r: 135,
        g: 206,
        b: 250,
    },
    CssColor {
        name: "lightslategray",
        r: 119,
        g: 136,
        b: 153,
    },
    CssColor {
        name: "lightslategrey",
        r: 119,
        g: 136,
        b: 153,
    },
    CssColor {
        name: "lightsteelblue",
        r: 176,
        g: 196,
        b: 222,
    },
    CssColor {
        name: "lightyellow",
        r: 255,
        g: 255,
        b: 224,
    },
    CssColor {
        name: "lime",
        r: 0,
        g: 255,
        b: 0,
    },
    CssColor {
        name: "limegreen",
        r: 50,
        g: 205,
        b: 50,
    },
    CssColor {
        name: "linen",
        r: 250,
        g: 240,
        b: 230,
    },
    CssColor {
        name: "magenta",
        r: 255,
        g: 0,
        b: 255,
    },
    CssColor {
        name: "maroon",
        r: 128,
        g: 0,
        b: 0,
    },
    CssColor {
        name: "mediumaquamarine",
        r: 102,
        g: 205,
        b: 170,
    },
    CssColor {
        name: "mediumblue",
        r: 0,
        g: 0,
        b: 205,
    },
    CssColor {
        name: "mediumorchid",
        r: 186,
        g: 85,
        b: 211,
    },
    CssColor {
        name: "mediumpurple",
        r: 147,
        g: 112,
        b: 219,
    },
    CssColor {
        name: "mediumseagreen",
        r: 60,
        g: 179,
        b: 113,
    },
    CssColor {
        name: "mediumslateblue",
        r: 123,
        g: 104,
        b: 238,
    },
    CssColor {
        name: "mediumspringgreen",
        r: 0,
        g: 250,
        b: 154,
    },
    CssColor {
        name: "mediumturquoise",
        r: 72,
        g: 209,
        b: 204,
    },
    CssColor {
        name: "mediumvioletred",
        r: 199,
        g: 21,
        b: 133,
    },
    CssColor {
        name: "midnightblue",
        r: 25,
        g: 25,
        b: 112,
    },
    CssColor {
        name: "mintcream",
        r: 245,
        g: 255,
        b: 250,
    },
    CssColor {
        name: "mistyrose",
        r: 255,
        g: 228,
        b: 225,
    },
    CssColor {
        name: "moccasin",
        r: 255,
        g: 228,
        b: 181,
    },
    CssColor {
        name: "navajowhite",
        r: 255,
        g: 222,
        b: 173,
    },
    CssColor {
        name: "navy",
        r: 0,
        g: 0,
        b: 128,
    },
    CssColor {
        name: "oldlace",
        r: 253,
        g: 245,
        b: 230,
    },
    CssColor {
        name: "olive",
        r: 128,
        g: 128,
        b: 0,
    },
    CssColor {
        name: "olivedrab",
        r: 107,
        g: 142,
        b: 35,
    },
    CssColor {
        name: "orange",
        r: 255,
        g: 165,
        b: 0,
    },
    CssColor {
        name: "orangered",
        r: 255,
        g: 69,
        b: 0,
    },
    CssColor {
        name: "orchid",
        r: 218,
        g: 112,
        b: 214,
    },
    CssColor {
        name: "palegoldenrod",
        r: 238,
        g: 232,
        b: 170,
    },
    CssColor {
        name: "palegreen",
        r: 152,
        g: 251,
        b: 152,
    },
    CssColor {
        name: "paleturquoise",
        r: 175,
        g: 238,
        b: 238,
    },
    CssColor {
        name: "palevioletred",
        r: 219,
        g: 112,
        b: 147,
    },
    CssColor {
        name: "papayawhip",
        r: 255,
        g: 239,
        b: 213,
    },
    CssColor {
        name: "peachpuff",
        r: 255,
        g: 218,
        b: 185,
    },
    CssColor {
        name: "peru",
        r: 205,
        g: 133,
        b: 63,
    },
    CssColor {
        name: "pink",
        r: 255,
        g: 192,
        b: 203,
    },
    CssColor {
        name: "plum",
        r: 221,
        g: 160,
        b: 221,
    },
    CssColor {
        name: "powderblue",
        r: 176,
        g: 224,
        b: 230,
    },
    CssColor {
        name: "purple",
        r: 128,
        g: 0,
        b: 128,
    },
    CssColor {
        name: "rebeccapurple",
        r: 102,
        g: 51,
        b: 153,
    },
    CssColor {
        name: "red",
        r: 255,
        g: 0,
        b: 0,
    },
    CssColor {
        name: "rosybrown",
        r: 188,
        g: 143,
        b: 143,
    },
    CssColor {
        name: "royalblue",
        r: 65,
        g: 105,
        b: 225,
    },
    CssColor {
        name: "saddlebrown",
        r: 139,
        g: 69,
        b: 19,
    },
    CssColor {
        name: "salmon",
        r: 250,
        g: 128,
        b: 114,
    },
    CssColor {
        name: "sandybrown",
        r: 244,
        g: 164,
        b: 96,
    },
    CssColor {
        name: "seagreen",
        r: 46,
        g: 139,
        b: 87,
    },
    CssColor {
        name: "seashell",
        r: 255,
        g: 245,
        b: 238,
    },
    CssColor {
        name: "sienna",
        r: 160,
        g: 82,
        b: 45,
    },
    CssColor {
        name: "silver",
        r: 192,
        g: 192,
        b: 192,
    },
    CssColor {
        name: "skyblue",
        r: 135,
        g: 206,
        b: 235,
    },
    CssColor {
        name: "slateblue",
        r: 106,
        g: 90,
        b: 205,
    },
    CssColor {
        name: "slategray",
        r: 112,
        g: 128,
        b: 144,
    },
    CssColor {
        name: "slategrey",
        r: 112,
        g: 128,
        b: 144,
    },
    CssColor {
        name: "snow",
        r: 255,
        g: 250,
        b: 250,
    },
    CssColor {
        name: "springgreen",
        r: 0,
        g: 255,
        b: 127,
    },
    CssColor {
        name: "steelblue",
        r: 70,
        g: 130,
        b: 180,
    },
    CssColor {
        name: "tan",
        r: 210,
        g: 180,
        b: 140,
    },
    CssColor {
        name: "teal",
        r: 0,
        g: 128,
        b: 128,
    },
    CssColor {
        name: "thistle",
        r: 216,
        g: 191,
        b: 216,
    },
    CssColor {
        name: "tomato",
        r: 255,
        g: 99,
        b: 71,
    },
    CssColor {
        name: "turquoise",
        r: 64,
        g: 224,
        b: 208,
    },
    CssColor {
        name: "violet",
        r: 238,
        g: 130,
        b: 238,
    },
    CssColor {
        name: "wheat",
        r: 245,
        g: 222,
        b: 179,
    },
    CssColor {
        name: "white",
        r: 255,
        g: 255,
        b: 255,
    },
    CssColor {
        name: "whitesmoke",
        r: 245,
        g: 245,
        b: 245,
    },
    CssColor {
        name: "yellow",
        r: 255,
        g: 255,
        b: 0,
    },
    CssColor {
        name: "yellowgreen",
        r: 154,
        g: 205,
        b: 50,
    },
];

/// Look up a CSS color by name (case-insensitive).
pub fn css_color_by_name(name: &str) -> Option<&'static CssColor> {
    let lower = name.to_ascii_lowercase();
    CSS_COLORS.iter().find(|c| c.name == lower)
}

/// Return the list of all CSS color names.
pub fn css_color_names() -> Vec<&'static str> {
    CSS_COLORS.iter().map(|c| c.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn css_color_count_is_148() {
        assert_eq!(CSS_COLORS.len(), 148);
    }

    #[test]
    fn known_color_lookups() {
        let red = css_color_by_name("red").unwrap();
        assert_eq!((red.r, red.g, red.b), (255, 0, 0));

        let cornflower = css_color_by_name("cornflowerblue").unwrap();
        assert_eq!((cornflower.r, cornflower.g, cornflower.b), (100, 149, 237));

        let rebecca = css_color_by_name("rebeccapurple").unwrap();
        assert_eq!((rebecca.r, rebecca.g, rebecca.b), (102, 51, 153));
    }

    #[test]
    fn case_insensitive_lookup() {
        assert!(css_color_by_name("Red").is_some());
        assert!(css_color_by_name("RED").is_some());
        assert!(css_color_by_name("CornflowerBlue").is_some());
    }

    #[test]
    fn unknown_color_returns_none() {
        assert!(css_color_by_name("matrixgreen").is_none());
        assert!(css_color_by_name("notacolor").is_none());
    }

    #[test]
    fn names_list_matches_count() {
        assert_eq!(css_color_names().len(), 148);
    }

    #[test]
    fn colors_are_alphabetically_sorted() {
        let names = css_color_names();
        for i in 1..names.len() {
            assert!(
                names[i - 1] <= names[i],
                "Colors not sorted: '{}' should come before '{}'",
                names[i - 1],
                names[i]
            );
        }
    }
}
