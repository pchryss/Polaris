pub struct Constellation {
    pub name: &'static str,
    pub pattern: [&'static str; 11],
}

pub const CONSTELLATIONS : &[Constellation] = &[
    AQUARIUS,
    BIG_DIPPER,
    CANCER,
    LIBRA,
    LEPUS,
    SCORPIUS,
    VIRGO,
];


pub const SCORPIUS: Constellation =  Constellation {
    name: "Scorpius",
    pattern: [
        "                        *     ",
        "                         *    ",
        "                   *      *   ",
        "                 *            ",
        "                              ",
        "               *              ",
        "                              ",
        "    *         *               ",
        "   *                          ",
        "  *           *               ",
        "     *   *                    ",

    ]
};

pub const VIRGO: Constellation =  Constellation {
    name: "Virgo",
    pattern: [
        "              *               ",
        "                             *",
        "                  *           ",
        "                      *       ",
        "                              ",
        "         *    *               ",
        " *                 *          ",
        "                              ",
        "  *    *           *          ",
        "    *                         ",
        "           *                  ",

    ]
};

pub const LIBRA: Constellation =  Constellation {
    name: "Libra",
    pattern: [
        "                              ",
        "              *               ",
        "                              ",
        "           *        *         ",
        "                              ",
        "                              ",
        "                  *           ",
        "                              ",
        "            *                 ",
        "           *                  ",
        "                              ",
    ]
};

pub const CANCER: Constellation =  Constellation {
    name: "Cancer",
    pattern: [
        "                              ",
        "             *                ",
        "                              ",
        "                              ",
        "              *               ",
        "             *                ",
        "                              ",
        "         *                    ",
        "                              ",
        "                    *         ",
        "                              ",
    ]
};

pub const BIG_DIPPER: Constellation =  Constellation {
    name: "Big Dipper",
    pattern: [
        "                              ",
        "                              ",
        "                              ",
        "      *                       ",
        "  *       *             *     ",
        "              *               ",
        "                              ",
        "                 *       *    ",
        "                              ",
        "                              ",
        "                              ",
    ]
};

pub const LEPUS: Constellation =  Constellation {
    name: "Lepus",
    pattern: [
        "                              ",
        "                      *  *    ",
        "     *                        ",
        " *       *                    ",
        "                       *      ",
        "               *              ",
        "                              ",
        "        *        *            ",
        "           *             *    ",
        "                              ",
        "                              ",
    ]
};

pub const AQUARIUS: Constellation =  Constellation {
    name: "Aquarius",
    pattern: [
        "                              ",
        "                     *        ",
        "                *             ",
        "           *                  ",
        "                              ",
        "        * *      *    *       ",
        "       *                      ",
        "                              ",
        "               *   *          ",
        "             *         *      ",
        "                              ",

    ]
};

pub const UNKNOWN: Constellation =  Constellation {
    name: "UNKNOWN",
    pattern: [
        "                              ",
        "                              ",
        "            ,------.          ",
        "           '  .--.  '         ",
        "           '--' _|  |         ",
        "            .--' __'          ",
        "            `---'             ",
        "            .---.             ",
        "            '---'             ",
        "                              ",
        "                              ",
    ]
};