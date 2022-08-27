mod site;
mod albums;

fn main() {
    let albums = albums::read_albums();
    site::build_site(albums);
}
