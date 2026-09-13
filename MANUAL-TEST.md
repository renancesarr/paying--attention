# Manual de Build e Teste

## Estado Atual

O repositório possui o núcleo puro `paying_attention_core`, o binário técnico
`paying-attention` e o spike GTK4 `paying-attention-desktop`. O aplicativo
desktop ainda é um spike de interface: ele oferece Check-in, Review e
Recuperação de Distração, mas ainda não contém os timers reais do produto.

Por isso, neste momento:

- `cargo build --workspace` compila todos os crates.
- `cargo test --workspace` executa as regras puras e os contratos dos formulários.
- `cargo run -p paying_attention_desktop` abre o Check-in do spike.

Use as prévias de Review e Recuperação de Distração para validar seus fluxos
visuais antes de existir o runtime de timers.

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

Execute o Check-in:

```bash
cargo build -p paying_attention_desktop
cargo run -p paying_attention_desktop
```

Execute a prévia de Review:

```bash
PAYING_ATTENTION_SCREEN=review cargo run -p paying_attention_desktop
```

Execute a prévia de Recuperação de Distração:

```bash
PAYING_ATTENTION_SCREEN=drift-recovery cargo run -p paying_attention_desktop
```

Cada comando deve abrir uma janela GTK4/libadwaita sem decorações e em tela
cheia em cada monitor conectado. As prévias são ferramenta de desenvolvimento,
não uma substituição para o runtime de Attention Workflow.

## Roteiro Manual do Spike

Com a janela em tela cheia, execute e registre cada item:

- [ ] Cada monitor conectado recebe uma janela em tela cheia e sem barra de título.
- [ ] A janela recebe foco ao abrir.
- [ ] Texto ou outro controle da janela recebe input de teclado.
- [ ] Review mostra a tarefa anterior, exige justificativa para `Não concluída` e `Em andamento`, e permite declarar nova tarefa ou continuar dentro do contador.
- [ ] Recuperação de Distração exige relato, categoria e uma ação consciente antes de liberar o botão.
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
