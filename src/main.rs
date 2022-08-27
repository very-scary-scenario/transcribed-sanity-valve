mod albums;
mod lyrics;
mod site;

fn main() {
    let albums = albums::read_albums();
    site::build_site(albums);
}
