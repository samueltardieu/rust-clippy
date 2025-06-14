const DIR_LOCALS_EL: &str = ".dir-locals.el";
const DIR_LOCALS: &str = r#";; Emacs configuration
((nil . ((eglot-workspace-configuration .
           (:rust-analyzer
            (:rustc
             (:source "discover" :linkedProjects ["./Cargo.toml" "clippy_dev/Cargo.toml" "lintcheck/Cargo.toml"])
             :cargo
             (:features ["internal"]))))))
 ("tests/ui" . ((auto-mode-alist ("\\.fixed\\'" . rustic-mode)))))
"#;

pub fn setup_dir_locals(force_override: bool) {
    if !force_override && matches!(std::fs::exists(DIR_LOCALS_EL), Ok(true)) {
        eprintln!("error: `{DIR_LOCALS_EL}` already exists");
        return;
    }
    if std::fs::write(DIR_LOCALS_EL, DIR_LOCALS).is_err() {
        eprintln!("error: unable to create file `{DIR_LOCALS_EL}`");
        return;
    }
    println!("info: created `{DIR_LOCALS_EL}`");
}

pub fn remove_dir_locals() {
    if std::fs::remove_file(DIR_LOCALS_EL).is_err() {
        eprintln!("error: unable to remove `{DIR_LOCALS_EL}`");
        return;
    }
    println!("info: removed `{DIR_LOCALS_EL}`");
}
