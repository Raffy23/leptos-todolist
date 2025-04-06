use crate::{components::{NewNoteDialog, NotesOverview}, model::Note};
use leptos::prelude::*;
use leptos_meta::Style;
use thaw::*;

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

    let open = RwSignal::new(false);

    view! {
        <NotesOverview notes=notes/>
        <NewNoteDialog notes=notes open=open />

        <Style>
            "
            .home-page__notes-button {
                position: fixed;
                bottom: 32px;
                right: 32px;
                border-radius: 50%;
                height: 96px;
                width: 96px;
                min-height: unset;
                min-width: unset;
            }
            "
        </Style>

        <Button
            class="home-page__notes-button"
            appearance=ButtonAppearance::Primary
            on_click=move |_| open.set(true)>
            "New Note"
        </Button>
    }
}
