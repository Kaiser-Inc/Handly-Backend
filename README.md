# Handly-Backend

## Requisitos

#### Rodando com docker
```
Docker 28.0.1 (Docker Copose V2)
```

#### Rodando no ambiente nativo
```
Rust 1.81.0
sqlx-cli 0.8.5
PostgreSQL 16.8
```

Para instalar o ```sqlx-cli``` rode o seguinte comando:
```
cargo install sqlx-cli
```

## Clonar o repositório
```
git clone git@github.com:Kaiser-Inc/Handly-Backend.git
cd handly-backend
```

## Configuração do ambiente
Crie um ```.env``` com as seguintes váriaveis:

```
# Chave obrigatória para assinar/verificar JWT
JWT_SECRET=secret_string

# Se for executar no ambiente local
# DATABASE_URL=postgres://user:password@localhost:port/db_name
# DATABASE_URL_TEST=postgres://user:password@localhost:port/
```

## Executar com Docker
```
docker compose build # primeira vez
docker compose up -d # sobe Postgres e API
# Teste com curl:
curl http://localhost:8080/health
```

## Executar sem Docker (para desenvolvimento)

PostgreSQL precisa estar rodando localmente e compatível com ```DATABASE_URL``` definido no ```.env```.

```
sqlx migrate run # aplica migrações
cargo run # inicia a API
```