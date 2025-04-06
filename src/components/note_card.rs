use leptos::prelude::*;
use leptos_meta::Style;
use thaw::*;

use crate::model::{Note, NoteId};

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
pub(crate) async fn check_note(note_id: NoteId) -> Result<(), ServerFnError> {
    use axum::http::StatusCode;

    let response_options = expect_context::<leptos_axum::ResponseOptions>();

    response_options.set_status(StatusCode::INTERNAL_SERVER_ERROR);
    Err(ServerFnError::new("Not Implemented"))
}

#[component]
pub(crate) fn NoteCard(note: Note) -> impl IntoView {
    let delete_action = ServerAction::<DeleteNote>::new();
    let check_action = ServerAction::<CheckNote>::new();

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
            "
        </Style>

        <Card class="notes-card__container">
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
                            check_action.dispatch(CheckNote { note_id: note.id });
                        }>
                    </Button>
                </Tooltip>
            </CardFooter>
        </Card>
    }
}
