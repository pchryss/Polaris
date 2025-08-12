pub struct Constellation {
    pub name: &'static str,
    pub pattern: [&'static str; 11],
}

pub const CONSTELLATIONS : &[Constellation] = &[
    BIG_DIPPER,
    LEPUS,
    AQUARIUS,
];

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