Claro! Aqui está o README atualizado com a seção de uso:

---

# Make Text from Size

Esta aplicação utilitária foi feita para gerar um arquivo de texto com o tamanho que for passado no parâmetro.

## Pré-requisitos

Para utilizar, você precisa ter o Rust instalado na máquina para fazer o build.

### Instalando o Rust

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Instalando a aplicação

Faça o build da aplicação usando o Cargo:

```sh
cargo build --release
```

Mova o aplicativo para uma pasta de aplicações, por exemplo, `/opt`, e adicione um link simbólico para o diretório de binários do sistema.

### Linux

```sh
sudo mkdir -p /opt/make_text_from_size

sudo mv target/release/mts /opt/make_text_from_size

sudo ln -s /opt/make_text_from_size/mts /usr/bin/mts
```

### macOS

```sh
sudo mv target/debug/mts /usr/local/bin/mts
```

## Uso

Para utilizar a aplicação, basta rodar o comando `mts` seguido do tamanho do arquivo que você deseja criar. Por exemplo, para criar um arquivo de 1MB, você pode usar o seguinte comando:

```sh
mts 1 exemplo.txt
```

Isso irá gerar um arquivo de texto com o tamanho de 1mb e nome exemplo.txt.

---

Se precisar de mais alguma coisa ou tiver dúvidas, estou à disposição!