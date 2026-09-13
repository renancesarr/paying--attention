# Manual de Build e Teste

## Estado Atual

O repositório possui o crate puro `paying_attention_core`. Ele valida a lógica de domínio, mas ainda não há um executável desktop, janela GTK ou binário chamado `paying-attention`.

Por isso, neste momento:

- `cargo build` compila a biblioteca.
- `cargo test -p paying_attention_core` executa os testes do Attention Workflow.
- `cargo run` ainda não é um teste válido: não existe um target binário para iniciar.

O próximo passo de produto é o spike da issue `#10`: criar um crate desktop temporário com GTK4/libadwaita e verificar seu comportamento no GNOME/Wayland.

## Validar o Núcleo Atual

Na raiz do repositório, execute:

```bash
cargo fmt --check
cargo test -p paying_attention_core
cargo build -p paying_attention_core
```

Resultado esperado:

- Formatação válida.
- Cinco testes de integração aprovados.
- Biblioteca compilada sem GTK, SQLite, relógio real, Telegram ou filesystem.

## Preparar o Spike GTK

Antes de criar ou executar o crate desktop, registre o ambiente que será testado:

```bash
echo "$XDG_SESSION_TYPE"
gnome-shell --version
```

Anote também manualmente:

- Versão do Ubuntu.
- Número e disposição dos monitores.
- Workspace ativo.
- Atalhos personalizados de GNOME que possam interferir no teste.

O spike precisará das ferramentas de compilação e dos pacotes de desenvolvimento GTK4/libadwaita. Confirme a disponibilidade deles no Ubuntu antes da instalação:

```bash
apt-cache policy libgtk-4-dev libadwaita-1-dev pkg-config
```

Quando os pacotes estiverem disponíveis, a instalação típica é:

```bash
sudo apt update
sudo apt install build-essential pkg-config libgtk-4-dev libadwaita-1-dev
```

## Build e Execução do Spike

Depois que a issue `#10` criar o crate planejado `paying_attention_desktop`, execute:

```bash
cargo build -p paying_attention_desktop
cargo run -p paying_attention_desktop
```

O resultado esperado é uma janela GTK4/libadwaita sem decorações e em tela cheia no monitor principal. Ela é uma prova técnica, não o bloqueio final do produto.

## Roteiro Manual do Spike

Com a janela em tela cheia, execute e registre cada item:

- [ ] A janela abre em tela cheia e sem barra de título.
- [ ] A janela recebe foco ao abrir.
- [ ] Texto ou outro controle da janela recebe input de teclado.
- [ ] `Alt+Tab` foi testado e o efeito foi registrado.
- [ ] `Super+Tab` foi testado e o efeito foi registrado.
- [ ] Troca de workspace foi testada e o efeito foi registrado.
- [ ] Interação com outra aplicação foi testada e o efeito foi registrado.
- [ ] O atalho de desenvolvimento `Ctrl+Shift+F` encerra a janela sem travar a sessão.

Capture pelo menos uma imagem ou vídeo curto mostrando a janela em tela cheia. Não há obrigação de o spike impedir toda saída do desktop: ele deve medir a fricção real no GNOME/Wayland.

## Decisão do Spike

Registre a evidência em `docs/evidence/gtk4-fullscreen-gnome-wayland.md` e inclua:

- Ambiente testado.
- Resultado de cada item do roteiro.
- Caminho para captura de imagem ou vídeo.
- Decisão `go` ou `no-go` para GTK4/libadwaita.
- Limitações observadas.

GTK é insuficiente para o produto final se não abrir em tela cheia, perder foco imediatamente, permitir ignorar via `Alt+Tab` ou `Super+Tab` de forma trivial, ou não receber input de modo confiável. Nesse caso, o MVP ainda pode continuar como ferramenta comportamental se a fricção for suficiente para uso pessoal, mas a integração com GNOME Shell deve permanecer registrada como trabalho futuro.

## Recuperação

O spike não deve configurar autostart, Telegram, áudio em loop, banco de dados ou qualquer bloqueio permanente. Durante desenvolvimento, use apenas `Ctrl+Shift+F` para sair da janela do spike. O futuro comando técnico `paying-attention unlock --force` pertence ao MVP, não ao spike inicial.
