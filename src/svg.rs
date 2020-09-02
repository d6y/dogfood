use crate::diagram::Diagram;
use crate::Fraction;
use num_traits::cast::ToPrimitive;
use svg::node;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::node::element::Rectangle;
use svg::Document;

// Interpret the diagram into an SVG and save to a file
pub fn save(diagram: &Diagram, filename: &str) -> Result<(), std::io::Error> {
    let (canvas, document) = draw(&diagram, Canvas::blank(), Document::new());
    let viewbox = format!("-10, -10, {}, {}", canvas.width, canvas.height);
    svg::save(filename, &document.set("viewBox", viewbox))
}

// Implementation outline:
// - As we walk the `diagram`, we draw at the current position (x,y) of the `canvas`.
// - As we compose parts of the `diagram` we move and expand the `canvas`.
// The `canvas` keeps our position while we actually update the SVG `document` object.
// You'll see in the code below we return `(canvas, document)` as we we navigate the diagram.
// I could have split `canvas` into "pen" and "bounds".
struct Canvas {
    x: i16,
    y: i16,
    width: i16,
    height: i16,
}

impl Canvas {
    fn blank() -> Canvas {
        Canvas {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }

    fn grow(self, w: i16, h: i16) -> Canvas {
        Canvas {
            width: self.width + w,
            height: self.height + h,
            ..self
        }
    }

    fn reposition(self, x: i16, y: i16) -> Canvas {
        Canvas { x, y, ..self }
    }
}

fn draw(diagram: &Diagram, canvas: Canvas, document: Document) -> (Canvas, Document) {
    match diagram {
        Diagram::Blank => (canvas, document),
        Diagram::Can(fraction) => rectangle(canvas, document, fraction),
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

    let _text_width = label.chars().count(); // TODO: need font info for sizing
    let text_height = 20; // TODO: need font info
    let text_x_offset = 0; // TODO: for centering, need font info

    let node = node::element::Text::new()
        .set("x", canvas.x + text_x_offset)
        .set("y", canvas.y + text_height)
        .add(node::Text::new(label));

    (canvas.grow(0, text_height * 2), document.add(node))
}

fn rectangle(canvas: Canvas, document: Document, contents: &Fraction) -> (Canvas, Document) {
    let width = 50;
    let height = 60;

    let data = Data::new()
        .move_to((canvas.x, canvas.y))
        .line_by((width, 0))
        .line_by((0, height))
        .line_by((-width, 0))
        .close();

    let outline = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("opacity", 0.8)
        .set("stroke-width", 3)
        .set("d", data);

    // Shading the inside of the can: how much gap (empty space) at the top of the can?
    let gap_at_top = (height as f32
        * (Fraction::new(1, 1) - contents)
            .to_f32()
            .unwrap_or_default())
    .round() as i16;

    let inner = Rectangle::new()
        .set("x", canvas.x)
        .set("y", canvas.y + gap_at_top)
        .set("width", width)
        .set("height", height - gap_at_top)
        .set("fill", "silver");

    (canvas.grow(width, height), document.add(inner).add(outline))
}
