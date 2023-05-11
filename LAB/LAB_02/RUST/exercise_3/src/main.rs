

mod lib;

fn main() {
    let mut file_system = lib::FileSystem::new();
    file_system.mk_dir("a/".to_string());
    file_system.mk_dir("a/b/".to_string());
    file_system.mk_dir("a/b/c/".to_string());

    println!("{}", file_system.root);
}
