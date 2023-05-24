use crate::{
    req::Request,
    resp::{Response, Status},
};
use std::{env::current_dir, io, path::PathBuf};

#[derive(Debug, Clone)]
pub struct StaticFileHandler {
    root: PathBuf,
}

impl StaticFileHandler {
    pub fn in_current_dir() -> io::Result<StaticFileHandler> {
        current_dir().map(StaticFileHandler::with_root)
    }

    pub fn with_root(root: PathBuf) -> StaticFileHandler {
        StaticFileHandler { root }
    }

    pub async fn handle(&self, request: Request) -> anyhow::Result<Response> {
        let mut path = self.root.join(request.path.strip_prefix('/').unwrap());

        if !path.is_file() && !path.is_dir() {
            return Ok(Response::from_html(
                Status::NotFound,
                include_str!("../static/404.html"),
            ));
        }

        if !path.is_file() && path.is_dir() {
            path = path.join("index.html");
            if !path.is_file() {
                return Ok(Response::from_html(
                    Status::NotFound,
                    include_str!("../static/404.html"),
                ));
            }
        }

        let file = tokio::fs::File::open(&path).await?;
        Response::from_file(&path, file).await
    }
}
