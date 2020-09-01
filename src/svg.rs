use crate::diagram::Diagram;
use svg::node;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

pub fn save(diagram: &Diagram, filename: &str) {
    let (canvas, document) = draw(&diagram, Canvas::blank(), Document::new());
    let viewbox = format!("-10, -10, {}, {}", canvas.width, canvas.height);
    svg::save(filename, &document.set("viewBox", viewbox)).unwrap();
}

struct Canvas {
    x: i16,
    y: i16,
    width: i16,
    height: i16,
}

impl Canvas {
    fn new(x: i16, y: i16, width: i16, height: i16) -> Canvas {
        Canvas {
            x,
            y,
            width,
            height,
        }
    }

    fn blank() -> Canvas {
        Canvas {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }

    fn grow(self, w: i16, h: i16) -> Canvas {
        Canvas::new(self.x, self.y, self.width + w, self.height + h)
    }

    fn reposition(self, x: i16, y: i16) -> Canvas {
        Canvas::new(x, y, self.width, self.height)
    }
}

fn draw(diagram: &Diagram, canvas: Canvas, document: Document) -> (Canvas, Document) {
    match diagram {
        Diagram::Blank => (canvas, document),
        Diagram::Can => rect(canvas, document),
        Diagram::Label { text } => self::text(canvas, document, text),
        Diagram::Stack { top, bottom } => {
            let (canvas, document) = draw(&top, canvas, document);
            let next_line = canvas.height;
            draw(&bottom, canvas.reposition(0, next_line), document)
        }
    }
}

fn text(canvas: Canvas, document: Document, label: &str) -> (Canvas, Document) {
    // NB: text is drawn up from the y baseline

    let _text_width = label.len(); // TODO: need font info for sizing
    let text_height = 20; // TODO: need font info
    let text_x_offset = 0; // TODO: for centering, need font info

    let node = node::element::Text::new()
        .set("x", canvas.x + text_x_offset)
        .set("y", canvas.y + text_height)
        .add(node::Text::new(label));

    (canvas.grow(0, text_height * 2), document.add(node))
}

fn rect(canvas: Canvas, document: Document) -> (Canvas, Document) {
    let width = 20;
    let height = 40;

    let data = Data::new()
        .move_to((canvas.x, canvas.y))
        .line_by((width, 0))
        .line_by((0, height))
        .line_by((-width, 0))
        .close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 2)
        .set("d", data);

    (canvas.grow(width, height), document.add(path))
}
