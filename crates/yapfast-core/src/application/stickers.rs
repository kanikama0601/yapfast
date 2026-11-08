//! The sticker picker, as Discord's client has it between the GIFs and the
//! emoji: every server's stickers, the open server's first, then Discord's
//! own packs, and a click that sends one as a message of its own, keeping a
//! reply. Without Nitro only the open server's stickers may be sent, and the
//! rest show greyed, as Discord's picker shows them; Discord's own may be
//! sent by anyone anywhere.

use super::*;

impl Application {
    /// Every sticker of every server the account is in, by server, the room's
    /// own server first, each saying whether this account may send it in the
    /// room it is looking at; then Discord's own packs, once they came.
    pub fn sticker_sets(&self) -> Vec<chat::StickerSet> {
        let mut sets = self.server_sticker_sets();
        sets.extend(self.state.sticker_packs.iter().cloned());
        sets
    }

    /// Asks for Discord's own sticker packs as the picker shows its
    /// stickers, the first time only, or again after they failed.
    pub(super) fn load_sticker_packs(&mut self) -> Vec<Effect> {
        if self.sticker_packs_asked || !self.state.can_pick_gifs() {
            return vec![];
        }
        self.sticker_packs_asked = true;
        vec![Effect::Backend(Command::Discord(
            DiscordCommand::StickerPacks,
        ))]
    }

    /// Discord's own packs came, or didn't: the next look asks again.
    pub(super) fn sticker_packs_came(&mut self, packs: Result<Vec<chat::StickerSet>, String>) {
        match packs {
            Ok(packs) => self.state.sticker_packs = Arc::new(packs),
            Err(error) => {
                tracing::debug!(%error, "could not load Discord's sticker packs");
                self.sticker_packs_asked = false;
            }
        }
    }

    fn server_sticker_sets(&self) -> Vec<chat::StickerSet> {
        #[cfg(feature = "discord")]
        if let Some(snapshot) = &self.discord {
            let here = self
                .state
                .selected
                .as_deref()
                .and_then(discord::parse_room)
                .map(|(guild, _)| guild);
            let nitro = snapshot
                .account
                .as_ref()
                .is_some_and(|account| account.premium_type > 0);
            let mut servers: Vec<_> = snapshot.servers.iter().collect();
            servers.sort_by_key(|server| Some(server.id.as_str()) != here);
            return servers
                .into_iter()
                .map(|server| {
                    let ours = Some(server.id.as_str()) == here;
                    chat::StickerSet {
                        id: server.id.clone(),
                        server: server.name.clone(),
                        icon: server.icon.clone(),
                        stickers: server
                            .stickers
                            .iter()
                            .filter(|sticker| sticker.available)
                            .map(|sticker| chat::PickerSticker {
                                id: sticker.id.clone(),
                                name: sticker.name.clone(),
                                tags: sticker.tags.clone(),
                                picture: sticker.picture(),
                                usable: nitro || ours,
                            })
                            .collect(),
                    }
                })
                .filter(|set| !set.stickers.is_empty())
                .collect();
        }
        vec![]
    }

    /// A sticker clicked in the picker, by its id: sends it as a message on
    /// its own, keeping a reply that was being written and the composer's
    /// text. One this account may not send here does nothing.
    pub(super) fn pick_sticker(&mut self, id: String) -> Vec<Effect> {
        let Some(room) = self.state.selected.clone() else {
            return vec![];
        };
        if !self.state.can_pick_gifs() {
            return vec![];
        }
        let usable = self
            .sticker_sets()
            .iter()
            .flat_map(|set| &set.stickers)
            .any(|sticker| sticker.id == id && sticker.usable);
        let Some(sticker) = id.parse::<u64>().ok().filter(|_| usable) else {
            return vec![];
        };
        let (reply, quiet_reply) = match self.state.compose.get(&room) {
            Some(Compose::Reply { item, ping, .. }) => (Some(item.clone()), !ping),
            _ => (None, false),
        };
        let mut effects = self.jump_to_present();
        effects.push(Effect::Backend(Command::Send {
            room,
            body: String::new(),
            reply,
            options: chat::SendOptions {
                quiet_reply,
                silent: false,
                sticker: Some(sticker),
            },
            files: vec![],
        }));
        effects
    }
}
