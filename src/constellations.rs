pub struct Constellation {
    pub name: &'static str,
    pub pattern: [&'static str; 7],
}

pub const CONSTELLATIONS : &[Constellation] = &[
    BIG_DIPPER
];

pub const BIG_DIPPER: Constellation =  Constellation {
    name: "Big Dipper",
    pattern: [
        "                         ",
        "    *                    ",
        " *     *            *    ",
        "           *             ",
        "                         ",
        "             *       *   ",
        "                         ",
    ]
};