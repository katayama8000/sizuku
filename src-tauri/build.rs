fn main() {
    // Recompile when the mode override flips so dev/build commands pick up the change.
    println!("cargo:rerun-if-env-changed=SIZUKU_MENU_BAR_MODE");
    tauri_build::build()
}
