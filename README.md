# API Rust - Actix Web

API REST simples desenvolvida em Rust utilizando o framework Actix Web.

## 📋 Pré-requisitos

- **Rust** (versão 1.70 ou superior)
- **Cargo** (gerenciador de pacotes do Rust)

### Instalação do Rust

Se você ainda não tem o Rust instalado, execute:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Após a instalação, reinicie o terminal ou execute:

```bash
source $HOME/.cargo/env
```

## 📦 Dependências

As dependências do projeto estão definidas no arquivo `Cargo.toml`:

```toml
[dependencies]
actix-web = "4.13.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Dependências Principais

- **actix-web** (4.13.0): Framework web assíncrono de alta performance para Rust
- **serde** (1.0): Framework de serialização/deserialização para Rust (com feature "derive" para macros)
- **serde_json** (1.0): Suporte para serialização e deserialização de JSON usando Serde

## 🚀 Como Executar

### Modo Normal

Para compilar e executar a aplicação:

```bash
cargo run
```

A API estará disponível em: `http://127.0.0.1:3000`

### Modo Watch (Desenvolvimento)

Para executar com recarregamento automático durante o desenvolvimento:

1. Instale o `cargo-watch`:

   ```bash
   cargo install cargo-watch
   ```

2. Execute o projeto em modo watch:
   ```bash
   cargo watch -x run
   ```

Com o modo watch ativo, qualquer alteração nos arquivos `.rs` irá recompilar e reiniciar automaticamente a aplicação.

### Outras Opções Úteis

```bash
# Compilar sem executar
cargo build

# Compilar em modo release (otimizado)
cargo build --release

# Executar a versão otimizada
cargo run --release

# Verificar o código sem compilar (rápido)
cargo check

# Watch com clear da tela antes de cada execução
cargo watch -c -x run

# Watch executando apenas após compilação bem-sucedida
cargo watch -x check -x run
```

## 📡 Endpoints Disponíveis

### GET /

Retorna uma mensagem de boas-vindas.

```bash
curl http://127.0.0.1:3000/
```

**Resposta:** `Hello world!`

### GET /echo_teste

Endpoint de teste.

```bash
curl http://127.0.0.1:3000/echo_teste
```

**Resposta:** `Teste 3!`

### POST /echo

Retorna o corpo da requisição enviado.

```bash
curl -X POST http://127.0.0.1:3000/echo \
  -H "Content-Type: text/plain" \
  -d "Sua mensagem aqui"
```

**Resposta:** `Sua mensagem aqui`

### GET /hey

Endpoint alternativo de saudação.

```bash
curl http://127.0.0.1:3000/hey
```

**Resposta:** `Hey there!`

## 🛠️ Desenvolvimento

### Estrutura do Projeto

```
api/
├── Cargo.toml          # Configuração do projeto e dependências
├── src/
│   └── main.rs         # Código principal da aplicação
└── README.md           # Este arquivo
```

### Limpando o Build

Para remover os arquivos compilados:

```bash
cargo clean
```

## 📝 Notas

- A aplicação está configurada para rodar em `127.0.0.1:3000`
- Todos os endpoints são assíncronos utilizando o runtime Tokio através do Actix
- A edição do Rust está configurada como **2024** no `Cargo.toml`

## 🔍 Logs e Debug

Para executar com logs detalhados, você pode configurar a variável de ambiente `RUST_LOG`:

```bash
RUST_LOG=debug cargo run
```

Níveis de log disponíveis: `error`, `warn`, `info`, `debug`, `trace`

---

Desenvolvido com 🦀 Rust e Actix Web
# api-rust
