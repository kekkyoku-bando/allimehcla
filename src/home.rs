use std::collections::HashMap;

use dioxus::prelude::*;

use crate::home_api::{self, Image};

#[component]
pub fn Home() -> Element {
    let images = use_loader(home_api::get_images)?;
    let grouped = use_memo(move || {
        let images = images();
        let mut map: HashMap<String, Vec<Image>> = HashMap::new();
        for image in images {
            map.entry(image.time.year()).or_default().push(image);
        }
        map
    });

    rsx! {
        Content { images: grouped }
    }
}

#[component]
fn Content(images: ReadSignal<HashMap<String, Vec<Image>>>) -> Element {
    let sorted_images = use_memo(move || {
        let map = images();
        let mut all_images: Vec<Image> = map.values().flat_map(|v| v.iter().cloned()).collect();
        all_images.sort_by_key(|image| image.time);
        all_images
    });

    let columns = use_memo(move || {
        let sorted = sorted_images();
        let num_cols = 4;
        let mut cols: Vec<Vec<Image>> = vec![vec![]; num_cols];
        for (i, img) in sorted.iter().enumerate() {
            cols[i % num_cols].push(img.clone());
        }
        cols
    });

    rsx! {
        div { class: "grid grid-cols-4 gap-4",
            for column in columns() {
                div { class: "grid gap-4",
                    for image in column {
                        div {
                            img {
                                class: "h-auto max-w-full rounded-base",
                                src: format!("data:image/png;base64, {}", image.base64),
                            }
                        }
                    }
                }
            }
        }
    }
}
