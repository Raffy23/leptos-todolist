use crate::{components::{NewNoteForm, NotesOverview}, model::Note};
use leptos::prelude::*;

#[server(GetNotes)]
#[tracing::instrument(name = "GetNotes", skip_all)]
pub(crate) async fn get_notes() -> Result<Vec<Note>, ServerFnError> {
    use crate::auth::AuthSession;
    use crate::repository::NoteRepository;
    use axum::http::StatusCode;
    use leptos_axum::extract;

    let response_options = expect_context::<leptos_axum::ResponseOptions>();
    let session: AuthSession = extract().await?;

    match session.user {
        Some(user) => {
            let notes_repository = expect_context::<NoteRepository>();
            let notes = notes_repository.find_by_owner(user.id).await?;

            Ok(notes.into_iter().map(|note| note.to_note()).collect())
        }
        None => {
            response_options.set_status(StatusCode::UNAUTHORIZED);
            Err(ServerFnError::new("Unauthorized"))
        }
    }
}

#[component]
pub(crate) fn HomePage() -> impl IntoView {
    let notes = Resource::new_blocking(
        || (),
        |_| async move { get_notes().await.unwrap_or_default() },
    );

    view! {
        <NewNoteForm notes=notes/>
        <NotesOverview notes=notes/>
    }
}
