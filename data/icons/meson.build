application_id = 'io.richard.kronos'

scalable_dir = 'hicolor' / 'scalable' / 'apps'
install_data(
    scalable_dir / ('@0@.svg').format(application_id),
    install_dir: get_option('datadir') / 'icons' / scalable_dir,
)

symbolic_dir = 'hicolor' / 'symbolic' / 'apps'
install_data(
    symbolic_dir / ('@0@-symbolic.svg').format(application_id),
    install_dir: get_option('datadir') / 'icons' / symbolic_dir,
)

meson.add_devenv(environment({'XDG_DATA_DIRS': meson.project_source_root() / 'data'}))
