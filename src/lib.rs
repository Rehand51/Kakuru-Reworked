#![allow(clippy::too_many_arguments)]

use std::collections::HashMap;

use eframe::egui::{self, Color32, RichText};

const NAVY: Color32 = Color32::from_rgb(23, 58, 83);
const GREEN: Color32 = Color32::from_rgb(4, 170, 109);

#[derive(Clone, Copy)]
struct CalculatorInfo {
    slug: &'static str,
    label: &'static str,
}

const CALCULATORS: [CalculatorInfo; 13] = [
    CalculatorInfo {
        slug: "trapesium",
        label: "TRAPESIUM",
    },
    CalculatorInfo {
        slug: "translasi",
        label: "TRANSLASI",
    },
    CalculatorInfo {
        slug: "suhu",
        label: "SUHU",
    },
    CalculatorInfo {
        slug: "spldv",
        label: "SISTEM PERSAMAAN LINEAR DUA VARIABEL",
    },
    CalculatorInfo {
        slug: "segitiga-siku",
        label: "SEGITIGA SIKU-SIKU",
    },
    CalculatorInfo {
        slug: "segitiga-sama-kaki",
        label: "SEGITIGA SAMA KAKI",
    },
    CalculatorInfo {
        slug: "rotasi",
        label: "ROTASI",
    },
    CalculatorInfo {
        slug: "persegi-panjang",
        label: "PERSEGI PANJANG",
    },
    CalculatorInfo {
        slug: "persegi",
        label: "PERSEGI",
    },
    CalculatorInfo {
        slug: "persamaan-kuadrat",
        label: "PERSAMAAN FUNGSI KUADRAT",
    },
    CalculatorInfo {
        slug: "kubus",
        label: "KUBUS",
    },
    CalculatorInfo {
        slug: "bola",
        label: "BOLA",
    },
    CalculatorInfo {
        slug: "balok",
        label: "BANGUN RUANG BALOK",
    },
];

#[derive(Clone)]
struct FieldSpec {
    name: &'static str,
    label: &'static str,
}

#[derive(Clone)]
struct SectionSpec {
    heading: &'static str,
    fields: Vec<FieldSpec>,
    note: &'static str,
}

struct SectionResult {
    heading: &'static str,
    outputs: Vec<String>,
}

fn field(name: &'static str, label: &'static str) -> FieldSpec {
    FieldSpec { name, label }
}

fn section(heading: &'static str, fields: Vec<FieldSpec>, note: &'static str) -> SectionSpec {
    SectionSpec {
        heading,
        fields,
        note,
    }
}

fn calculator_sections(slug: &str) -> Vec<SectionSpec> {
    use SectionSpec as S;
    match slug {
        "trapesium" => vec![
            section(
                "KELILING",
                vec![
                    field("a", "Masukan nilai sisi A"),
                    field("b", "Masukan nilai sisi B"),
                    field("c", "Masukan nilai sisi C"),
                    field("d", "Masukan nilai sisi D"),
                ],
                "",
            ),
            section(
                "LUAS",
                vec![
                    field("alas_a", "Masukan Nilai Alas A:"),
                    field("alas_b", "Masukan Nilai Alas B:"),
                    field("tinggi", "Masukan Nilai Tinggi:"),
                ],
                "",
            ),
        ],
        "translasi" => vec![
            section(
                "MENCARI TITIK AWAL",
                vec![
                    field("x1_awal", "Masukan Nilai X1"),
                    field("y1_awal", "Masukan Nilai Y1"),
                    field("a_awal", "Masukan Nilai a"),
                    field("b_awal", "Masukan Nilai b"),
                ],
                "",
            ),
            section(
                "MENCARI TITIK TRANSLASI",
                vec![
                    field("x1_translasi", "Masukan Nilai X1"),
                    field("y1_translasi", "Masukan Nilai Y1"),
                    field("x_translasi", "Masukan Nilai X"),
                    field("y_translasi", "Masukan Nilai Y"),
                ],
                "",
            ),
            section(
                "MENCARI TITIK BAYANGAN",
                vec![
                    field("a_bayangan", "Masukan Nilai a"),
                    field("b_bayangan", "Masukan Nilai b"),
                    field("x_bayangan", "Masukan Nilai x"),
                    field("y_bayangan", "Masukan Nilai y"),
                ],
                "",
            ),
        ],
        "suhu" => vec![section(
            "KONVERSI",
            vec![field("celsius", "Masukan Nilai Celcius")],
            "",
        )],
        "spldv" => vec![
            section("Masih dalam tahap pengembangan", vec![], ""),
            section(
                "Persamaan 1: ax+by=c",
                vec![
                    field("a", "Masukan Nilai Variabel a"),
                    field("b", "Masukan Nilai Variabel b"),
                    field("c", "Masukan Nilai Variabel c"),
                ],
                "",
            ),
            section(
                "Persamaan 2: px+qy=r",
                vec![
                    field("p", "Masukan Nilai Variabel p"),
                    field("q", "Masukan Nilai Variabel q"),
                    field("r", "Masukan Nilai Variabel r"),
                ],
                "Note: AI KAKURU akan mencari variabel x dengan metode eliminasi lalu mencari variabel y dengan metode subtitusi",
            ),
        ],
        "segitiga-siku" => vec![
            section(
                "KELILING",
                vec![
                    field("sisi_1", "Masukan Panjang sisi 1"),
                    field("sisi_2", "Masukan Panjang sisi 2"),
                    field("sisi_3", "Masukan Panjang sisi 3"),
                ],
                "",
            ),
            section(
                "LUAS",
                vec![
                    field("alas", "Masukan panjang Alas"),
                    field("tinggi", "Masukan Tinggi"),
                ],
                "",
            ),
            section(
                "MENCARI ALAS",
                vec![field("keliling", "Masukan Keliling:")],
                "",
            ),
            section(
                "MENCARI TINGGI",
                vec![field("luas", "Masukan Nilai luas")],
                "",
            ),
        ],
        "segitiga-sama-kaki" => vec![
            section(
                "KELILING",
                vec![
                    field("sisi_1", "Masukan Panjang Sisi 1:"),
                    field("sisi_2", "Masukan Panjang Sisi 2:"),
                    field("sisi_3", "Masukan Panjang Sisi 3:"),
                ],
                "",
            ),
            section(
                "LUAS",
                vec![
                    field("alas", "Masukan Nilai Alas:"),
                    field("tinggi", "Masukan Nilai Tinggi:"),
                ],
                "",
            ),
        ],
        "rotasi" => vec![section(
            "",
            vec![field("x", "Masukan Nilai x"), field("y", "Masukan Nilai y")],
            "",
        )],
        "persegi-panjang" => vec![
            section(
                "KELILING",
                vec![
                    field("panjang", "Masukan Nilai Panjang:"),
                    field("lebar", "Masukan Nilai Lebar:"),
                ],
                "",
            ),
            section("MENCARI PANJANG", vec![], ""),
            section("LUAS", vec![], ""),
        ],
        "persegi" => vec![
            section("KELILING", vec![field("sisi", "Masukan Panjang Sisi:")], ""),
            section("LUAS", vec![], ""),
        ],
        "persamaan-kuadrat" => vec![
            section(
                "DISKRIMINAN",
                vec![
                    field("b_discriminant", "Masukan Nilai b"),
                    field("a_discriminant", "Masukan Nilai a"),
                    field("c_discriminant", "Masukan Nilai c"),
                ],
                "",
            ),
            section(
                "SUMBU SIMETRI",
                vec![
                    field("b_axis", "Masukan Nilai b:"),
                    field("a_axis", "Masukan Nilai a:"),
                ],
                "",
            ),
            section(
                "NILAI OPTIMUM",
                vec![
                    field("b_optimum", "Masukan Nilai b"),
                    field("a_optimum", "Masukan Nilai a"),
                    field("c_optimum", "Masukan Nilai c"),
                ],
                "",
            ),
        ],
        "kubus" => vec![
            section("KELILING", vec![field("sisi", "Masukan Panjang Sisi:")], ""),
            section("LUAS", vec![], ""),
            section("VOLUME", vec![], ""),
        ],
        "bola" => vec![
            section(
                "KELILING BOLA",
                vec![field("radius", "Masukan Panjang Jari - jari")],
                "",
            ),
            section("LUAS BOLA", vec![], ""),
            section(
                "VOLUME BOLA",
                vec![],
                "Kiri menggunakan 22/7, kanan menggunakan 3.14",
            ),
        ],
        "balok" => vec![
            section(
                "KELILING",
                vec![
                    field("panjang", "Masukan Nilai Panjang:"),
                    field("lebar", "Masukan Nilai Lebar:"),
                    field("tinggi", "Masukan Nilai Tinggi:"),
                ],
                "",
            ),
            section("LUAS BALOK", vec![], ""),
            section("VOLUME BALOK", vec![], ""),
        ],
        _ => Vec::<S>::new(),
    }
}

fn number(value: f64) -> String {
    if value == 0.0 && value.is_sign_negative() {
        "-0".to_owned()
    } else {
        value.to_string()
    }
}

fn val(values: &HashMap<String, f64>, key: &str) -> f64 {
    values.get(key).copied().unwrap_or_default()
}

fn result(heading: &'static str, outputs: Vec<String>) -> SectionResult {
    SectionResult { heading, outputs }
}

fn calculate(slug: &str, values: &HashMap<String, f64>) -> Vec<SectionResult> {
    let v = |key| val(values, key);
    match slug {
        "trapesium" => vec![
            result("KELILING", vec![number(v("a") + v("b") + v("c") + v("d"))]),
            result(
                "LUAS",
                vec![number(0.5 * (v("alas_a") + v("alas_b")) * v("tinggi"))],
            ),
        ],
        "translasi" => vec![
            result(
                "MENCARI TITIK AWAL",
                vec![format!(
                    "({}, {})",
                    number(v("x1_awal") - v("a_awal")),
                    number(v("y1_awal") - v("b_awal"))
                )],
            ),
            result(
                "MENCARI TITIK TRANSLASI",
                vec![format!(
                    "({}, {})",
                    number(v("x1_translasi") - v("x_translasi")),
                    number(v("y1_translasi") - v("y_translasi"))
                )],
            ),
            result(
                "MENCARI TITIK BAYANGAN",
                vec![format!(
                    "({}, {})",
                    number(v("a_bayangan") + v("x_bayangan")),
                    number(v("b_bayangan") + v("y_bayangan"))
                )],
            ),
        ],
        "suhu" => vec![result(
            "KONVERSI",
            vec![
                format!("Fahrenheit: {}", number((9.0 / 5.0) * v("celsius") + 32.0)),
                format!("Kelvin: {}", number(v("celsius") + 273.15)),
            ],
        )],
        "spldv" => Vec::new(),
        "segitiga-siku" => vec![
            result(
                "KELILING",
                vec![number(v("sisi_1") + v("sisi_2") + v("sisi_3"))],
            ),
            result("LUAS", vec![number(v("alas") * v("tinggi") / 2.0)]),
            result("MENCARI ALAS", vec![number(v("keliling") / 3.0)]),
            result("MENCARI TINGGI", vec![number(v("luas") / 2.0)]),
        ],
        "segitiga-sama-kaki" => vec![
            result(
                "KELILING",
                vec![number(v("sisi_1") + v("sisi_2") + v("sisi_3"))],
            ),
            result("LUAS", vec![number(0.5 * v("alas") * v("tinggi"))]),
        ],
        "rotasi" => vec![result(
            "",
            vec![
                format!(
                    "(+90/-270): sumbu x: {}, sumbu y: {}",
                    number(-v("y")),
                    number(v("x"))
                ),
                format!(
                    "(180): sumbu x: {}, sumbu y: {}",
                    number(v("y")),
                    number(-v("x"))
                ),
                format!(
                    "(-90/+270): sumbu x: {}, sumbu y: {}",
                    number(-v("x")),
                    number(-v("y"))
                ),
            ],
        )],
        "persegi-panjang" => vec![
            result(
                "KELILING",
                vec![number(2.0 * v("panjang") + 2.0 * v("lebar"))],
            ),
            result("MENCARI PANJANG", vec![]),
            result("LUAS", vec![number(v("panjang") * v("lebar"))]),
        ],
        "persegi" => vec![
            result("KELILING", vec![number(v("sisi") * v("sisi"))]),
            result("LUAS", vec![number(4.0 * v("sisi"))]),
        ],
        "persamaan-kuadrat" => vec![
            result(
                "DISKRIMINAN",
                vec![
                    "Format: (b² -4 a c)".to_owned(),
                    number(
                        -v("b_discriminant") * v("b_discriminant")
                            - 4.0 * v("a_discriminant") * v("c_discriminant"),
                    ),
                ],
            ),
            result(
                "SUMBU SIMETRI",
                vec![
                    "Format: -b/2a".to_owned(),
                    number(if v("a_axis") == 0.0 {
                        0.0
                    } else {
                        -v("b_axis") / (2.0 * v("a_axis"))
                    }),
                ],
            ),
            result(
                "NILAI OPTIMUM",
                vec![
                    "Format: -D/4a [-(b² - 4ac/4a)]".to_owned(),
                    number(if v("a_optimum") == 0.0 {
                        0.0
                    } else {
                        -(v("b_optimum") * v("b_optimum") - 4.0 * v("a_optimum") * v("c_optimum"))
                            / (4.0 * v("a_optimum"))
                    }),
                ],
            ),
        ],
        "kubus" => vec![
            result("KELILING", vec![number(12.0 * v("sisi"))]),
            result("LUAS", vec![number(6.0 * v("sisi"))]),
            result("VOLUME", vec![number(v("sisi").powi(3))]),
        ],
        "bola" => {
            let r = v("radius");
            vec![
                result(
                    "KELILING BOLA",
                    vec![format!(
                        "{} & {}",
                        number(2.0 * 22.0 / 7.0 * r),
                        number(2.0 * 3.14 * r)
                    )],
                ),
                result(
                    "LUAS BOLA",
                    vec![format!(
                        "{} & {}",
                        number(4.0 * 22.0 / 7.0 * r * r),
                        number(4.0 * 3.14 * r * r)
                    )],
                ),
                result(
                    "VOLUME BOLA",
                    vec![format!(
                        "{} & {}",
                        number(4.0 / 3.0 * 22.0 / 7.0 * r.powi(3)),
                        number(4.0 / 3.0 * std::f64::consts::PI * r.powi(3))
                    )],
                ),
            ]
        }
        "balok" => {
            let area =
                v("panjang") * v("lebar") + v("panjang") * v("tinggi") + v("lebar") * v("tinggi");
            vec![
                result(
                    "KELILING",
                    vec![number(4.0 * (v("panjang") + v("lebar") + v("tinggi")))],
                ),
                result("LUAS BALOK", vec![number(area)]),
                result(
                    "VOLUME BALOK",
                    vec![number(v("panjang") * area * v("tinggi"))],
                ),
            ]
        }
        _ => Vec::new(),
    }
}

pub struct KakuruApp {
    selected: Option<&'static str>,
    inputs: HashMap<String, String>,
    results: Option<Vec<SectionResult>>,
    validation_error: Option<String>,
    feedback_name: String,
    feedback_email: String,
    feedback_message: String,
    feedback_status: Option<String>,
}

impl KakuruApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut visuals = egui::Visuals::light();
        visuals.panel_fill = Color32::from_rgb(245, 247, 248);
        visuals.window_fill = Color32::WHITE;
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(235, 240, 243);
        visuals.selection.bg_fill = GREEN;
        cc.egui_ctx.set_visuals(visuals);
        Self {
            selected: None,
            inputs: HashMap::new(),
            results: None,
            validation_error: None,
            feedback_name: String::new(),
            feedback_email: String::new(),
            feedback_message: String::new(),
            feedback_status: None,
        }
    }

    fn select(&mut self, slug: Option<&'static str>) {
        if self.selected != slug {
            self.selected = slug;
            self.inputs.clear();
            self.results = None;
            self.validation_error = None;
        }
    }

    fn navigation(&mut self, ui: &mut egui::Ui, compact: bool) {
        let current = self
            .selected
            .and_then(|slug| CALCULATORS.iter().find(|item| item.slug == slug))
            .map(|item| item.label)
            .unwrap_or("BERANDA");
        if compact {
            egui::ComboBox::from_id_salt("navigation")
                .selected_text(current)
                .width(ui.available_width())
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_label(self.selected.is_none(), "BERANDA")
                        .clicked()
                    {
                        self.select(None);
                    }
                    for item in CALCULATORS {
                        if ui
                            .selectable_label(self.selected == Some(item.slug), item.label)
                            .clicked()
                        {
                            self.select(Some(item.slug));
                        }
                    }
                });
            return;
        }

        ui.label(RichText::new("KAKURU").strong().color(NAVY).size(20.0));
        ui.add_space(12.0);
        if ui
            .selectable_label(self.selected.is_none(), "⌂  BERANDA")
            .clicked()
        {
            self.select(None);
        }
        ui.add_space(8.0);
        for item in CALCULATORS {
            if ui
                .selectable_label(self.selected == Some(item.slug), item.label)
                .clicked()
            {
                self.select(Some(item.slug));
            }
        }
    }

    fn home(&mut self, ui: &mut egui::Ui) {
        ui.heading(
            RichText::new("カクル / KAKURU")
                .color(NAVY)
                .strong()
                .size(30.0),
        );
        ui.label("Kalkulator rumus untuk membantu menyelesaikan soal matematika.");
        ui.add_space(16.0);
        self.info_card(ui, "Apa itu KAKURU?", "KAKURU adalah kumpulan kalkulator yang dirancang untuk mempermudah perhitungan menggunakan rumus matematika.");
        self.info_card(ui, "Cara menggunakan", "Pilih rumus dari menu, masukkan nilai yang diperlukan, lalu tekan Hitung untuk melihat hasilnya.");
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.heading("Pilih kalkulator");
            ui.add_space(6.0);
            egui::Grid::new("calculator_home_grid")
                .num_columns(2)
                .spacing([8.0, 8.0])
                .show(ui, |ui| {
                    for (index, item) in CALCULATORS.iter().enumerate() {
                        if ui.button(item.label).clicked() {
                            self.select(Some(item.slug));
                        }
                        if index % 2 == 1 {
                            ui.end_row();
                        }
                    }
                });
        });
        ui.add_space(12.0);
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.heading("Saran dan laporan masalah");
            ui.label("Kirim melalui aplikasi email di perangkat kamu.");
            ui.add_space(8.0);
            egui::Grid::new("feedback_form")
                .num_columns(2)
                .show(ui, |ui| {
                    ui.label("Nama Kamu");
                    ui.text_edit_singleline(&mut self.feedback_name);
                    ui.end_row();
                    ui.label("Email Kamu");
                    ui.text_edit_singleline(&mut self.feedback_email);
                    ui.end_row();
                    ui.label("Pesan");
                    ui.add(egui::TextEdit::multiline(&mut self.feedback_message).desired_rows(3));
                    ui.end_row();
                });
            if ui
                .add(
                    egui::Button::new(RichText::new("✉  Kirim saran").color(Color32::WHITE))
                        .fill(GREEN),
                )
                .clicked()
            {
                self.send_feedback();
            }
            ui.label("azzamputraraihan@gmail.com  •  raihanmuaret80@gmail.com");
            if let Some(status) = &self.feedback_status {
                ui.label(status);
            }
        });
        self.info_card(ui, "Dibuat oleh", "Reza Setiawan dan Hersan Ramadhan sebagai penyusun; Azzam Putra Raihan dan Muhammad Raihan sebagai programmer.");
    }

    fn info_card(&self, ui: &mut egui::Ui, title: &str, body: &str) {
        egui::Frame::group(ui.style()).show(ui, |ui| {
            ui.heading(title);
            ui.label(body);
        });
        ui.add_space(8.0);
    }

    fn calculator(&mut self, ui: &mut egui::Ui, slug: &'static str) {
        let specs = calculator_sections(slug);
        for spec in &specs {
            egui::Frame::group(ui.style()).show(ui, |ui| {
                if !spec.heading.is_empty() {
                    ui.heading(spec.heading);
                    ui.add_space(6.0);
                }
                egui::Grid::new(("fields", spec.heading))
                    .num_columns(2)
                    .spacing([12.0, 8.0])
                    .show(ui, |ui| {
                        for field in &spec.fields {
                            ui.label(field.label);
                            let text = self
                                .inputs
                                .entry(field.name.to_owned())
                                .or_insert_with(|| "0".to_owned());
                            ui.add(egui::TextEdit::singleline(text).desired_width(180.0));
                            ui.end_row();
                        }
                    });
                if !spec.note.is_empty() {
                    ui.add_space(6.0);
                    ui.label(spec.note);
                }
                if let Some(results) = &self.results {
                    if let Some(section_result) =
                        results.iter().find(|item| item.heading == spec.heading)
                    {
                        for output in &section_result.outputs {
                            ui.label(RichText::new(output).strong());
                        }
                    }
                }
            });
            ui.add_space(10.0);
        }
        if ui
            .add(egui::Button::new(RichText::new("▦  Hitung").color(Color32::WHITE)).fill(GREEN))
            .clicked()
        {
            let mut parsed = HashMap::new();
            for spec in &specs {
                for field in &spec.fields {
                    let raw = self
                        .inputs
                        .get(field.name)
                        .map(String::as_str)
                        .unwrap_or("0");
                    match raw.trim().parse::<f64>() {
                        Ok(value) => {
                            parsed.insert(field.name.to_owned(), value);
                        }
                        Err(_) => {
                            self.validation_error =
                                Some(format!("{}: Masukkan angka yang valid", field.label));
                            self.results = None;
                            return;
                        }
                    }
                }
            }
            self.validation_error = None;
            self.results = Some(calculate(slug, &parsed));
        }
        if let Some(error) = &self.validation_error {
            ui.colored_label(Color32::from_rgb(180, 35, 40), error);
        }
    }

    fn send_feedback(&mut self) {
        if self.feedback_name.trim().is_empty() || !self.feedback_email.contains('@') {
            self.feedback_status = Some("Masukkan nama dan email yang valid.".to_owned());
            return;
        }
        let body = format!(
            "Nama: {}\nEmail: {}\n\n{}",
            self.feedback_name.trim(),
            self.feedback_email.trim(),
            self.feedback_message.trim()
        );
        let uri = format!(
            "mailto:azzamputraraihan@gmail.com?subject={}&body={}",
            urlencoding::encode("Masukan untuk KAKURU"),
            urlencoding::encode(&body)
        );
        self.feedback_status = Some(match webbrowser::open(&uri) {
            Ok(()) => "Aplikasi email dibuka.".to_owned(),
            Err(error) => format!("Tidak dapat membuka aplikasi email: {error}"),
        });
    }
}

impl eframe::App for KakuruApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let compact = ctx.screen_rect().width() < 680.0;
        egui::TopBottomPanel::top("app_header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("KAKURU").color(NAVY).strong().size(20.0));
                if compact {
                    ui.separator();
                    self.navigation(ui, true);
                } else {
                    let title = self
                        .selected
                        .and_then(|slug| CALCULATORS.iter().find(|item| item.slug == slug))
                        .map(|item| item.label)
                        .unwrap_or("Kalkulator rumus matematika");
                    ui.label(title);
                }
            });
        });

        if !compact {
            egui::SidePanel::left("navigation")
                .resizable(false)
                .default_width(260.0)
                .width_range(220.0..=300.0)
                .show(ctx, |ui| self.navigation(ui, false));
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.set_max_width(900.0);
                    ui.add_space(8.0);
                    if let Some(slug) = self.selected {
                        self.calculator(ui, slug);
                    } else {
                        self.home(ui);
                    }
                });
        });
    }
}

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );
    let options = eframe::NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };
    eframe::run_native(
        "KAKURU",
        options,
        Box::new(|cc| Ok(Box::new(KakuruApp::new(cc)))),
    )
    .expect("failed to start KAKURU");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_celsius_to_fahrenheit_and_kelvin() {
        let values = HashMap::from([("celsius".to_owned(), 0.0)]);
        let result = calculate("suhu", &values);
        assert_eq!(result[0].outputs, ["Fahrenheit: 32", "Kelvin: 273.15"]);
    }

    #[test]
    fn computes_trapezium_perimeter_and_area() {
        let values = HashMap::from([
            ("a".to_owned(), 2.0),
            ("b".to_owned(), 3.0),
            ("c".to_owned(), 4.0),
            ("d".to_owned(), 5.0),
            ("alas_a".to_owned(), 6.0),
            ("alas_b".to_owned(), 8.0),
            ("tinggi".to_owned(), 3.0),
        ]);
        let result = calculate("trapesium", &values);
        assert_eq!(result[0].outputs, ["14"]);
        assert_eq!(result[1].outputs, ["21"]);
    }

    #[test]
    fn keeps_the_spldv_placeholder_without_calculated_results() {
        assert_eq!(calculator_sections("spldv").len(), 3);
        assert!(calculate("spldv", &HashMap::new()).is_empty());
    }

    #[test]
    fn keeps_negative_zero_output() {
        assert_eq!(number(-0.0), "-0");
    }
}
