fn main() {
    // Force a dark widget style so the panel matches the dark canvas regardless of
    // the host's light/dark system appearance (the labels/titles assume a dark
    // background). Overridable at build time via the SLINT_STYLE env var.
    let config = slint_build::CompilerConfiguration::new().with_style("fluent-dark".into());
    slint_build::compile_with_config("ui/app.slint", config).unwrap();
}
