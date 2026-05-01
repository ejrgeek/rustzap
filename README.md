# RustZap

Cliente desktop nativo para WhatsApp Web no Linux, construído com Rust e Tauri. Empacota a interface web oficial num webview isolado, com integração ao ambiente de trabalho e suporte a Wayland.

## Funcionalidades

- Carregamento direto do WhatsApp Web (sem frontend local)
- Modo escuro forçado, sem flash branco inicial
- [TESTAR/PROBLEMAS] Ocultação da janela ao fechar — sessão permanece ativa ?
- [TESTAR/PROBELMAS] Ícone na bandeja do sistema com menu de contexto (alternar, configurações, sair)
- [TESTAR/PROBLEMAS] Notificações nativas via D-Bus/Libnotify ?
- Persistência de sessão (cookies, localStorage)
- Suporte a telas HiDPI, touchscreen e aceleração de hardware (compatível com Dell XPS 13)
- [TESTAR] Funciona nativamente em Wayland (sem XWayland) ?

## Pré-requisitos

O projeto depende das seguintes bibliotecas de desenvolvimento (exemplo para Ubuntu 26.10):

```bash
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
  libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
```

Para gerar AppImage, instale também:

```bash
sudo apt install libfuse2
```

Para desenvolvimento é necessário ter o Rust e o CLI do Tauri v2:

```bash
cargo install tauri-cli --version "^2"
```


## Compilação

Modo desenvolvimento:
```bash
cargo tauri dev
```

Build de produção:

```bash
cargo tauri build
```

## Instalação
Pacote Debian:

```bash
sudo dpkg -i src-tauri/target/release/bundle/deb/RustZap_x.x.x_amd64.deb
```


AppImage:

```bash
chmod +x src-tauri/target/release/bundle/appimage/RustZap_x.x.x_amd64.AppImage
./src-tauri/target/release/bundle/appimage/RustZap_x.x.x_amd64.AppImage
```

Binário isolado:

```bash
./src-tauri/target/release/rustzap
```

## Build Flatpak

#### Localmente

Instale as ferramentas:
```bash
sudo apt install flatpak-builder
flatpak remote-add --user --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
flatpak install --user -y org.gnome.Platform//46 org.gnome.Sdk//46
```

Compile o app:
```bash
cargo tauri build
```

Execute o script
```bash
./flatpak/build-flatpak.sh
```


#### GitHub Actions



```bash

```


## Licença MIT