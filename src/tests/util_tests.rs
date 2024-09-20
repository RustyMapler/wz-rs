#[cfg(test)]
mod tests {
    use crate::resolve_uol_path;

    #[test]
    fn it_resolves_uol_relative_path() {
        assert_eq!(
            resolve_uol_path(
                "00012000.img/stand1/0/head".to_string(),
                "../../front/head".to_string()
            ),
            "00012000.img/front/head"
        )
    }

    #[test]
    fn it_resolves_uol_single_path() {
        assert_eq!(
            resolve_uol_path("Obj/acc6.img/minar/acc/25/11".to_string(), "1".to_string()),
            "Obj/acc6.img/minar/acc/25/1"
        )
    }
}
