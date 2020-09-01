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
}

impl Diagram {
    pub fn new() -> Diagram {
        Diagram::Blank
    }

    pub fn label(text: String) -> Diagram {
        Diagram::Label { text: text }
    }

    pub fn can() -> Diagram {
        Diagram::Can
    }

    pub fn above(self, bottom: Diagram) -> Diagram {
        Diagram::Stack {
            top: Box::new(self),
            bottom: Box::new(bottom),
        }
    }
}
