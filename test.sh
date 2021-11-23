ls --color=always | cargo run --quiet | cargo run --example line-appender --quiet -- "aaa " " bbb"

echo -e '\x1b[1;31mThis red\nbold text\nspawns many lines' | cargo run --quiet | cargo run --example line-appender --quiet -- "aaa " " bbb"