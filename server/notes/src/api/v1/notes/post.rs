use kernel::{
    api,
    log::macros::*,
    api::middlewares::{
        GetRequestLogger,
        GetRequestId,
        GetRequestAuth,
    },
    KernelError,
};
use futures::{
    future::Future,
    future::ok,
    future::Either,
};
use actix_web::{
    web, Error, HttpRequest, HttpResponse, ResponseError,
};
use crate::{
    controllers,
    api::v1::models,
};


pub fn post(note_data: web::Json<models::CreateNoteBody>, state: web::Data<api::State>, req: HttpRequest)
-> impl Future<Item = HttpResponse, Error = Error> {
    let logger = req.logger();
    let auth = req.request_auth();
    let request_id = req.request_id().0;

    if auth.session.is_none() || auth.account.is_none() {
        return Either::A(ok(KernelError::Unauthorized("Authentication required".to_string()).error_response()));
    }

    return Either::B(
        state.db.send(controllers::CreateNote{
            title: note_data.title.clone(),
            body: note_data.body.clone(),
            account_id: auth.account.expect("error unwraping non none account").id,
            session_id: auth.session.expect("error unwraping non none session").id,
            request_id,
        })
        .map_err(|_| KernelError::ActixMailbox)
        .from_err()
        .and_then(move |note| {
            match note {
                Ok(note) => {
                    let res = models::NoteResponse{
                        id: note.id,
                        created_at: note.created_at,
                        updated_at: note.updated_at,
                        archived_at: note.archived_at,
                        removed_at: note.removed_at,
                        title: note.title,
                        body: note.body,
                    };
                    let res = api::Response::data(res);
                    ok(HttpResponse::Ok().json(&res))
                },
                Err(err) => {
                    slog_error!(logger, "{}", err);
                    ok(err.error_response())
                },
            }
        })
    );
}
