use serde::{Deserialize, Serialize};

pub mod field;

#[derive(Debug, Clone,Deserialize,Serialize)]
pub struct Clip{
    pub clip_id: crate::domain::clip::field::ClipId,
    pub shortcode: crate::domain::clip::field::Shortcode,
    pub content: crate::domain::clip::field::Content,
    pub title: crate::domain::clip::field::Title,
    pub posted: crate::domain::clip::field::Posted,
    pub expires: crate::domain::clip::field::Expires,
    pub password: crate::domain::clip::field::Password,
    pub hits: crate::domain::clip::field::Hits,
}