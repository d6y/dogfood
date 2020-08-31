#[derive(Debug)]
pub enum Diagram {
    Blank,
    Label {
        text: String,
    },
    Can,
    Stack {
        top: Box<Diagram>,
        bottom: Box<Diagram>,
    },
    Pair {
        left: Box<Diagram>,
        right: Box<Diagram>,
    },
}

impl Diagram {
    pub fn new() -> Diagram {
        Diagram::Blank
    }

    pub fn label(text: &String) -> Diagram {
        Diagram::Label {
            text: text.to_owned(),
        }
    }

    pub fn can() -> Diagram {
        Diagram::Can
    }

    pub fn next_to(self, right: Diagram) -> Diagram {
        Diagram::Pair {
            left: Box::new(self),
            right: Box::new(right),
        }
    }

    pub fn above(self, bottom: Diagram) -> Diagram {
        Diagram::Stack {
            top: Box::new(self),
            bottom: Box::new(bottom),
        }
    }
}
