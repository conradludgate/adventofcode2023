fn main() {
    let day = time::OffsetDateTime::now_local().unwrap().day() as i32;

    cargo_aoc_setup::create_project(day);
    cargo_aoc_setup::get_project_description(day);
}
