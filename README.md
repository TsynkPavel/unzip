# parallel-unzip

Параллельный распаковщик zip-архивов на базе [ripunzip](https://github.com/google/ripunzip).

## Сборка

```bash
cargo build --release
```

Готовый бинарь: `target/release/parallel-unzip`

## Использование

```bash
# Распаковать в текущий каталог
./target/release/parallel-unzip archive.zip

# Распаковать в указанный каталог
./target/release/parallel-unzip archive.zip -d ./out
```

После завершения утилита выводит, сколько секунд заняла распаковка.
