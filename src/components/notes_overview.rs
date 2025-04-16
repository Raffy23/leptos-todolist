use leptos::prelude::*;
use leptos_meta::Style;
use thaw::*;

use crate::model::{Note, NoteDto};
use crate::components::note_card::NoteCard;

#[component]
pub(crate) fn NotesOverview(notes_resource: Resource<Vec<NoteDto>>) -> impl IntoView {
    let (note_deleted, set_note_deleted) = signal(None);

    let (notes, set_notes) = signal(Vec::new());
    Effect::new(move || {
        set_notes.set(
            notes_resource
            .read()
            .as_ref()
            .unwrap_or(&Vec::new())
            .into_iter()
            .map(Note::from_dto)
            .collect::<Vec<_>>()
        );
    });

    Effect::new(move || {
        if let Some(note_id) = note_deleted.get() {
            notes_resource.update(move |notes| {
                if let Some(notes) = notes.as_mut() {
                    notes.retain(|note| note.id != note_id);
                }
            });
        }
    });

    view! {
        <Style>
            "
            .notes-overview__container {
                flex-wrap:wrap;
                margin-top: 20px;
                margin-bottom: 20px;
            }
            "
        </Style>

        <Transition
            fallback=move || view! { <p>"Loading Notes ..."</p> }>
            <Flex class="notes-overview__container" justify=FlexJustify::Start>
                <For
                    each=move || notes.get()
                    key=|note| note.id
                    let(note)
                >
                    <NoteCard note=note note_deleted=set_note_deleted />
                </For>
            </Flex>
        </Transition>
    }
}
