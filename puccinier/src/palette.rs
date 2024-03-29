use std::collections::HashMap;

pub type Color = HashMap<&'static str, &'static str>;
pub type Palette = HashMap<&'static str, Color>;

pub fn palettes() -> HashMap<&'static str, Palette> {
    HashMap::from([
        (
            "latte",
            HashMap::from([
                (
                    "rosewater",
                    HashMap::from([
                        ("hex", "dc8a78"),
                        ("rgb", "rgb(220, 138, 120)"),
                        ("hsl", "hsl(11, 59%, 67%)"),
                    ]),
                ),
                (
                    "flamingo",
                    HashMap::from([
                        ("hex", "dd7878"),
                        ("rgb", "rgb(221, 120, 120)"),
                        ("hsl", "hsl(0, 60%, 67%)"),
                    ]),
                ),
                (
                    "pink",
                    HashMap::from([
                        ("hex", "ea76cb"),
                        ("rgb", "rgb(234, 118, 203)"),
                        ("hsl", "hsl(316, 73%, 69%)"),
                    ]),
                ),
                (
                    "mauve",
                    HashMap::from([
                        ("hex", "8839ef"),
                        ("rgb", "rgb(136, 57, 239)"),
                        ("hsl", "hsl(266, 85%, 58%)"),
                    ]),
                ),
                (
                    "red",
                    HashMap::from([
                        ("hex", "d20f39"),
                        ("rgb", "rgb(210, 15, 57)"),
                        ("hsl", "hsl(347, 87%, 44%)"),
                    ]),
                ),
                (
                    "maroon",
                    HashMap::from([
                        ("hex", "e64553"),
                        ("rgb", "rgb(230, 69, 83)"),
                        ("hsl", "hsl(355, 76%, 59%)"),
                    ]),
                ),
                (
                    "peach",
                    HashMap::from([
                        ("hex", "fe640b"),
                        ("rgb", "rgb(254, 100, 11)"),
                        ("hsl", "hsl(22, 99%, 52%)"),
                    ]),
                ),
                (
                    "yellow",
                    HashMap::from([
                        ("hex", "df8e1d"),
                        ("rgb", "rgb(223, 142, 29)"),
                        ("hsl", "hsl(35, 77%, 49%)"),
                    ]),
                ),
                (
                    "green",
                    HashMap::from([
                        ("hex", "40a02b"),
                        ("rgb", "rgb(64, 160, 43)"),
                        ("hsl", "hsl(109, 58%, 40%)"),
                    ]),
                ),
                (
                    "teal",
                    HashMap::from([
                        ("hex", "179299"),
                        ("rgb", "rgb(23, 146, 153)"),
                        ("hsl", "hsl(183, 74%, 35%)"),
                    ]),
                ),
                (
                    "sky",
                    HashMap::from([
                        ("hex", "04a5e5"),
                        ("rgb", "rgb(4, 165, 229)"),
                        ("hsl", "hsl(197, 97%, 46%)"),
                    ]),
                ),
                (
                    "sapphire",
                    HashMap::from([
                        ("hex", "209fb5"),
                        ("rgb", "rgb(32, 159, 181)"),
                        ("hsl", "hsl(189, 70%, 42%)"),
                    ]),
                ),
                (
                    "blue",
                    HashMap::from([
                        ("hex", "1e66f5"),
                        ("rgb", "rgb(30, 102, 245)"),
                        ("hsl", "hsl(220, 91%, 54%)"),
                    ]),
                ),
                (
                    "lavender",
                    HashMap::from([
                        ("hex", "7287fd"),
                        ("rgb", "rgb(114, 135, 253)"),
                        ("hsl", "hsl(231, 97%, 72%)"),
                    ]),
                ),
                (
                    "text",
                    HashMap::from([
                        ("hex", "4c4f69"),
                        ("rgb", "rgb(76, 79, 105)"),
                        ("hsl", "hsl(234, 16%, 35%)"),
                    ]),
                ),
                (
                    "subtext1",
                    HashMap::from([
                        ("hex", "5c5f77"),
                        ("rgb", "rgb(92, 95, 119)"),
                        ("hsl", "hsl(233, 13%, 41%)"),
                    ]),
                ),
                (
                    "subtext0",
                    HashMap::from([
                        ("hex", "6c6f85"),
                        ("rgb", "rgb(108, 111, 133)"),
                        ("hsl", "hsl(233, 10%, 47%)"),
                    ]),
                ),
                (
                    "overlay2",
                    HashMap::from([
                        ("hex", "7c7f93"),
                        ("rgb", "rgb(124, 127, 147)"),
                        ("hsl", "hsl(232, 10%, 53%)"),
                    ]),
                ),
                (
                    "overlay1",
                    HashMap::from([
                        ("hex", "8c8fa1"),
                        ("rgb", "rgb(140, 143, 161)"),
                        ("hsl", "hsl(231, 10%, 59%)"),
                    ]),
                ),
                (
                    "overlay0",
                    HashMap::from([
                        ("hex", "9ca0b0"),
                        ("rgb", "rgb(156, 160, 176)"),
                        ("hsl", "hsl(228, 11%, 65%)"),
                    ]),
                ),
                (
                    "surface2",
                    HashMap::from([
                        ("hex", "acb0be"),
                        ("rgb", "rgb(172, 176, 190)"),
                        ("hsl", "hsl(227, 12%, 71%)"),
                    ]),
                ),
                (
                    "surface1",
                    HashMap::from([
                        ("hex", "bcc0cc"),
                        ("rgb", "rgb(188, 192, 204)"),
                        ("hsl", "hsl(225, 14%, 77%)"),
                    ]),
                ),
                (
                    "surface0",
                    HashMap::from([
                        ("hex", "ccd0da"),
                        ("rgb", "rgb(204, 208, 218)"),
                        ("hsl", "hsl(223, 16%, 83%)"),
                    ]),
                ),
                (
                    "base",
                    HashMap::from([
                        ("hex", "eff1f5"),
                        ("rgb", "rgb(239, 241, 245)"),
                        ("hsl", "hsl(220, 23%, 95%)"),
                    ]),
                ),
                (
                    "mantle",
                    HashMap::from([
                        ("hex", "e6e9ef"),
                        ("rgb", "rgb(230, 233, 239)"),
                        ("hsl", "hsl(220, 22%, 92%)"),
                    ]),
                ),
                (
                    "crust",
                    HashMap::from([
                        ("hex", "dce0e8"),
                        ("rgb", "rgb(220, 224, 232)"),
                        ("hsl", "hsl(220, 21%, 89%)"),
                    ]),
                ),
            ]),
        ),
        (
            "frappe",
            HashMap::from([
                (
                    "rosewater",
                    HashMap::from([
                        ("hex", "f2d5cf"),
                        ("rgb", "rgb(242, 213, 207)"),
                        ("hsl", "hsl(10, 57%, 88%)"),
                    ]),
                ),
                (
                    "flamingo",
                    HashMap::from([
                        ("hex", "eebebe"),
                        ("rgb", "rgb(238, 190, 190)"),
                        ("hsl", "hsl(0, 59%, 84%)"),
                    ]),
                ),
                (
                    "pink",
                    HashMap::from([
                        ("hex", "f4b8e4"),
                        ("rgb", "rgb(244, 184, 228)"),
                        ("hsl", "hsl(316, 73%, 84%)"),
                    ]),
                ),
                (
                    "mauve",
                    HashMap::from([
                        ("hex", "ca9ee6"),
                        ("rgb", "rgb(202, 158, 230)"),
                        ("hsl", "hsl(277, 59%, 76%)"),
                    ]),
                ),
                (
                    "red",
                    HashMap::from([
                        ("hex", "e78284"),
                        ("rgb", "rgb(231, 130, 132)"),
                        ("hsl", "hsl(359, 68%, 71%)"),
                    ]),
                ),
                (
                    "maroon",
                    HashMap::from([
                        ("hex", "ea999c"),
                        ("rgb", "rgb(234, 153, 156)"),
                        ("hsl", "hsl(358, 66%, 76%)"),
                    ]),
                ),
                (
                    "peach",
                    HashMap::from([
                        ("hex", "ef9f76"),
                        ("rgb", "rgb(239, 159, 118)"),
                        ("hsl", "hsl(20, 79%, 70%)"),
                    ]),
                ),
                (
                    "yellow",
                    HashMap::from([
                        ("hex", "e5c890"),
                        ("rgb", "rgb(229, 200, 144)"),
                        ("hsl", "hsl(40, 62%, 73%)"),
                    ]),
                ),
                (
                    "green",
                    HashMap::from([
                        ("hex", "a6d189"),
                        ("rgb", "rgb(166, 209, 137)"),
                        ("hsl", "hsl(96, 44%, 68%)"),
                    ]),
                ),
                (
                    "teal",
                    HashMap::from([
                        ("hex", "81c8be"),
                        ("rgb", "rgb(129, 200, 190)"),
                        ("hsl", "hsl(172, 39%, 65%)"),
                    ]),
                ),
                (
                    "sky",
                    HashMap::from([
                        ("hex", "99d1db"),
                        ("rgb", "rgb(153, 209, 219)"),
                        ("hsl", "hsl(189, 48%, 73%)"),
                    ]),
                ),
                (
                    "sapphire",
                    HashMap::from([
                        ("hex", "85c1dc"),
                        ("rgb", "rgb(133, 193, 220)"),
                        ("hsl", "hsl(199, 55%, 69%)"),
                    ]),
                ),
                (
                    "blue",
                    HashMap::from([
                        ("hex", "8caaee"),
                        ("rgb", "rgb(140, 170, 238)"),
                        ("hsl", "hsl(222, 74%, 74%)"),
                    ]),
                ),
                (
                    "lavender",
                    HashMap::from([
                        ("hex", "babbf1"),
                        ("rgb", "rgb(186, 187, 241)"),
                        ("hsl", "hsl(239, 66%, 84%)"),
                    ]),
                ),
                (
                    "text",
                    HashMap::from([
                        ("hex", "c6d0f5"),
                        ("rgb", "rgb(198, 208, 245)"),
                        ("hsl", "hsl(227, 70%, 87%)"),
                    ]),
                ),
                (
                    "subtext1",
                    HashMap::from([
                        ("hex", "b5bfe2"),
                        ("rgb", "rgb(181, 191, 226)"),
                        ("hsl", "hsl(227, 44%, 80%)"),
                    ]),
                ),
                (
                    "subtext0",
                    HashMap::from([
                        ("hex", "a5adce"),
                        ("rgb", "rgb(165, 173, 206)"),
                        ("hsl", "hsl(228, 29%, 73%)"),
                    ]),
                ),
                (
                    "overlay2",
                    HashMap::from([
                        ("hex", "949cbb"),
                        ("rgb", "rgb(148, 156, 187)"),
                        ("hsl", "hsl(228, 22%, 66%)"),
                    ]),
                ),
                (
                    "overlay1",
                    HashMap::from([
                        ("hex", "838ba7"),
                        ("rgb", "rgb(131, 139, 167)"),
                        ("hsl", "hsl(227, 17%, 58%)"),
                    ]),
                ),
                (
                    "overlay0",
                    HashMap::from([
                        ("hex", "737994"),
                        ("rgb", "rgb(115, 121, 148)"),
                        ("hsl", "hsl(229, 13%, 52%)"),
                    ]),
                ),
                (
                    "surface2",
                    HashMap::from([
                        ("hex", "626880"),
                        ("rgb", "rgb(98, 104, 128)"),
                        ("hsl", "hsl(228, 13%, 44%)"),
                    ]),
                ),
                (
                    "surface1",
                    HashMap::from([
                        ("hex", "51576d"),
                        ("rgb", "rgb(81, 87, 109)"),
                        ("hsl", "hsl(227, 15%, 37%)"),
                    ]),
                ),
                (
                    "surface0",
                    HashMap::from([
                        ("hex", "414559"),
                        ("rgb", "rgb(65, 69, 89)"),
                        ("hsl", "hsl(230, 16%, 30%)"),
                    ]),
                ),
                (
                    "base",
                    HashMap::from([
                        ("hex", "303446"),
                        ("rgb", "rgb(48, 52, 70)"),
                        ("hsl", "hsl(229, 19%, 23%)"),
                    ]),
                ),
                (
                    "mantle",
                    HashMap::from([
                        ("hex", "292c3c"),
                        ("rgb", "rgb(41, 44, 60)"),
                        ("hsl", "hsl(231, 19%, 20%)"),
                    ]),
                ),
                (
                    "crust",
                    HashMap::from([
                        ("hex", "232634"),
                        ("rgb", "rgb(35, 38, 52)"),
                        ("hsl", "hsl(229, 20%, 17%)"),
                    ]),
                ),
            ]),
        ),
        (
            "macchiato",
            HashMap::from([
                (
                    "rosewater",
                    HashMap::from([
                        ("hex", "f4dbd6"),
                        ("rgb", "rgb(244, 219, 214)"),
                        ("hsl", "hsl(10, 58%, 90%)"),
                    ]),
                ),
                (
                    "flamingo",
                    HashMap::from([
                        ("hex", "f0c6c6"),
                        ("rgb", "rgb(240, 198, 198)"),
                        ("hsl", "hsl(0, 58%, 86%)"),
                    ]),
                ),
                (
                    "pink",
                    HashMap::from([
                        ("hex", "f5bde6"),
                        ("rgb", "rgb(245, 189, 230)"),
                        ("hsl", "hsl(316, 74%, 85%)"),
                    ]),
                ),
                (
                    "mauve",
                    HashMap::from([
                        ("hex", "c6a0f6"),
                        ("rgb", "rgb(198, 160, 246)"),
                        ("hsl", "hsl(267, 83%, 80%)"),
                    ]),
                ),
                (
                    "red",
                    HashMap::from([
                        ("hex", "ed8796"),
                        ("rgb", "rgb(237, 135, 150)"),
                        ("hsl", "hsl(351, 74%, 73%)"),
                    ]),
                ),
                (
                    "maroon",
                    HashMap::from([
                        ("hex", "ee99a0"),
                        ("rgb", "rgb(238, 153, 160)"),
                        ("hsl", "hsl(355, 71%, 77%)"),
                    ]),
                ),
                (
                    "peach",
                    HashMap::from([
                        ("hex", "f5a97f"),
                        ("rgb", "rgb(245, 169, 127)"),
                        ("hsl", "hsl(21, 86%, 73%)"),
                    ]),
                ),
                (
                    "yellow",
                    HashMap::from([
                        ("hex", "eed49f"),
                        ("rgb", "rgb(238, 212, 159)"),
                        ("hsl", "hsl(40, 70%, 78%)"),
                    ]),
                ),
                (
                    "green",
                    HashMap::from([
                        ("hex", "a6da95"),
                        ("rgb", "rgb(166, 218, 149)"),
                        ("hsl", "hsl(105, 48%, 72%)"),
                    ]),
                ),
                (
                    "teal",
                    HashMap::from([
                        ("hex", "8bd5ca"),
                        ("rgb", "rgb(139, 213, 202)"),
                        ("hsl", "hsl(171, 47%, 69%)"),
                    ]),
                ),
                (
                    "sky",
                    HashMap::from([
                        ("hex", "91d7e3"),
                        ("rgb", "rgb(145, 215, 227)"),
                        ("hsl", "hsl(189, 59%, 73%)"),
                    ]),
                ),
                (
                    "sapphire",
                    HashMap::from([
                        ("hex", "7dc4e4"),
                        ("rgb", "rgb(125, 196, 228)"),
                        ("hsl", "hsl(199, 66%, 69%)"),
                    ]),
                ),
                (
                    "blue",
                    HashMap::from([
                        ("hex", "8aadf4"),
                        ("rgb", "rgb(138, 173, 244)"),
                        ("hsl", "hsl(220, 83%, 75%)"),
                    ]),
                ),
                (
                    "lavender",
                    HashMap::from([
                        ("hex", "b7bdf8"),
                        ("rgb", "rgb(183, 189, 248)"),
                        ("hsl", "hsl(234, 82%, 85%)"),
                    ]),
                ),
                (
                    "text",
                    HashMap::from([
                        ("hex", "cad3f5"),
                        ("rgb", "rgb(202, 211, 245)"),
                        ("hsl", "hsl(227, 68%, 88%)"),
                    ]),
                ),
                (
                    "subtext1",
                    HashMap::from([
                        ("hex", "b8c0e0"),
                        ("rgb", "rgb(184, 192, 224)"),
                        ("hsl", "hsl(228, 39%, 80%)"),
                    ]),
                ),
                (
                    "subtext0",
                    HashMap::from([
                        ("hex", "a5adcb"),
                        ("rgb", "rgb(165, 173, 203)"),
                        ("hsl", "hsl(227, 27%, 72%)"),
                    ]),
                ),
                (
                    "overlay2",
                    HashMap::from([
                        ("hex", "939ab7"),
                        ("rgb", "rgb(147, 154, 183)"),
                        ("hsl", "hsl(228, 20%, 65%)"),
                    ]),
                ),
                (
                    "overlay1",
                    HashMap::from([
                        ("hex", "8087a2"),
                        ("rgb", "rgb(128, 135, 162)"),
                        ("hsl", "hsl(228, 15%, 57%)"),
                    ]),
                ),
                (
                    "overlay0",
                    HashMap::from([
                        ("hex", "6e738d"),
                        ("rgb", "rgb(110, 115, 141)"),
                        ("hsl", "hsl(230, 12%, 49%)"),
                    ]),
                ),
                (
                    "surface2",
                    HashMap::from([
                        ("hex", "5b6078"),
                        ("rgb", "rgb(91, 96, 120)"),
                        ("hsl", "hsl(230, 14%, 41%)"),
                    ]),
                ),
                (
                    "surface1",
                    HashMap::from([
                        ("hex", "494d64"),
                        ("rgb", "rgb(73, 77, 100)"),
                        ("hsl", "hsl(231, 16%, 34%)"),
                    ]),
                ),
                (
                    "surface0",
                    HashMap::from([
                        ("hex", "363a4f"),
                        ("rgb", "rgb(54, 58, 79)"),
                        ("hsl", "hsl(230, 19%, 26%)"),
                    ]),
                ),
                (
                    "base",
                    HashMap::from([
                        ("hex", "24273a"),
                        ("rgb", "rgb(36, 39, 58)"),
                        ("hsl", "hsl(232, 23%, 18%)"),
                    ]),
                ),
                (
                    "mantle",
                    HashMap::from([
                        ("hex", "1e2030"),
                        ("rgb", "rgb(30, 32, 48)"),
                        ("hsl", "hsl(233, 23%, 15%)"),
                    ]),
                ),
                (
                    "crust",
                    HashMap::from([
                        ("hex", "181926"),
                        ("rgb", "rgb(24, 25, 38)"),
                        ("hsl", "hsl(236, 23%, 12%)"),
                    ]),
                ),
            ]),
        ),
        (
            "mocha",
            HashMap::from([
                (
                    "rosewater",
                    HashMap::from([
                        ("hex", "f5e0dc"),
                        ("rgb", "rgb(245, 224, 220)"),
                        ("hsl", "hsl(10, 56%, 91%)"),
                    ]),
                ),
                (
                    "flamingo",
                    HashMap::from([
                        ("hex", "f2cdcd"),
                        ("rgb", "rgb(242, 205, 205)"),
                        ("hsl", "hsl(0, 59%, 88%)"),
                    ]),
                ),
                (
                    "pink",
                    HashMap::from([
                        ("hex", "f5c2e7"),
                        ("rgb", "rgb(245, 194, 231)"),
                        ("hsl", "hsl(316, 72%, 86%)"),
                    ]),
                ),
                (
                    "mauve",
                    HashMap::from([
                        ("hex", "cba6f7"),
                        ("rgb", "rgb(203, 166, 247)"),
                        ("hsl", "hsl(267, 84%, 81%)"),
                    ]),
                ),
                (
                    "red",
                    HashMap::from([
                        ("hex", "f38ba8"),
                        ("rgb", "rgb(243, 139, 168)"),
                        ("hsl", "hsl(343, 81%, 75%)"),
                    ]),
                ),
                (
                    "maroon",
                    HashMap::from([
                        ("hex", "eba0ac"),
                        ("rgb", "rgb(235, 160, 172)"),
                        ("hsl", "hsl(350, 65%, 77%)"),
                    ]),
                ),
                (
                    "peach",
                    HashMap::from([
                        ("hex", "fab387"),
                        ("rgb", "rgb(250, 179, 135)"),
                        ("hsl", "hsl(23, 92%, 75%)"),
                    ]),
                ),
                (
                    "yellow",
                    HashMap::from([
                        ("hex", "f9e2af"),
                        ("rgb", "rgb(249, 226, 175)"),
                        ("hsl", "hsl(41, 86%, 83%)"),
                    ]),
                ),
                (
                    "green",
                    HashMap::from([
                        ("hex", "a6e3a1"),
                        ("rgb", "rgb(166, 227, 161)"),
                        ("hsl", "hsl(115, 54%, 76%)"),
                    ]),
                ),
                (
                    "teal",
                    HashMap::from([
                        ("hex", "94e2d5"),
                        ("rgb", "rgb(148, 226, 213)"),
                        ("hsl", "hsl(170, 57%, 73%)"),
                    ]),
                ),
                (
                    "sky",
                    HashMap::from([
                        ("hex", "89dceb"),
                        ("rgb", "rgb(137, 220, 235)"),
                        ("hsl", "hsl(189, 71%, 73%)"),
                    ]),
                ),
                (
                    "sapphire",
                    HashMap::from([
                        ("hex", "74c7ec"),
                        ("rgb", "rgb(116, 199, 236)"),
                        ("hsl", "hsl(199, 76%, 69%)"),
                    ]),
                ),
                (
                    "blue",
                    HashMap::from([
                        ("hex", "89b4fa"),
                        ("rgb", "rgb(137, 180, 250)"),
                        ("hsl", "hsl(217, 92%, 76%)"),
                    ]),
                ),
                (
                    "lavender",
                    HashMap::from([
                        ("hex", "b4befe"),
                        ("rgb", "rgb(180, 190, 254)"),
                        ("hsl", "hsl(232, 97%, 85%)"),
                    ]),
                ),
                (
                    "text",
                    HashMap::from([
                        ("hex", "cdd6f4"),
                        ("rgb", "rgb(205, 214, 244)"),
                        ("hsl", "hsl(226, 64%, 88%)"),
                    ]),
                ),
                (
                    "subtext1",
                    HashMap::from([
                        ("hex", "bac2de"),
                        ("rgb", "rgb(186, 194, 222)"),
                        ("hsl", "hsl(227, 35%, 80%)"),
                    ]),
                ),
                (
                    "subtext0",
                    HashMap::from([
                        ("hex", "a6adc8"),
                        ("rgb", "rgb(166, 173, 200)"),
                        ("hsl", "hsl(228, 24%, 72%)"),
                    ]),
                ),
                (
                    "overlay2",
                    HashMap::from([
                        ("hex", "9399b2"),
                        ("rgb", "rgb(147, 153, 178)"),
                        ("hsl", "hsl(228, 17%, 64%)"),
                    ]),
                ),
                (
                    "overlay1",
                    HashMap::from([
                        ("hex", "7f849c"),
                        ("rgb", "rgb(127, 132, 156)"),
                        ("hsl", "hsl(230, 13%, 55%)"),
                    ]),
                ),
                (
                    "overlay0",
                    HashMap::from([
                        ("hex", "6c7086"),
                        ("rgb", "rgb(108, 112, 134)"),
                        ("hsl", "hsl(231, 11%, 47%)"),
                    ]),
                ),
                (
                    "surface2",
                    HashMap::from([
                        ("hex", "585b70"),
                        ("rgb", "rgb(88, 91, 112)"),
                        ("hsl", "hsl(233, 12%, 39%)"),
                    ]),
                ),
                (
                    "surface1",
                    HashMap::from([
                        ("hex", "45475a"),
                        ("rgb", "rgb(69, 71, 90)"),
                        ("hsl", "hsl(234, 13%, 31%)"),
                    ]),
                ),
                (
                    "surface0",
                    HashMap::from([
                        ("hex", "313244"),
                        ("rgb", "rgb(49, 50, 68)"),
                        ("hsl", "hsl(237, 16%, 23%)"),
                    ]),
                ),
                (
                    "base",
                    HashMap::from([
                        ("hex", "1e1e2e"),
                        ("rgb", "rgb(30, 30, 46)"),
                        ("hsl", "hsl(240, 21%, 15%)"),
                    ]),
                ),
                (
                    "mantle",
                    HashMap::from([
                        ("hex", "181825"),
                        ("rgb", "rgb(24, 24, 37)"),
                        ("hsl", "hsl(240, 21%, 12%)"),
                    ]),
                ),
                (
                    "crust",
                    HashMap::from([
                        ("hex", "11111b"),
                        ("rgb", "rgb(17, 17, 27)"),
                        ("hsl", "hsl(240, 23%, 9%)"),
                    ]),
                ),
            ]),
        ),
    ])
}
