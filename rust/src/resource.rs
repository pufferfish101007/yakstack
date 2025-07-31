use godot::{classes::Texture, prelude::*};

/// Loads an image texture, trying PNG first and then SVG after.
///
/// Godot's SVG renderer isn't perfect so PNGs are preferred, but SVGs
/// can be useful during development.
///
/// When providing generic arguments, the first generic argument `P` can be
/// inferred automatically using `_`; only `T` need be specified.
pub fn load_texture<P, T>(path: P) -> Gd<T>
where
    P: AsRef<str>,
    T: Inherits<Resource> + Inherits<Texture>,
{
    try_load(&format!("{}.png", path.as_ref()))
        .or_else(|_| {
            godot_warn!(
                r#"couldn't find png texture for "{}", loading svg instead"#,
                path.as_ref()
            );
            try_load(&format!("{}.svg", path.as_ref()))
        })
        .expect(format!("couldn't find png or svg texture for {}", path.as_ref()).as_str())
}
