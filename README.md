# Mosec

## Настройка базы данных
```sh
cargo install sqlx-cli --no-default-features --features=rustls,sqlite
sqlx database create
sqlx migrate run

# cargo sqlx prepare
```

## Запуск проекта
```sh
cargo run
```

## Работа с sqlx-cli
> https://github.com/launchbadge/sqlx/tree/main/sqlx-cli

## Ресурсы
> design.tusur.ru

## Модель
Узлы <-> Транзакция <-> Категория
