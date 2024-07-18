use crate::block::Block;
#[allow(unused_imports)]
use crate::block::BlockType::{Once, Periodic}; // Signal, PeriodicOrSignal
#[allow(unused_imports)]
use crate::block::CommandType::{Function, Shell};

use crate::blocks::cpu::cpu_usage;
//use crate::blocks::datetime::current_date;
//use crate::blocks::datetime::current_time;
use crate::blocks::memory::memory_used;

pub const SEPARATOR: &str = " │ ";
pub const PREFIX: &str = " ";
pub const SUFFIX: &str = " ";

pub const BLOCKS: &[Block] = &[
    Block {
        kind: Periodic(30),
        command: Shell(&["sb-battery"]),
        prefix: " ",
        suffix: "",
    },
    Block {
        kind: Periodic(1),
        command: Function(cpu_usage),
        prefix: "  ",
        suffix: "",
    },
    Block {
        kind: Periodic(1),
        command: Function(memory_used),
        prefix: "󱞟 ",
        suffix: "G",
    },
    Block {
        kind: Periodic(30),
        command: Shell(&["date", "+%a %d %b %Y   %I:%M %p"]),
        prefix: "  ",
        suffix: "",
    },
    // Block {
    //     kind: Once,
    //     command: Function(current_date),
    //     prefix: "  ",
    //     suffix: "",
    // },
    // Block {
    //     kind: Periodic(30),
    //     command: Function(current_time),
    //     prefix: "  ",
    //     suffix: "",
    // },
];
