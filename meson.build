project('textedit', 'rust', 
          version: '0.1.2',
    meson_version: '>= 0.50.0',
  default_options: [ 'warning_level=2',
                   ],
)

i18n = import('i18n')


cargo_sources = files(
  'Cargo.toml',
  'Cargo.lock',
)

subdir('data')
subdir('src')
subdir('po')

meson.add_install_script('build-aux/meson/postinstall.py')
