# Validação Manual dos Tickets 10, 12, 15, 16 e 17

Use este roteiro no Ubuntu 26.04 com GNOME em Wayland. Ele valida o
comportamento que depende do desktop real e produz evidência reaproveitável
para fechar os tickets. Não use credenciais reais do Telegram durante estes
testes.

## Preparação

Na raiz do repositório, execute:

```bash
cargo build -p paying_attention_desktop
echo "$XDG_SESSION_TYPE"
gnome-shell --version
```

Registre no resultado:

- versão do Ubuntu;
- versão do GNOME;
- valor de `XDG_SESSION_TYPE`;
- quantidade, resolução e disposição dos monitores;
- workspace ativo e atalhos personalizados relevantes.

Crie uma pasta de evidências que não será versionada com dados pessoais:

```bash
mkdir -p /tmp/paying-attention-evidence
```

Para cada item aprovado, guarde uma captura de tela ou vídeo curto e uma nota
com o comando usado, o resultado observado e a data. Para falhas, registre o
mesmo material e descreva a diferença entre o esperado e o observado.

O atalho de desenvolvimento `Ctrl+Shift+F` fecha o aplicativo. Use-o antes de
abrir outra execução. Não habilite autostart durante esta validação.

## Registro Estruturado de Evidência

Além de capturas e do relatório de evidência, grave cada observação no banco
SQLite de validação. Ele é separado do banco de dados pessoal do aplicativo e
fica em `$XDG_STATE_HOME/paying-attention/manual-validation.sqlite`, ou em
`~/.local/state/paying-attention/manual-validation.sqlite` quando
`XDG_STATE_HOME` não estiver definido.

Depois de observar um comportamento, registre-o com o CLI. Exemplo para o
fullscreen testado no ticket 10:

```bash
cargo run -p paying_attention_cli -- validation record \
  --ticket 10 \
  --check fullscreen-on-both-displays \
  --outcome passed \
  --observed-at "$(date --iso-8601=seconds)" \
  --command "cargo run -p paying_attention_desktop" \
  --observation "Attention Block abriu em fullscreen nas duas telas." \
  --artifact "/tmp/paying-attention-evidence/fullscreen.png"
```

Os resultados aceitos são `passed`, `failed` e `blocked`. Para registrar o
resultado já observado de `Alt+Tab`, use `failed`, pois ele contorna o bloqueio
pretendido:

```bash
cargo run -p paying_attention_cli -- validation record \
  --ticket 10 \
  --check alt-tab \
  --outcome failed \
  --observed-at "$(date --iso-8601=seconds)" \
  --command "cargo run -p paying_attention_desktop" \
  --observation "Alt+Tab alternou para outra aplicacao e contornou a Attention Block."
```

Gere um artefato Markdown versionável quando terminar uma sessão de testes:

```bash
mkdir -p docs/evidence
cargo run -p paying_attention_cli -- validation report --ticket 10 \
  > docs/evidence/manual-validation-ticket-10.md
```

O SQLite é a fonte estruturada; o Markdown gerado é a evidência legível para
revisão e Git. Não registre tarefas pessoais, justificativas sensíveis, tokens
do Telegram ou capturas que contenham essas informações.

## Ticket 10: Fullscreen GTK4 no GNOME Wayland

Execute:

```bash
cargo run -p paying_attention_desktop
```

Verifique:

- [ ] Cada monitor conectado abriu uma janela própria, sem barra de título e em tela cheia.
- [ ] A janela recebeu foco e o campo de tarefa aceitou teclado imediatamente.
- [ ] `Alt+Tab` foi testado: registre se permite contornar a Attention Block e quão fácil isso é.
- [ ] `Super+Tab` foi testado e registrado.
- [ ] Trocar de workspace foi testado e registrado.
- [ ] Clicar ou digitar em outra aplicação foi testado e registrado.
- [ ] `Ctrl+Shift+F` encerrou todas as janelas sem travar a sessão.

**Evidência mínima:** uma captura mostrando todas as telas bloqueadas, uma
captura ou vídeo do foco no campo de texto e a tabela de resultados dos atalhos
testados. Atualize
`docs/evidence/gtk4-fullscreen-gnome-wayland.md` com a decisão `go` ou
`no-go`, limitações observadas e o caminho das capturas.

Considere `no-go` se a janela não entra em fullscreen, não recebe foco, ou se
`Alt+Tab`/`Super+Tab` tornam o bloqueio trivial de ignorar. Isso não impede o
MVP comportamental, mas confirma que uma integração GNOME mais forte será
necessária no futuro.

## Ticket 12: Review e Recuperação de Distração

Execute as duas prévias separadamente:

```bash
PAYING_ATTENTION_SCREEN=review cargo run -p paying_attention_desktop
PAYING_ATTENTION_SCREEN=drift-recovery cargo run -p paying_attention_desktop
```

Na tela **Review**, verifique:

- [ ] A tarefa anterior está visível.
- [ ] É possível selecionar relevante ou irrelevante.
- [ ] Com `Não concluída` e justificativa vazia, o botão de iniciar próximo foco permanece desabilitado.
- [ ] Com `Em andamento` e justificativa vazia, o botão também permanece desabilitado.
- [ ] Ao preencher a justificativa, o botão é habilitado quando as demais respostas estão completas.
- [ ] A opção de nova tarefa exige texto.
- [ ] A continuação mostra o contador e só fica disponível dentro do limite.

Na tela **Recuperação de Distração**, verifique:

- [ ] Relato vazio mantém o botão desabilitado.
- [ ] Categoria, relato e ação consciente são exigidos.
- [ ] Ação `Declarar nova tarefa` habilita o campo de próxima tarefa e exige texto.
- [ ] Retomar, reiniciar e marcar incompleta funcionam sem exigir nova tarefa.

**Evidência mínima:** uma captura de cada tela em estado inválido e em estado
válido, identificando as escolhas feitas. Não inclua tarefa ou justificativa
sensível nas imagens.

## Ticket 15: Tray, Meeting Mode e instância única

Execute o aplicativo e abra o ícone na área de notificação do GNOME:

```bash
cargo run -p paying_attention_desktop
```

Verifique:

- [ ] O menu exibe `Iniciar modo reunião`, `Abrir configurações` e `Abrir histórico de atenção`.
- [ ] `Iniciar modo reunião` abre um formulário com motivo e 30, 60 e 90 minutos.
- [ ] O botão de iniciar reunião fica desabilitado sem motivo.
- [ ] Com motivo e duração, o formulário fecha após iniciar o modo reunião.
- [ ] As opções de configurações e histórico abrem janelas próprias.

### Instância única

Com a primeira execução aberta, inicie uma segunda instância em outro terminal:

```bash
target/debug/paying-attention-desktop
```

**Esperado:** a segunda execução deve encerrar ou encaminhar o pedido para a
primeira, sem criar outra Attention Block, outro tray ou timers concorrentes.

**Evidência mínima:** captura do menu aberto, captura do formulário de reunião
e saída dos dois processos. Se a segunda instância abrir uma segunda janela ou
um segundo tray, marque o ticket como **falhou**: isso é uma lacuna de
implementação, não uma pendência manual que possa ser aprovada.

## Ticket 16: Configurações no SQLite

Abra `Abrir configurações` pelo tray. Altere valores que sejam fáceis de
reconhecer, por exemplo:

- atraso de boot: `7`;
- ciclo de foco: `25`;
- inatividade de Attention Block: `3`;
- inatividade de foco: `9`;
- duração padrão de reunião: `30`.

Use um caminho de áudio de teste sem conteúdo pessoal e deixe os campos do
Telegram vazios. Salve, feche a janela, reabra as configurações e confirme que
os valores persistiram.

O banco esperado é:

```bash
DB="${XDG_DATA_HOME:-$HOME/.local/share}/paying-attention/paying-attention.sqlite"
sqlite3 "$DB" 'SELECT boot_delay_minutes, focus_cycle_minutes, attention_block_idle_minutes, focus_cycle_idle_minutes, meeting_default_duration_minutes FROM app_config;'
```

**Evidência mínima:** captura antes/depois da reabertura e a saída do comando
SQL acima. O resultado deve conter os valores salvos. Não registre token ou
chat id do Telegram em texto, captura ou commit.

Se `sqlite3` não estiver instalado, a reabertura da tela continua sendo a
evidência funcional; anote essa ausência no relatório.

## Ticket 17: Histórico de Atenção

O runtime atual ainda não gera registros de Focus Cycle automaticamente. Para
validar a leitura e apresentação do histórico sem misturar dados pessoais,
insira uma amostra no banco de teste depois de abrir e fechar o aplicativo uma
vez, para que ele crie o schema.

```bash
DB="${XDG_DATA_HOME:-$HOME/.local/share}/paying-attention/paying-attention.sqlite"
sqlite3 "$DB" <<'SQL'
INSERT INTO focus_cycles (
  declared_task, started_at, ended_at, relevance, completion, completion_justification
) VALUES (
  'Validar historico', '2026-09-13T08:00:00-03:00', '2026-09-13T08:20:00-03:00',
  'relevant', 'in_progress', 'Ainda ajustando o runtime.'
);
INSERT INTO attention_history (event_type) VALUES
  ('nagging_started'),
  ('drift_recovery_submitted'),
  ('meeting_mode_started'),
  ('telegram_failed');
SQL
```

Abra `Abrir histórico de atenção` pelo tray e verifique:

- [ ] O ciclo mostra tarefa, início, fim, status e justificativa.
- [ ] Os quatro eventos de amostra são apresentados com texto em PT-BR.
- [ ] A janela não mostra logs técnicos, gráficos, gamificação ou análise por IA.

**Evidência mínima:** captura da janela de histórico, saída de
`sqlite3 "$DB" 'SELECT * FROM focus_cycles;'` e uma nota de que os dados foram
amostras descartáveis. Após a validação, remova somente os registros inseridos
para teste ou use um diretório `XDG_DATA_HOME` temporário desde o início.

## Resultado Consolidado

Para cada ticket, registre `aprovado`, `falhou` ou `bloqueado`, a data, o
ambiente e links/caminhos para a evidência. Um ticket só pode ser considerado
validado quando todos os itens marcados para ele passaram ou tiverem uma falha
documentada que virou nova tarefa de implementação.
