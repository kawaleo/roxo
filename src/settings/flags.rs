#[derive(Debug)]
pub struct Flag {
    pub flag: [&'static str; 2],
    pub takes: Input,
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Input {
    Invalid,
    Required(String),
    Optional(Option<String>),
}

pub fn get_flags() -> Vec<Flag> {
    vec![
        Flag {
            flag: ["-a", "--all"],
            takes: Input::Invalid,
        },
        Flag {
            flag: ["-s", "--size"],
            takes: Input::Invalid,
        },
        Flag {
            flag: ["-t", "--time"],
            takes: Input::Invalid,
        },
        Flag {
            flag: ["-h", "--hide"],
            takes: Input::Invalid,
        },
    ]
}
