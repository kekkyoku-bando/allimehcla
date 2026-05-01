use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;
use std::sync::LazyLock;

use dioxus::prelude::*;

static IMAGES: LazyLock<Vec<Image>> = LazyLock::new(|| {
    vec![
        Image {
            asset: asset!("/assets/images/2023_04_15.png"),
            time: ImageTime::Specific {
                day: 15,
                month: 4,
                year: 2023,
            },
        },
        Image {
            asset: asset!("/assets/images/2023_05_27.png"),
            time: ImageTime::Specific {
                day: 27,
                month: 5,
                year: 2023,
            },
        },
        Image {
            asset: asset!("/assets/images/2023_05_30.png"),
            time: ImageTime::Specific {
                day: 30,
                month: 5,
                year: 2023,
            },
        },
        Image {
            asset: asset!("/assets/images/2023_07_21.png"),
            time: ImageTime::Specific {
                day: 21,
                month: 7,
                year: 2023,
            },
        },
        Image {
            asset: asset!("/assets/images/2023_07_30.png"),
            time: ImageTime::Specific {
                day: 30,
                month: 7,
                year: 2023,
            },
        },
        Image {
            asset: asset!("/assets/images/2023_08_23.png"),
            time: ImageTime::Specific {
                day: 23,
                month: 8,
                year: 2023,
            },
        },
        Image {
            asset: asset!("/assets/images/2023_08_30.png"),
            time: ImageTime::Specific {
                day: 30,
                month: 8,
                year: 2023,
            },
        },
        Image {
            asset: asset!("/assets/images/2023_11_11.png"),
            time: ImageTime::Specific {
                day: 11,
                month: 11,
                year: 2023,
            },
        },
        Image {
            asset: asset!("/assets/images/2023_12_09.png"),
            time: ImageTime::Specific {
                day: 9,
                month: 12,
                year: 2023,
            },
        },
        Image {
            asset: asset!("/assets/images/2023_12_10.png"),
            time: ImageTime::Specific {
                day: 10,
                month: 12,
                year: 2023,
            },
        },
        Image {
            asset: asset!("/assets/images/2024_01_05.png"),
            time: ImageTime::Specific {
                day: 5,
                month: 1,
                year: 2024,
            },
        },
        Image {
            asset: asset!("/assets/images/2024_01_30.png"),
            time: ImageTime::Specific {
                day: 30,
                month: 1,
                year: 2024,
            },
        },
        Image {
            asset: asset!("/assets/images/2024_02_02_2.png"),
            time: ImageTime::Specific {
                day: 2,
                month: 2,
                year: 2024,
            },
        },
        Image {
            asset: asset!("/assets/images/2024_03_17.png"),
            time: ImageTime::Specific {
                day: 17,
                month: 3,
                year: 2024,
            },
        },
        Image {
            asset: asset!("/assets/images/2024_04_15.png"),
            time: ImageTime::Specific {
                day: 15,
                month: 4,
                year: 2024,
            },
        },
        Image {
            asset: asset!("/assets/images/2024_08_31.png"),
            time: ImageTime::Specific {
                day: 31,
                month: 8,
                year: 2024,
            },
        },
        Image {
            asset: asset!("/assets/images/2024_10_12.png"),
            time: ImageTime::Specific {
                day: 12,
                month: 10,
                year: 2024,
            },
        },
        Image {
            asset: asset!("/assets/images/2025_05_12.png"),
            time: ImageTime::Specific {
                day: 12,
                month: 5,
                year: 2025,
            },
        },
        Image {
            asset: asset!("/assets/images/2025_05_21.png"),
            time: ImageTime::Specific {
                day: 21,
                month: 5,
                year: 2025,
            },
        },
        Image {
            asset: asset!("/assets/images/2025_07_24.png"),
            time: ImageTime::Specific {
                day: 24,
                month: 7,
                year: 2025,
            },
        },
        Image {
            asset: asset!("/assets/images/2025_08_14.png"),
            time: ImageTime::Specific {
                day: 14,
                month: 8,
                year: 2025,
            },
        },
        Image {
            asset: asset!("/assets/images/2025_09_15.png"),
            time: ImageTime::Specific {
                day: 15,
                month: 9,
                year: 2025,
            },
        },
        Image {
            asset: asset!("/assets/images/2025_09_29.png"),
            time: ImageTime::Specific {
                day: 29,
                month: 9,
                year: 2025,
            },
        },
        Image {
            asset: asset!("/assets/images/2025_09_29_2.png"),
            time: ImageTime::Specific {
                day: 29,
                month: 9,
                year: 2025,
            },
        },
        Image {
            asset: asset!("/assets/images/2025_11_11.png"),
            time: ImageTime::Specific {
                day: 11,
                month: 11,
                year: 2025,
            },
        },
        Image {
            asset: asset!("/assets/images/2025_11_27.png"),
            time: ImageTime::Specific {
                day: 27,
                month: 11,
                year: 2025,
            },
        },
        Image {
            asset: asset!("/assets/images/2025_12_06.png"),
            time: ImageTime::Specific {
                day: 6,
                month: 12,
                year: 2025,
            },
        },
        Image {
            asset: asset!("/assets/images/2026_01_14.png"),
            time: ImageTime::Specific {
                day: 14,
                month: 1,
                year: 2026,
            },
        },
        Image {
            asset: asset!("/assets/images/2026_01_14_czn.png"),
            time: ImageTime::Specific {
                day: 14,
                month: 1,
                year: 2026,
            },
        },
        Image {
            asset: asset!("/assets/images/2026_01_25.png"),
            time: ImageTime::Specific {
                day: 25,
                month: 1,
                year: 2026,
            },
        },
        Image {
            asset: asset!("/assets/images/2026_02_07.png"),
            time: ImageTime::Specific {
                day: 7,
                month: 2,
                year: 2026,
            },
        },
        Image {
            asset: asset!("/assets/images/2026_02_24.png"),
            time: ImageTime::Specific {
                day: 24,
                month: 2,
                year: 2026,
            },
        },
        Image {
            asset: asset!("/assets/images/2026_02_24_2.png"),
            time: ImageTime::Specific {
                day: 24,
                month: 2,
                year: 2026,
            },
        },
        Image {
            asset: asset!("/assets/images/2026_03_21.png"),
            time: ImageTime::Specific {
                day: 21,
                month: 3,
                year: 2026,
            },
        },
        Image {
            asset: asset!("/assets/images/2026_03_26.png"),
            time: ImageTime::Specific {
                day: 26,
                month: 3,
                year: 2026,
            },
        },
        Image {
            asset: asset!("/assets/images/2026_03_26_2.png"),
            time: ImageTime::Specific {
                day: 26,
                month: 3,
                year: 2026,
            },
        },
        Image {
            asset: asset!("/assets/images/2026_04_01.png"),
            time: ImageTime::Specific {
                day: 1,
                month: 4,
                year: 2026,
            },
        },
        Image {
            asset: asset!("/assets/images/2026_04_18.png"),
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
            images.sort_by_key(|image| Reverse(image.time));

            let cols = sorted_columns
                .entry(year)
                .or_insert_with(|| vec![vec![]; COLUMNS]);
            for (i, image) in images.into_iter().enumerate() {
                cols[i % COLUMNS].push(image);
            }
        }

        let mut sorted_years: Vec<(Option<u32>, Vec<Vec<Image>>)> =
            sorted_columns.into_iter().collect();
        sorted_years.sort_by_key(|(year, _)| Reverse(year.unwrap_or(u32::MIN)));
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
    let mut lightbox_image: Signal<Option<Asset>> = use_signal(|| None);

    rsx! {
        div { class: "flex flex-col",
            p { "This is a selected collection of my works throughout each year." }
            p { class: "text-base", "Email: allimehcla@gmail.com" }
            p { class: "text-base", "X: allimehcla" }
            br {}

            for (year, columns) in sorted() {
                div { class: "flex flex-col mb-2",
                    p { class: "text-4xl mb-4", {year} }
                    div { class: "grid grid-cols-4 gap-4",
                        for column in columns {
                            div { class: "grid gap-4",
                                for image in column {
                                    div {
                                        img {
                                            class: "h-auto max-w-full cursor-pointer",
                                            src: image.asset,
                                            onclick: move |_| {
                                                lightbox_image.set(Some(image.asset));
                                            },
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        if let Some(asset) = lightbox_image() {
            div {
                class: "fixed inset-0 z-50 flex items-center justify-center p-2 bg-black/80 cursor-pointer",
                onclick: move |_| lightbox_image.set(None),
                img {
                    class: "max-w-full max-h-[calc(100vh-4rem)] object-contain",
                    src: asset,
                }
            }
        }
    }
}
