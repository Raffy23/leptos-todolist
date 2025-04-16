use leptos::{logging::log, prelude::*};
use leptos_meta::Style;
use thaw::*;

use crate::model::{Note, NoteDto, NoteId};

#[server(DeleteNote)]
#[tracing::instrument(name = "DeleteNote", skip_all)]
pub(crate) async fn delete_note(note_id: NoteId) -> Result<(), ServerFnError> {
    use crate::auth::AuthSession;
    use crate::repository::NoteRepository;
    use axum::http::StatusCode;
    use leptos_axum::extract;

    let response_options = expect_context::<leptos_axum::ResponseOptions>();
    let notes = expect_context::<NoteRepository>();

    let session: AuthSession = extract().await?;
    match session.user {
        Some(user) => {
            if notes.delete(user.id, note_id).await? > 0 {
                Ok(())
            } else {
                response_options.set_status(StatusCode::NOT_FOUND);
                Err(ServerFnError::new("Note not found"))
            }
        }
        None => {
            response_options.set_status(StatusCode::UNAUTHORIZED);
            Err(ServerFnError::new("Unauthorized"))
        }
    }
}

#[server(CheckNote)]
#[tracing::instrument(name = "CheckNote", skip_all)]
pub(crate) async fn check_note(note_id: NoteId, toggle: bool) -> Result<(), ServerFnError> {
    use crate::auth::AuthSession;
    use crate::repository::NoteRepository;
    use axum::http::StatusCode;
    use leptos_axum::extract;

    let response_options = expect_context::<leptos_axum::ResponseOptions>();
    let notes = expect_context::<NoteRepository>();

    let session: AuthSession = extract().await?;
    match session.user {
        Some(user) => {
            let changed_rows = notes.update_checked(user.id, note_id, toggle).await?;

            if changed_rows > 0 {
                Ok(())
            } else {
                response_options.set_status(StatusCode::NOT_FOUND);
                Err(ServerFnError::new("Note not found"))
            }
        }
        None => {
            response_options.set_status(StatusCode::UNAUTHORIZED);
            Err(ServerFnError::new("Unauthorized"))
        }
    }
}

#[component]
pub(crate) fn NoteCard(note: Note, note_deleted: WriteSignal<Option<NoteId>>) -> impl IntoView {
    let delete_action = ServerAction::<DeleteNote>::new();
    let check_action = ServerAction::<CheckNote>::new();

    Effect::new(move || {
        if let Some(Ok(_)) = delete_action.value().get() {
            note_deleted.set(Some(note.id));
        }
    });

    Effect::new(move || {
        if let Some(Ok(_)) = check_action.value().get() {
            note.checked.update(|checked| {
                *checked = !*checked;
            });
        }
    });

    view! {
        <Style>
            "
            .notes-card__container {
                width: 238px;
                margin: 0 1% 20px;
                max-height: 238px;
            }

            .notes-card__footer {
                display: flex;
                justify-content: end;
            }

            .notes-card__dimmed {
                background-color: var(--colorNeutralForegroundDisabled);
            }
            "
        </Style>

        <Card class="notes-card__container" class:notes-card__dimmed=note.checked>
            <CardHeader><b>{note.title}</b></CardHeader>
            {note.content}
            <CardFooter class="notes-card__footer">
                <Tooltip content="Delete Note">
                    <Button
                        icon=icondata::FiTrash
                        on_click=move |_| {
                            delete_action.dispatch(DeleteNote { note_id: note.id });
                        }>
                    </Button>
                </Tooltip>
                <Tooltip content="Complete Note">
                    <Button
                        icon=icondata::FiCheck
                        on_click=move |_| {
                            check_action.dispatch(CheckNote { note_id: note.id, toggle: !*note.checked.read() });
                        }>
                    </Button>
                </Tooltip>
            </CardFooter>
        </Card>
    }
}
