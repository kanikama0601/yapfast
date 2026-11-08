//! The sticker picker's grid: every server's stickers under its name, the
//! open server's first, in rows of as many as the picker's width holds, or
//! those a search finds by name or related emoji. Its rows go when the picker
//! closes or shows another tab; their pictures come through `PictureStore`
//! as they show.

use yapfast_core::chat::StickerSet;

use super::*;

/// The rows that `sets` show for `query`, `columns` stickers to a row, each
/// server's under its name, and the first sticker shown that this account
/// may send, which Enter sends.
fn rows(sets: &[StickerSet], query: &str, columns: usize) -> (Vec<StickerRow>, String) {
    let query = query.trim().to_lowercase();
    let mut rows = Vec::new();
    let mut first = String::new();
    for set in sets {
        let found: Vec<_> = set
            .stickers
            .iter()
            .filter(|sticker| {
                query.is_empty()
                    || sticker.name.to_lowercase().contains(&query)
                    || sticker.tags.to_lowercase().contains(&query)
            })
            .collect();
        if found.is_empty() {
            continue;
        }
        if first.is_empty()
            && let Some(sticker) = found.iter().find(|sticker| sticker.usable)
        {
            first.clone_from(&sticker.id);
        }
        rows.push(StickerRow {
            heading: set.server.to_uppercase().into(),
            cells: ModelRc::default(),
        });
        for chunk in found.chunks(columns.max(1)) {
            let cells: Vec<StickerCell> = chunk
                .iter()
                .map(|sticker| StickerCell {
                    id: sticker.id.as_str().into(),
                    name: sticker.name.as_str().into(),
                    picture_url: sticker.picture.as_str().into(),
                    usable: sticker.usable,
                })
                .collect();
            rows.push(StickerRow {
                heading: SharedString::new(),
                cells: ModelRc::new(VecModel::from(cells)),
            });
        }
    }
    (rows, first)
}

/// Gives the picker the rows `query` finds, `columns` stickers to a row.
fn show(window: &YapfastWindow, app: &Application, query: &str, columns: i32) {
    let sets = app.sticker_sets();
    let (rows, first) = rows(&sets, query, usize::try_from(columns).unwrap_or(1));
    window.set_sticker_rows(ModelRc::new(VecModel::from(rows)));
    window.set_sticker_first_usable(first.into());
}

impl Adapter {
    /// Builds the picker's rows again when Discord's own packs come while it
    /// shows the stickers.
    pub(super) fn show_stickers(&mut self, window: &YapfastWindow) {
        let packs = &self.app.state.sticker_packs;
        if Arc::ptr_eq(packs, &self.sticker_packs) {
            return;
        }
        self.sticker_packs = packs.clone();
        if window.get_stickers_shown() {
            let query = window.get_sticker_query();
            show(window, &self.app, &query, window.get_sticker_columns());
        }
    }
}

pub(super) fn bind(window: &YapfastWindow, adapter: &Rc<RefCell<Adapter>>) {
    let state = Rc::downgrade(adapter);
    let weak = window.as_weak();
    window.on_sticker_search(move |query, columns| {
        if let (Some(adapter), Some(window)) = (state.upgrade(), weak.upgrade()) {
            show(&window, &adapter.borrow().app, &query, columns);
        }
    });
    let state = adapter.clone();
    let weak = window.as_weak();
    window.on_load_sticker_packs(move || {
        if let Some(window) = weak.upgrade() {
            state.borrow_mut().action(&window, Action::LoadStickerPacks);
        }
    });
    let weak = window.as_weak();
    window.on_close_stickers(move || {
        if let Some(window) = weak.upgrade() {
            window.set_sticker_rows(ModelRc::default());
            window.set_sticker_first_usable(SharedString::new());
        }
    });
    let state = adapter.clone();
    let weak = window.as_weak();
    window.on_sticker_pick(move |id| {
        if let Some(window) = weak.upgrade() {
            state
                .borrow_mut()
                .action(&window, Action::PickSticker(id.into()));
        }
    });
}

#[cfg(test)]
mod tests {
    use yapfast_core::chat::PickerSticker;

    use super::*;

    fn sticker(id: &str, name: &str, tags: &str, usable: bool) -> PickerSticker {
        PickerSticker {
            id: id.into(),
            name: name.into(),
            tags: tags.into(),
            picture: format!("https://cdn/stickers/{id}.png"),
            usable,
        }
    }

    #[test]
    fn rows_hold_a_servers_stickers_under_its_name_and_a_search_finds_tags() {
        let sets = [
            StickerSet {
                id: "1".into(),
                server: "Here".into(),
                icon: None,
                stickers: vec![
                    sticker("11", "Wave", "👋", true),
                    sticker("12", "Bounce", "ball", true),
                    sticker("13", "Nod", "yes", true),
                ],
            },
            StickerSet {
                id: "2".into(),
                server: "Elsewhere".into(),
                icon: None,
                stickers: vec![sticker("21", "Ball", "", false)],
            },
        ];
        let shape = |rows: &[StickerRow]| {
            rows.iter()
                .map(|row| {
                    if row.heading.is_empty() {
                        row.cells
                            .iter()
                            .map(|cell| cell.id.to_string())
                            .collect::<Vec<_>>()
                            .join(" ")
                    } else {
                        format!("# {}", row.heading)
                    }
                })
                .collect::<Vec<_>>()
        };
        let (all, first) = rows(&sets, "", 2);
        assert_eq!(shape(&all), ["# HERE", "11 12", "13", "# ELSEWHERE", "21"]);
        assert_eq!(first, "11");
        // "ball" is one's tag and the other's name; the first that may be
        // sent is the one Enter sends.
        let (found, first) = rows(&sets, " BALL ", 4);
        assert_eq!(shape(&found), ["# HERE", "12", "# ELSEWHERE", "21"]);
        assert_eq!(first, "12");
        let (found, first) = rows(&sets[1..], "ball", 4);
        assert_eq!(shape(&found), ["# ELSEWHERE", "21"]);
        assert_eq!(first, "", "none of them may be sent");
        assert!(rows(&sets, "nothing", 4).0.is_empty());
    }
}
