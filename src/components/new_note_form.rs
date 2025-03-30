use leptos::{ev, logging::log, prelude::*};
use thaw::*;

use crate::model::Note;

#[server(NewNote)]
#[tracing::instrument(name = "NewNote", skip_all)]
pub(crate) async fn new_note(title: String, content: String) -> Result<Note, ServerFnError> {
    use crate::auth::AuthSession;
    use crate::repository::NoteRepository;
    use axum::http::StatusCode;
    use leptos_axum::extract;

    let response_options = expect_context::<leptos_axum::ResponseOptions>();
    let notes = expect_context::<NoteRepository>();

    let session: AuthSession = extract().await?;
    match session.user {
        Some(user) => {
            let uuid = notes.create(user.id, &title, &content).await?;

            Ok(Note {
                id: uuid,
                title: title.clone(),
                content: content.clone(),
            })
        }
        None => {
            response_options.set_status(StatusCode::UNAUTHORIZED);
            Err(ServerFnError::new("Unauthorized"))
        }
    }
}

#[component]
pub(crate) fn NewNoteForm(notes: Resource<Vec<Note>>) -> impl IntoView {
    let new_note_action = ServerAction::<NewNote>::new();
    let (new_note, set_new_note) = signal(false);

    Effect::new(
        move || {
        if let Some(Ok(note)) = new_note_action.value().get() {
            set_new_note.set(false);

            match notes.get_untracked() {
                Some(_) => {
                    notes.update(move |notes| {
                        notes.as_mut().unwrap().push(note);
                    });
                }
                None => {
                    notes.set(Some(vec![note]));
                }
            }
        }
    });

    view! {
        <div>
            <Show when=move || { !new_note.get() }>
                <Flex>
                    <Input
                        placeholder="Add Note"
                        input_type=InputType::Text
                        on_focus=move |_| {
                            set_new_note.set(true);
                        }
                        name="note"/>
                </Flex>
            </Show>

            <Show when=move || { new_note.get() }>
                <ActionForm action=new_note_action>
                    <Card>
                        <CardHeader>
                            "New Note"
                        </CardHeader>

                        <FieldContextProvider>
                            <Space vertical=true>
                                <Field label="Title" name="title">
                                    <Input input_type=InputType::Text/>
                                </Field>

                                <Field label="Note" name="content">
                                    <Input
                                        placeholder="Write Note"
                                        input_type=InputType::Text
                                        />
                                </Field>

                                <Button
                                    appearance=ButtonAppearance::Subtle
                                    button_type=ButtonType::Submit
                                    on_click={
                                        let field_context = FieldContextInjection::expect_context();
                                        move |e: ev::MouseEvent| {
                                            if !field_context.validate() {
                                                e.prevent_default();
                                            }
                                        }
                                    }
                                >
                                    "Save"
                                </Button>
                            </Space>
                        </FieldContextProvider>
                    </Card>
                </ActionForm>
            </Show>
        </div>
    }
}
