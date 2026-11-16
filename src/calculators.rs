use askama::Template;
use std::collections::HashMap;

#[derive(Template)]
#[template(path = "calculator.html")]
pub struct CalculatorTemplate {
    pub title: String,
    pub nav: Vec<NavItem>,
    pub sections: Vec<Section>,
}

#[derive(Clone)]
pub struct NavItem {
    pub slug: String,
    pub label: String,
    pub active: bool,
}

pub struct Field {
    pub name: String,
    pub label: String,
    pub value: String,
}

pub struct Section {
    pub heading: String,
    pub fields: Vec<Field>,
    pub outputs: Vec<String>,
    pub note: String,
}

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate {
    pub nav: Vec<NavItem>,
}

pub fn navigation(current: Option<&str>) -> Vec<NavItem> {
    let pages = [
        ("trapesium", "TRAPESIUM"),
        ("translasi", "TRANSLASI"),
        ("suhu", "SUHU"),
        ("spldv", "SISTEM PERSAMAAN LINEAR DUA VARIABEL"),
        ("segitiga-siku", "SEGITIGA SIKU-SIKU"),
        ("segitiga-sama-kaki", "SEGITIGA SAMA KAKI"),
        ("rotasi", "ROTASI"),
        ("persegi-panjang", "PERSEGI PANJANG"),
        ("persegi", "PERSEGI"),
        ("persamaan-kuadrat", "PERSAMAAN FUNGSI KUADRAT"),
        ("kubus", "KUBUS"),
        ("bola", "BOLA"),
        ("balok", "BANGUN RUANG BALOK"),
    ];

    std::iter::once(NavItem {
        slug: String::new(),
        label: "BERANDA".to_owned(),
        active: current.is_none(),
    })
    .chain(pages.into_iter().map(|(slug, label)| NavItem {
        slug: slug.to_owned(),
        label: label.to_owned(),
        active: current == Some(slug),
    }))
    .collect()
}

fn section(
    heading: &str,
    definitions: &[(&str, &str)],
    form: Option<&HashMap<String, String>>,
) -> Result<(Section, Vec<f64>), String> {
    let mut fields = Vec::with_capacity(definitions.len());
    let mut values = Vec::with_capacity(definitions.len());

    for (name, label) in definitions {
        let value = match form {
            Some(form) => {
                let raw = form
                    .get(*name)
                    .ok_or_else(|| format!("Missing numeric input: {label}"))?;
                raw.parse::<f64>()
                    .map_err(|_| format!("Invalid numeric input for {label}"))?
            }
            None => 0.0,
        };
        fields.push(Field {
            name: (*name).to_owned(),
            label: (*label).to_owned(),
            value: form
                .and_then(|values| values.get(*name))
                .cloned()
                .unwrap_or_else(|| "0".to_owned()),
        });
        values.push(value);
    }

    Ok((
        Section {
            heading: heading.to_owned(),
            fields,
            outputs: Vec::new(),
            note: String::new(),
        },
        values,
    ))
}

fn output(section: &mut Section, label: &str, value: f64) {
    if label.is_empty() {
        section.outputs.push(value.to_string());
    } else {
        section.outputs.push(format!("{label}: {value}"));
    }
}

fn coordinate(section: &mut Section, x: f64, y: f64) {
    section.outputs.push(format!("({}, {})", x, y));
}

pub fn build_page(
    slug: &str,
    form: Option<&HashMap<String, String>>,
) -> Option<Result<CalculatorTemplate, String>> {
    let (title, sections) = match slug {
        "trapesium" => {
            let (mut perimeter, sides) = match section(
                "KELILING",
                &[
                    ("a", "Masukan nilai sisi A"),
                    ("b", "Masukan nilai sisi B"),
                    ("c", "Masukan nilai sisi C"),
                    ("d", "Masukan nilai sisi D"),
                ],
                form,
            ) {
                Ok(values) => values,
                Err(error) => return Some(Err(error)),
            };
            output(
                &mut perimeter,
                "",
                sides[0] + sides[1] + sides[2] + sides[3],
            );
            let (mut area, bases) = match section(
                "LUAS",
                &[
                    ("alas_a", "Masukan Nilai Alas A:"),
                    ("alas_b", "Masukan Nilai Alas B:"),
                    ("tinggi", "Masukan Nilai Tinggi:"),
                ],
                form,
            ) {
                Ok(values) => values,
                Err(error) => return Some(Err(error)),
            };
            output(&mut area, "", 0.5 * (bases[0] + bases[1]) * bases[2]);
            ("TRAPESIUM", vec![perimeter, area])
        }
        "translasi" => {
            let (mut origin, values) = match section(
                "MENCARI TITIK AWAL",
                &[
                    ("x1_awal", "Masukan Nilai X1"),
                    ("y1_awal", "Masukan Nilai Y1"),
                    ("a_awal", "Masukan Nilai a"),
                    ("b_awal", "Masukan Nilai b"),
                ],
                form,
            ) {
                Ok(values) => values,
                Err(error) => return Some(Err(error)),
            };
            coordinate(&mut origin, values[0] - values[2], values[1] - values[3]);

            let (mut translation, values) = match section(
                "MENCARI TITIK TRANSLASI",
                &[
                    ("x1_translasi", "Masukan Nilai X1 "),
                    ("y1_translasi", "Masukan Nilai Y1 "),
                    ("x_translasi", "Masukan Nilai X"),
                    ("y_translasi", "Masukan Nilai Y"),
                ],
                form,
            ) {
                Ok(values) => values,
                Err(error) => return Some(Err(error)),
            };
            coordinate(
                &mut translation,
                values[0] - values[2],
                values[1] - values[3],
            );

            let (mut image, values) = match section(
                "MENCARI TITIK BAYANGAN",
                &[
                    ("a_bayangan", "Masukan Nilai a "),
                    ("b_bayangan", "Masukan Nilai b "),
                    ("x_bayangan", "Masukan Nilai x "),
                    ("y_bayangan", "Masukan Nilai y "),
                ],
                form,
            ) {
                Ok(values) => values,
                Err(error) => return Some(Err(error)),
            };
            coordinate(&mut image, values[0] + values[2], values[1] + values[3]);
            ("TRANSLASI", vec![origin, translation, image])
        }
        "suhu" => {
            let (mut conversion, values) =
                match section("KONVERSI", &[("celsius", "Masukan Nilai Celcius")], form) {
                    Ok(values) => values,
                    Err(error) => return Some(Err(error)),
                };
            let celsius = values[0];
            output(&mut conversion, "Fahrenheit", (9.0 / 5.0) * celsius + 32.0);
            output(&mut conversion, "Kelvin", celsius + 273.15);
            ("SUHU", vec![conversion])
        }
        "spldv" => {
            let development_note = Section {
                heading: "Masih dalam tahap pengembangan".to_owned(),
                fields: Vec::new(),
                outputs: Vec::new(),
                note: String::new(),
            };
            let (equations, values) = match section(
                "Persamaan 1: ax+by=c",
                &[
                    ("a", "Masukan Nilai Variabel a"),
                    ("b", "Masukan Nilai Variabel b"),
                    ("c", "Masukan Nilai Variabel c"),
                ],
                form,
            ) {
                Ok(values) => values,
                Err(error) => return Some(Err(error)),
            };
            let (second_equation, second) = match section(
                "Persamaan 2: px+qy=r",
                &[
                    ("p", "Masukan Nilai Variabel p"),
                    ("q", "Masukan Nilai Variabel q"),
                    ("r", "Masukan Nilai Variabel r"),
                ],
                form,
            ) {
                Ok(values) => values,
                Err(error) => return Some(Err(error)),
            };
            let (a, b, c) = (values[0], values[1], values[2]);
            let (p, q, r) = (second[0], second[1], second[2]);
            let denominator = a * q - p * b;
            let (_x, _y) = if denominator == 0.0 || b == 0.0 {
                (0.0, 0.0)
            } else {
                let x = (c * q - r * b) / denominator;
                (x, (1.0 / b) * (c - a * x))
            };
            let mut second_equation = second_equation;
            second_equation.note = "Note: AI KAKURU akan mencari variabel x dengan metode eliminasi lalu mencari variabel y dengan metode subtitusi".to_owned();
            (
                "SISTEM PERSAMAAN LINEAR DUA VARIABEL",
                vec![development_note, equations, second_equation],
            )
        }
        "segitiga-siku" => {
            let (mut perimeter, sides) = match section(
                "KELILING",
                &[
                    ("sisi_1", "Masukan Panjang sisi 1"),
                    ("sisi_2", "Masukan Panjang sisi 2"),
                    ("sisi_3", "Masukan Panjang sisi 3"),
                ],
                form,
            ) {
                Ok(values) => values,
                Err(error) => return Some(Err(error)),
            };
            output(&mut perimeter, "", sides.iter().sum());
            let (mut area, values) = match section(
                "LUAS",
                &[
                    ("alas", "Masukan panjang Alas"),
                    ("tinggi", "Masukan Tinggi"),
                ],
                form,
            ) {
                Ok(values) => values,
                Err(error) => return Some(Err(error)),
            };
            output(&mut area, "", values[0] * values[1] * 1.0 / 2.0);
            let (mut base, values) =
                match section("MENCARI ALAS", &[("keliling", "Masukan Keliling:")], form) {
                    Ok(values) => values,
                    Err(error) => return Some(Err(error)),
                };
            output(&mut base, "", values[0] / 3.0);
            let (mut height, values) =
                match section("MENCARI TINGGI", &[("luas", "Masukan Nilai luas")], form) {
                    Ok(values) => values,
                    Err(error) => return Some(Err(error)),
                };
            output(&mut height, "", values[0] * 1.0 / 2.0);
            ("SEGITIGA SIKU-SIKU", vec![perimeter, area, base, height])
        }
        "segitiga-sama-kaki" => {
            let (mut perimeter, sides) = match section(
                "KELILING",
                &[
                    ("sisi_1", "Masukan Panjang Sisi 1:"),
                    ("sisi_2", "Masukan Panjang Sisi 2:"),
                    ("sisi_3", "Masukan Panjang Sisi 3:"),
                ],
                form,
            ) {
                Ok(values) => values,
                Err(error) => return Some(Err(error)),
            };
            output(&mut perimeter, "", sides.iter().sum());
            let (mut area, values) = match section(
                "LUAS",
                &[
                    ("alas", "Masukan Nilai Alas:"),
                    ("tinggi", "Masukan Nilai Tinggi:"),
                ],
                form,
            ) {
                Ok(values) => values,
                Err(error) => return Some(Err(error)),
            };
            output(&mut area, "", 0.5 * values[0] * values[1]);
            ("SEGITIGA SAMA KAKI", vec![perimeter, area])
        }
        "rotasi" => {
            let (mut rotations, values) = match section(
                "",
                &[("x", "Masukan Nilai x"), ("y", "Masukan Nilai y")],
                form,
            ) {
                Ok(values) => values,
                Err(error) => return Some(Err(error)),
            };
            rotations.heading.clear();
            rotations.outputs.push(format!(
                "(+90/-270): sumbu x: {}, sumbu y: {}",
                -values[1], values[0]
            ));
            rotations.outputs.push(format!(
                "(180): sumbu x: {}, sumbu y: {}",
                values[1], -values[0]
            ));
            rotations.outputs.push(format!(
                "(-90/+270): sumbu x: {}, sumbu y: {}",
                -values[0], -values[1]
            ));
            ("ROTASI", vec![rotations])
        }
        "persegi-panjang" => {
            let (mut rectangle, values) = match section(
                "KELILING",
                &[
                    ("panjang", "Masukan Nilai Panjang:"),
                    ("lebar", "Masukan Nilai Lebar:"),
                ],
                form,
            ) {
                Ok(values) => values,
                Err(error) => return Some(Err(error)),
            };
            output(&mut rectangle, "", 2.0 * values[0] + 2.0 * values[1]);
            let empty_heading = Section {
                heading: "MENCARI PANJANG".to_owned(),
                fields: Vec::new(),
                outputs: Vec::new(),
                note: String::new(),
            };
            let mut area = Section {
                heading: "LUAS".to_owned(),
                fields: Vec::new(),
                outputs: Vec::new(),
                note: String::new(),
            };
            output(&mut area, "", values[0] * values[1]);
            ("PERSEGI PANJANG", vec![rectangle, empty_heading, area])
        }
        "persegi" => {
            let (mut square, values) =
                match section("KELILING", &[("sisi", "Masukan Panjang Sisi:")], form) {
                    Ok(values) => values,
                    Err(error) => return Some(Err(error)),
                };
            output(&mut square, "", values[0].powi(2));
            let mut area = Section {
                heading: "LUAS".to_owned(),
                fields: Vec::new(),
                outputs: Vec::new(),
                note: String::new(),
            };
            output(&mut area, "", 4.0 * values[0]);
            ("PERSEGI", vec![square, area])
        }
        "persamaan-kuadrat" => {
            let (mut discriminant, values) = match section(
                "DISKRIMINAN",
                &[
                    ("b_discriminant", "Masukan Nilai b"),
                    ("a_discriminant", "Masukan Nilai a"),
                    ("c_discriminant", "Masukan Nilai c"),
                ],
                form,
            ) {
                Ok(values) => values,
                Err(error) => return Some(Err(error)),
            };
            discriminant.outputs.push("Format: (b² -4 a c)".to_owned());
            output(
                &mut discriminant,
                "",
                -values[0].powi(2) - (4.0 * values[1] * values[2]),
            );
            let (mut axis, values) = match section(
                "SUMBU SIMETRI",
                &[
                    ("b_axis", "Masukan Nilai b:"),
                    ("a_axis", "Masukan Nilai a:"),
                ],
                form,
            ) {
                Ok(values) => values,
                Err(error) => return Some(Err(error)),
            };
            axis.outputs.push("Format: -b/2a".to_owned());
            output(
                &mut axis,
                "",
                if values[1] == 0.0 {
                    0.0
                } else {
                    -values[0] / (2.0 * values[1])
                },
            );
            let (mut optimum, values) = match section(
                "NILAI OPTIMUM",
                &[
                    ("b_optimum", "Masukan Nilai b "),
                    ("a_optimum", "Masukan Nilai a "),
                    ("c_optimum", "Masukan Nilai c "),
                ],
                form,
            ) {
                Ok(values) => values,
                Err(error) => return Some(Err(error)),
            };
            optimum
                .outputs
                .push("Format: -D/4a [-(b² - 4ac/4a)]".to_owned());
            output(
                &mut optimum,
                "",
                if values[1] == 0.0 {
                    0.0
                } else {
                    -(values[0].powi(2) - 4.0 * values[1] * values[2]) / (4.0 * values[1])
                },
            );
            (
                "PERSAMAAN FUNGSI KUADRAT",
                vec![discriminant, axis, optimum],
            )
        }
        "kubus" => {
            let (mut cube, values) =
                match section("KELILING", &[("sisi", "Masukan Panjang Sisi:")], form) {
                    Ok(values) => values,
                    Err(error) => return Some(Err(error)),
                };
            let side = values[0];
            output(&mut cube, "", 12.0 * side);
            let mut area = Section {
                heading: "LUAS".to_owned(),
                fields: Vec::new(),
                outputs: Vec::new(),
                note: String::new(),
            };
            output(&mut area, "", 6.0 * side);
            let mut volume = Section {
                heading: "VOLUME".to_owned(),
                fields: Vec::new(),
                outputs: Vec::new(),
                note: String::new(),
            };
            output(&mut volume, "", side * side * side);
            ("KUBUS", vec![cube, area, volume])
        }
        "bola" => {
            let (mut sphere, values) = match section(
                "KELILING BOLA",
                &[("radius", "Masukan Panjang Jari - jari")],
                form,
            ) {
                Ok(values) => values,
                Err(error) => return Some(Err(error)),
            };
            let radius = values[0];
            sphere.outputs.push(format!(
                "{} & {}",
                2.0 * 22.0 / 7.0 * radius,
                2.0 * 3.14 * radius
            ));
            let mut area = Section {
                heading: "LUAS BOLA".to_owned(),
                fields: Vec::new(),
                outputs: Vec::new(),
                note: String::new(),
            };
            area.outputs.push(format!(
                "{} & {}",
                4.0 * 22.0 / 7.0 * radius.powi(2),
                4.0 * 3.14 * radius.powi(2)
            ));
            let mut volume = Section {
                heading: "VOLUME BOLA".to_owned(),
                fields: Vec::new(),
                outputs: Vec::new(),
                note: "Kiri menggunakan 22/7, kanan menggunakan 3.14".to_owned(),
            };
            volume.outputs.push(format!(
                "{} & {}",
                4.0 / 3.0 * 22.0 / 7.0 * radius.powi(3),
                4.0 / 3.0 * std::f64::consts::PI * radius.powi(3)
            ));
            ("BOLA", vec![sphere, area, volume])
        }
        "balok" => {
            let (mut cuboid, values) = match section(
                "KELILING",
                &[
                    ("panjang", "Masukan Nilai Panjang:"),
                    ("lebar", "Masukan Nilai Lebar:"),
                    ("tinggi", "Masukan Nilai Tinggi:"),
                ],
                form,
            ) {
                Ok(values) => values,
                Err(error) => return Some(Err(error)),
            };
            let (length, width, height) = (values[0], values[1], values[2]);
            output(&mut cuboid, "", 4.0 * (length + width + height));
            let partial_area = length * width + length * height + width * height;
            let area = Section {
                heading: "LUAS BALOK".to_owned(),
                fields: Vec::new(),
                outputs: vec![partial_area.to_string()],
                note: String::new(),
            };
            let volume = length * partial_area * height;
            let volume = Section {
                heading: "VOLUME BALOK".to_owned(),
                fields: Vec::new(),
                outputs: vec![volume.to_string()],
                note: String::new(),
            };
            cuboid.heading = "KELILING".to_owned();
            ("BANGUN RUANG BALOK", vec![cuboid, area, volume])
        }
        _ => return None,
    };

    Some(Ok(CalculatorTemplate {
        title: title.to_owned(),
        nav: navigation(Some(slug)),
        sections,
    }))
}

#[cfg(test)]
mod tests {
    use super::build_page;
    use std::collections::HashMap;

    fn form(values: &[(&str, &str)]) -> HashMap<String, String> {
        values
            .iter()
            .map(|(name, value)| ((*name).to_owned(), (*value).to_owned()))
            .collect()
    }

    fn outputs(slug: &str, values: &[(&str, &str)]) -> Vec<String> {
        build_page(slug, Some(&form(values)))
            .expect("calculator route should exist")
            .expect("inputs should be valid")
            .sections
            .into_iter()
            .flat_map(|section| section.outputs)
            .collect()
    }

    #[test]
    fn square_keeps_the_existing_perimeter_and_area_formulas() {
        assert_eq!(
            outputs("persegi", &[("sisi", "3")]),
            vec!["9".to_owned(), "12".to_owned()]
        );
    }

    #[test]
    fn cuboid_volume_uses_the_overwritten_width_value() {
        assert_eq!(
            outputs(
                "balok",
                &[("panjang", "2"), ("lebar", "3"), ("tinggi", "4")]
            ),
            vec!["36".to_owned(), "26".to_owned(), "208".to_owned()]
        );
    }

    #[test]
    fn quadratic_discriminant_preserves_its_current_sign() {
        assert_eq!(
            outputs(
                "persamaan-kuadrat",
                &[
                    ("b_discriminant", "2"),
                    ("a_discriminant", "1"),
                    ("c_discriminant", "1"),
                    ("b_axis", "2"),
                    ("a_axis", "1"),
                    ("b_optimum", "2"),
                    ("a_optimum", "1"),
                    ("c_optimum", "1"),
                ]
            ),
            vec![
                "Format: (b² -4 a c)".to_owned(),
                "-8".to_owned(),
                "Format: -b/2a".to_owned(),
                "-1".to_owned(),
                "Format: -D/4a [-(b² - 4ac/4a)]".to_owned(),
                "-0".to_owned(),
            ]
        );
    }

    #[test]
    fn every_calculator_is_in_navigation() {
        assert_eq!(super::navigation(None).len(), 14);
    }

    #[test]
    fn every_navigation_calculator_renders_with_default_inputs() {
        for item in super::navigation(None).into_iter().skip(1) {
            let page = build_page(&item.slug, None)
                .expect("navigation entry should have a route")
                .expect("default values should render");
            assert!(!page.title.is_empty());
        }
    }

    #[test]
    fn invalid_numeric_inputs_are_reported() {
        let invalid = form(&[("celsius", "not a number")]);
        assert!(matches!(
            build_page("suhu", Some(&invalid)),
            Some(Err(error)) if error == "Invalid numeric input for Masukan Nilai Celcius"
        ));
    }
}
