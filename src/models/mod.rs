pub mod comic;
pub mod comic_response;

pub use comic::Comic;
pub use comic_response::ComicResponse;

// converts ComicResponse to Comic
pub fn comic(cr: &ComicResponse) -> Comic {
    comic::Comic {
        title: cr.title.clone(),
        number: cr.num,
        date: cr.formatted_date(),
        description: cr.alt.clone(),
        image: cr.img.clone(),
    }
}
