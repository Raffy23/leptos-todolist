use leptos::prelude::*;
use leptos_meta::Style;
use thaw::*;

use crate::model::Note;
use crate::components::note_card::NoteCard;

#[component]
pub(crate) fn NotesOverview(notes: Resource<Vec<Note>>) -> impl IntoView {
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
                    each=move || notes.get().unwrap_or_default()
                    key=|note| note.id
                    children=move |note| {
                        view! {
                            <NoteCard note=note />
                        }
                    }/>
            </Flex>
        </Transition>
    }
}
