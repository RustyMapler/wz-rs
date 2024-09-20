pub fn resolve_uol_path(original_path: String, uol_path: String) -> String {
    // Figure out how many times to backtrack
    let splitted_uol_path = uol_path.split("../");
    let splitted_uol_path = splitted_uol_path.collect::<Vec<&str>>();
    let last_path = splitted_uol_path.last().unwrap();
    let backtrack_len = splitted_uol_path.len();

    let splitted_original_path = original_path.split("/");
    let mut splitted_original_path = splitted_original_path.collect::<Vec<&str>>();

    // Backtrack the original path
    for _ in 0..backtrack_len {
        splitted_original_path.pop();
    }

    // Apply the new UOL path
    splitted_original_path.push(last_path);

    // Join into a path and return
    splitted_original_path.join("/")
}
