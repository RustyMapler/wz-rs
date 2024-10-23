pub fn resolve_uol_path(original_path: String, uol_path: String) -> String {
    // Calculate the number of backtracks and get the last part of the UOL path
    let backtrack_len = uol_path.matches("../").count();
    let last_path = uol_path.rsplit("../").next().unwrap();

    // Split the original path and backtrack
    let mut splitted_original_path: Vec<&str> = original_path.split('/').collect();
    splitted_original_path.truncate(splitted_original_path.len().saturating_sub(backtrack_len));

    // Apply the new UOL path
    splitted_original_path.push(last_path);

    // Join into a path and return
    splitted_original_path.join("/")
}
