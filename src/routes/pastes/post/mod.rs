use database::{DbConn, schema};
use database::models::pastes::NewPaste;
use database::models::deletion_keys::NewDeletionKey;
use database::models::files::NewFile;
use models::paste::{Paste, Content};
use models::status::{Status, ErrorKind};
use routes::{RouteResult, OptionalUser};
use store::Store;

use diesel;
use diesel::prelude::*;

use rocket::http::Status as HttpStatus;

use rocket_contrib::Json;

use uuid::Uuid;

use std::fs::File;
use std::io::Write;

mod output;

use self::output::Success;

type InfoResult = ::std::result::Result<Json<Paste>, ::rocket_contrib::SerdeError>;

#[post("/", format = "application/json", data = "<info>")]
fn post(info: InfoResult, user: OptionalUser, conn: DbConn) -> RouteResult<Success> {
  // TODO: can this be a request guard?
  let info = match info {
    Ok(x) => x,
    Err(e) => {
      let message = format!("could not parse json: {}", e);
      return Ok(Status::show_error(HttpStatus::BadRequest, ErrorKind::BadJson(Some(message))));
    },
  };

  // check that files are valid
  // move validate_files to Paste?
  if let Err(e) = Store::validate_files(&info.files) {
    return Ok(Status::show_error(HttpStatus::BadRequest, ErrorKind::InvalidFile(Some(e))));
  }
  // move this to PasteId::create?
  // rocket has already verified the paste info is valid, so create a paste
  let id = Store::new_paste()?;

  let np = NewPaste::new(
    *id,
    info.metadata.name.clone(),
    info.metadata.visibility,
    user.as_ref().map(|x| x.id()),
  );
  diesel::insert_into(schema::pastes::table)
    .values(&np)
    .execute(&*conn)?;

  let deletion_key = if user.is_none() {
    let key = NewDeletionKey::generate(*id);
    diesel::insert_into(schema::deletion_keys::table)
      .values(&key)
      .execute(&*conn)?;
    Some(key.key())
  } else {
    None
  };

  let files = id.files_directory();

  // PasteId::write_files?
  // write the files
  let mut new_files = Vec::with_capacity(info.files.len());
  let mut count = 0;
  for pf in info.into_inner().files {
    let file_id = Uuid::new_v4();
    let pf_path = files.join(file_id.simple().to_string());

    let mut file = File::create(pf_path)?;
    let content = match pf.content {
      Content::Text(c) => c.into_bytes(),
      // all base64/compress/decompress is handled via serde
      Content::Base64(b) | Content::Gzip(b) | Content::Xz(b) => b,
    };
    file.write_all(&content)?;
    let name = match pf.name {
      Some(n) => n,
      None => {
        count += 1;
        format!("pastefile{}", count)
      },
    };
    new_files.push(NewFile::new(file_id, *id, name, None));
  }

  diesel::insert_into(schema::files::table)
    .values(&new_files)
    .execute(&*conn)?;

  // TODO: change this for authed via api key
  id.commit("No one", "no-one@example.com", "create paste")?;

  // return success
  let output = Success::new(*id, deletion_key);
  Ok(Status::show_success(HttpStatus::Ok, output))
}
