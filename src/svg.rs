use super::can::FULL;
use num_rational::Rational;
use svg::node;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

pub fn draw(filename: &str, days: &Vec<Rational>, labels: &Vec<String>) {
    let h_margin = 0;
    let v_margin = 50;

    let day_height = 120;
    let am_width = 90;

    let can_height = 60;
    let can_width = 50;

    let day_origin = |day: usize, am: bool| {
        if am {
            (h_margin, day_height * day + v_margin)
        } else {
            (h_margin + am_width, day_height * day + v_margin)
        }
    };

    let can = |origin| {
        let data = Data::new()
            .move_to(origin)
            .line_by((can_width, 0))
            .line_by((0, can_height))
            .line_by((-can_width, 0))
            .close();

        let path = Path::new()
            .set("fill", "none")
            .set("stroke", "black")
            .set("stroke-width", 3)
            .set("d", data);

        path
    };

    let text_under = |origin, str: &str| {
        let (x, y) = origin;
        node::element::Text::new()
            .set("x", x + (can_width / 2) - (8 * str.len() as i32 / 2)) // TODO: arbitrary, fix for font
            .set("y", y + (can_height as f32 * 1.3).round() as usize)
            .add(node::Text::new(str))
    };

    let mut document = Document::new().set(
        "viewBox",
        (
            0,
            0,
            am_width * 2 + h_margin * 2,
            days.len() * day_height + days.len() + v_margin * 2,
        ),
    );

    let mut am = *FULL;
    for (i, day) in days.iter().enumerate() {
        let am_origin = day_origin(i, true);
        let am_can = can(am_origin);

        let pm_origin = day_origin(i, false);
        let pm_can = can(pm_origin);

        let pm_label = text_under(pm_origin, &labels[i]);

        document = document.add(am_can).add(pm_can).add(pm_label);
    }

    svg::save(filename, &document).unwrap();
}
