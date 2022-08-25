mod build_site;
mod read_albums;

fn main() {
    let albums = read_albums::read_albums();
    build_site::build_site(albums);
}
