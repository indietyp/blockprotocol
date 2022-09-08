pub(crate) fn camel_case_to_pascal_case(value: &str) -> String {
    value
        .split('-')
        .map(ToOwned::to_owned)
        .map(|mut value| {
            if let Some(first) = value.get_mut(0..1) {
                first.make_ascii_uppercase();
            }

            value
        })
        .collect::<Vec<_>>()
        .concat()
}
