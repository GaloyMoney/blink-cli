use static_files::resource_dir;

fn main() -> std::io::Result<()> {
    resource_dir("src/app/server/public").build()
}
