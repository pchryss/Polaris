pub struct Constellation {
    pub name: &'static str,
    pub pattern: [&'static str; 13],
}

pub const CONSTELLATIONS : &[Constellation] = &[
    ARIES,
    AQUARIUS,
    BIG_DIPPER,
    CANCER,
    CAPRICORN,
    GEMINI,
    LIBRA,
    LEO,
    LEPUS,
    PISCES,
    SAGGITTARIUS,
    SCORPIO,
    TAURUS,
    VIRGO,
];

pub const LEO: Constellation =  Constellation {
    name: "Leo",
    pattern: [
        "                              ",
        "                       *      ",
        "                           *  ",
        "                   *          ",
        "                              ",
        "                   *          ",
        "      *               *       ",
        "                              ",
        "        *               *     ",
        " *                            ",
        "                              ",
        "                              ",
        "                              ",
    ]
};

pub const TAURUS: Constellation =  Constellation {
    name: "Taurus",
    pattern: [
        "                              ",
        "                              ",
        "     *                        ",
        "                              ",
        "   *         *                ",
        "            *  *              ",
        "                   *          ",
        "                        **    ",
        "                              ",
        "                   *          ",
        "                        *     ",
        "                              ",
        "                              ",
    ]
};

pub const GEMINI: Constellation =  Constellation {
    name: "Gemini",
    pattern: [
        "                 *            ",
        "     *                        ",
        "            *                 ",
        "                   *         *",
        "                          * * ",
        "  *     *                     ",
        "     *                   *    ",
        "                              ",
        "   *       *     *            ",
        "                             ",
        "                         *     ",
        "              *               ",
        "                       *      ",
    ]
};

pub const ARIES: Constellation =  Constellation {
    name: "Aries",
    pattern: [
        "                              ",
        "                              ",
        "                              ",
        "   *                          ",
        "                  *           ",
        "                              ",
        "                        *     ",
        "                         *    ",
        "                              ",
        "                              ",
        "                              ",
        "                              ",
        "                              ",
    ]
};

pub const CAPRICORN: Constellation =  Constellation {
    name: "Capricorn",
    pattern: [
        "                              ",
        "                           *  ",
        " *                        *   ",
        "     *          *             ",
        "                              ",
        "                       *      ",
        "                              ",
        "        *                     ",
        "                              ",
        "              *     *         ",
        "                   *          ",
        "                              ",
        "                              ",
    ]
};

pub const PISCES: Constellation =  Constellation {
    name: "Pisces",
    pattern: [
        "                              ",
        "          *                   ",
        "         *                    ",
        "          *                   ",
        "                              ",
        "                              ",
        "        *                     ",
        "                              ",
        "      *                       ",
        "             *      *    *    ",
        "       *                *   * ",
        "  *                     *  *  ",
        "                              ",
    ]
};

pub const SAGGITTARIUS: Constellation =  Constellation {
    name: "Saggittarius",
    pattern: [
        "       *                      ",
        "         *                    ",
        "               *          *   ",
        "             *                ",
        "                      *       ",
        "      *        *  *           ",
        "             *        *      *",
        " *              *         *   ",
        "                     *        ",
        "   *                   *      ",
        "          *                   ",
        "     *                        ",
        "          *                   ",
    ]
};

pub const SCORPIO: Constellation =  Constellation {
    name: "Scorpio",
    pattern: [
        "                              ",
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
        "                              ",

    ]
};

pub const VIRGO: Constellation =  Constellation {
    name: "Virgo",
    pattern: [
        "                              ",
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
        "                              ",

    ]
};

pub const LIBRA: Constellation =  Constellation {
    name: "Libra",
    pattern: [
        "                              ",
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
        "                              ",

    ]
};

pub const CANCER: Constellation =  Constellation {
    name: "Cancer",
    pattern: [
        "                              ",
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
        "                              ",
    ]
};

pub const BIG_DIPPER: Constellation =  Constellation {
    name: "Big Dipper",
    pattern: [
        "                              ",
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
        "                              ",
    ]
};

pub const LEPUS: Constellation =  Constellation {
    name: "Lepus",
    pattern: [
        "                              ",
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
        "                              ",
    ]
};

pub const AQUARIUS: Constellation =  Constellation {
    name: "Aquarius",
    pattern: [
        "                              ",
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
        "                              ",

    ]
};

pub const UNKNOWN: Constellation =  Constellation {
    name: "UNKNOWN",
    pattern: [
        "                              ",
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
        "                              ",

    ]
};