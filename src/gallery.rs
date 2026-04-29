use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::LazyLock;

use dioxus::prelude::*;

static IMAGES: LazyLock<Vec<Image>> = LazyLock::new(|| {
    vec![
        Image {
            asset: asset!("../assets/images/2024_01_05.png"),
            time: ImageTime::Specific {
                day: 5,
                month: 1,
                year: 2024,
            },
        },
        Image {
            asset: asset!("../assets/images/2024_03_17.png"),
            time: ImageTime::Specific {
                day: 17,
                month: 3,
                year: 2024,
            },
        },
        Image {
            asset: asset!("../assets/images/2024_04_15.png"),
            time: ImageTime::Specific {
                day: 15,
                month: 4,
                year: 2024,
            },
        },
        Image {
            asset: asset!("../assets/images/2024_10_12.png"),
            time: ImageTime::Specific {
                day: 12,
                month: 10,
                year: 2024,
            },
        },
        Image {
            asset: asset!("../assets/images/2025_07_24.png"),
            time: ImageTime::Specific {
                day: 24,
                month: 7,
                year: 2025,
            },
        },
        Image {
            asset: asset!("../assets/images/2025_11_11.png"),
            time: ImageTime::Specific {
                day: 11,
                month: 11,
                year: 2025,
            },
        },
        Image {
            asset: asset!("../assets/images/2026_03_21.png"),
            time: ImageTime::Specific {
                day: 21,
                month: 3,
                year: 2026,
            },
        },
        Image {
            asset: asset!("../assets/images/2026_04_01.png"),
            time: ImageTime::Specific {
                day: 1,
                month: 4,
                year: 2026,
            },
        },
        Image {
            asset: asset!("../assets/images/2026_04_18.png"),
            time: ImageTime::Specific {
                day: 18,
                month: 4,
                year: 2026,
            },
        },
    ]
});

#[derive(PartialEq, Clone)]
struct Image {
    pub asset: Asset,
    pub time: ImageTime,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum ImageTime {
    #[allow(dead_code)]
    Unknown,
    Specific {
        day: u32,
        month: u32,
        year: u32,
    },
}

impl ImageTime {
    pub fn year(&self) -> Option<u32> {
        match self {
            ImageTime::Unknown => None,
            ImageTime::Specific { year, .. } => Some(*year),
        }
    }
}

impl PartialOrd for ImageTime {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ImageTime {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (ImageTime::Unknown, ImageTime::Unknown) => Ordering::Equal,
            (ImageTime::Unknown, ImageTime::Specific { .. }) => Ordering::Less,
            (ImageTime::Specific { .. }, ImageTime::Unknown) => Ordering::Greater,
            (
                ImageTime::Specific {
                    year: y1,
                    month: m1,
                    day: d1,
                },
                ImageTime::Specific {
                    year: y2,
                    month: m2,
                    day: d2,
                },
            ) => y1.cmp(y2).then(m1.cmp(m2)).then(d1.cmp(d2)),
        }
    }
}

#[component]
pub fn Gallery() -> Element {
    let grouped = use_memo(move || {
        let mut map: HashMap<Option<u32>, Vec<Image>> = HashMap::new();
        for image in IMAGES.clone() {
            map.entry(image.time.year()).or_default().push(image);
        }
        map
    });

    rsx! {
        Content { images: grouped }
    }
}

#[component]
fn Content(images: ReadSignal<HashMap<Option<u32>, Vec<Image>>>) -> Element {
    const COLUMNS: usize = 4;

    let sorted: Memo<Vec<(String, Vec<Vec<Image>>)>> = use_memo(move || {
        let grouped_by_years = images();
        let mut sorted_columns: HashMap<Option<u32>, Vec<Vec<Image>>> = HashMap::new();

        for (year, mut images) in grouped_by_years {
            images.sort_by_key(|image| image.time);

            let cols = sorted_columns
                .entry(year)
                .or_insert_with(|| vec![vec![]; COLUMNS]);
            for (i, image) in images.into_iter().enumerate() {
                cols[i % COLUMNS].push(image);
            }
        }

        let mut sorted_years: Vec<(Option<u32>, Vec<Vec<Image>>)> =
            sorted_columns.into_iter().collect();
        sorted_years.sort_by(|(first, _), (second, _)| {
            second.unwrap_or(u32::MIN).cmp(&first.unwrap_or(u32::MIN))
        });
        sorted_years
            .into_iter()
            .map(|(year, columns)| {
                (
                    year.as_ref()
                        .map(u32::to_string)
                        .unwrap_or("Unknown".to_string()),
                    columns,
                )
            })
            .collect()
    });

    rsx! {
        for (year, columns) in sorted() {
            div { class: "flex flex-col mb-2",
                p { class: "text-4xl mb-4", {year} }
                div { class: "grid grid-cols-4 gap-4",
                    for column in columns {
                        div { class: "grid gap-4",
                            for image in column {
                                div {
                                    img {
                                        class: "h-auto max-w-full",
                                        src: image.asset,
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
