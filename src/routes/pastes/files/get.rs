use database::DbConn;
use models::paste::PasteId;
use models::paste::output::OutputFile;
use models::status::{Status, ErrorKind};
use routes::{RouteResult, OptionalUser};

use rocket::http::Status as HttpStatus;

#[get("/<paste_id>/files")]
fn get_files(paste_id: PasteId, user: OptionalUser, conn: DbConn) -> RouteResult<Vec<OutputFile>> {
  let paste = match paste_id.get(&conn)? {
    Some(paste) => paste,
    None => return Ok(Status::show_error(HttpStatus::NotFound, ErrorKind::MissingPaste)),
  };

  if let Some((status, kind)) = paste.check_access(user.as_ref().map(|x| x.id())) {
    return Ok(Status::show_error(status, kind));
  }

  let files: Vec<OutputFile> = paste_id.files(&conn)?
    .into_iter()
    .map(|f| OutputFile::new(&f.id(), Some(f.name().clone()), None))
    .collect();

  Ok(Status::show_success(HttpStatus::Ok, files))
}
