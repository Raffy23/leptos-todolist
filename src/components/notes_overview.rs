use leptos::{prelude::*, tachys::reactive_graph::node_ref};
use leptos_meta::Style;
use thaw::*;

use crate::model::Note;

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
            .notes-overview__container > .thaw-card {
                width: 238px;
                margin: 0 1% 20px;
                max-height: 238px;
            }
            "
        </Style>

        <Transition
            fallback=move || view! { <p>"Loading Notes ..."</p> }>
            <Flex class="notes-overview__container" justify=FlexJustify::Start>
                //{
                //    notes.get().map(|notes| {
                //        notes.into_iter().map(|note| {
                //            view! {
                //                <Card>
                //                    <CardHeader>{note.title.clone()}</CardHeader>
                //                    {note.content.clone()}
                //                </Card>
                //            }
                //        }).collect::<Vec<_>>()
                //    })
                //}
                <For
                    each=move || notes.get().unwrap_or_default()
                    key=|note| note.id
                    children=move |note| {
                        view! {
                            <Card>
                                <CardHeader>{note.title}</CardHeader>
                                {note.content}
                            </Card>
                        }
                    }/>
            </Flex>
        </Transition>
    }
}
