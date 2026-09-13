# Documento de Especificação de Software (SRS)
## Projeto: App de Foco e Atenção (MVP Linux)
**Versão:** 1.0 (MVP)
**Status:** Em Planejamento

---

## 1. Introdução
### 1.1. Propósito
Este documento define os requisitos funcionais e não-funcionais para o MVP de um aplicativo de desktop para Linux focado em gestão de atenção e bloqueio de distrações. O objetivo do software é forçar a declaração consciente de tarefas através de bloqueios de tela periódicos, utilizando atrito (incomodo visual/sonoro) para combater a procrastinação.

### 1.2. Escopo do MVP
O MVP contempla estritamente o **workflow mecânico de bloqueio, declaração de tarefa, foco e revisão**. Funcionalidades de análise de dados, organização automática de agenda ou gamificação complexa estão explicitamente fora do escopo desta versão.

---

## 2. Requisitos Funcionais (RF)

### RF01. Inicialização e Primeiro Ciclo
*   **RF01.1:** O aplicativo deve iniciar automaticamente em segundo plano no login do usuário.
*   **RF01.2:** Imediatamente após o login, um timer de **10 minutos** é iniciado.
*   **RF01.3:** Ao completar os 10 minutos, o aplicativo deve assumir o controle da tela principal (Fullscreen).

### RF02. Interface de Bloqueio (Kiosk Mode)
*   **RF02.1:** Quando ativo, o app deve renderizar uma janela sem decorações (sem barra de título, sem bordas) em tela cheia.
*   **RF02.2:** A janela deve sobrepor todas as outras aplicações, impedindo a interação do usuário com o sistema operacional até que o fluxo seja concluído.

### RF03. Fluxo de Check-in Inicial (Fase 1)
*   **RF03.1:** A tela de bloqueio inicial deve apresentar duas seleções de 5 opções (estilo "fuzzy"/rápido):
    *   *Ambiente:* (Ex: Casa, Escritório, Café, Biblioteca, Outro).
    *   *Ânimo/Energia:* (Ex: Exausto, Baixo, Neutro, Focado, Flow).
*   **RF03.2:** A tela deve exigir um campo de texto obrigatório para a **Definição da Tarefa Atual**.
*   **RF03.3:** O botão de "Liberar Tela" deve permanecer desabilitado até que as seleções de contexto e o texto da tarefa sejam preenchidos.

### RF04. Ciclo de Foco (Fase 2)
*   **RF04.1:** Após a liberação da tela, o app deve iniciar um timer de **20 minutos** e ocultar a interface.
*   **RF04.2:** O app deve monitorar silenciosamente a atividade do sistema (mouse/teclado) para fins do mecanismo de inatividade (ver RF06).

### RF05. Fluxo de Revisão e Próximo Ciclo (Fase 3)
*   **RF05.1:** Ao completar os 20 minutos, o app deve bloquear a tela novamente.
*   **RF05.2:** A tela deve exibir a tarefa do ciclo anterior e pedir duas respostas binárias/simples:
    *   *A tarefa foi realmente relevante?* (Sim / Não).
    *   *A tarefa foi concluída?* (Sim / Não / Em Andamento).
*   **RF05.3:** A tela deve exigir a definição da **Próxima Tarefa**. O usuário terá duas opções:
    *   Digitar uma nova tarefa em texto livre.
    *   Clicar no botão **"Continuar Tarefa Antiga"**.

### RF06. Regra de Continuação (Escalonamento)
*   **RF06.1:** Uma mesma tarefa pode ser executada no máximo **3 vezes** (1 declaração original + 2 continuações).
*   **RF06.2:** O botão "Continuar Tarefa Antiga" deve exibir um contador (ex: "Continuar (1/2)").
*   **RF06.3:** Ao atingir o limite de 2 continuações, o botão deve ser desabilitado, obrigando o usuário a digitar uma nova tarefa ou alterar o texto da atual.

### RF07. Sistema de "Nagging" (Punição por Inação)
*   **RF07.1:** Durante as telas de bloqueio (Fase 1 ou Fase 3), se não houver **nenhuma** interação (mouse ou teclado) por **2 minutos**, o "Modo Punição" é ativado.
*   **RF07.2.** **Ações do Modo Punição:**
    *   *Telegram:* Disparo de mensagem via Bot para o usuário alertando sobre a tela bloqueada.
    *   *Visual:* Transição constante e cíclica do fundo da tela entre o modo Dark e o modo White (efeito strobo/transição suave).
    *   *Sonoro:* Disparo de um som pré-definido (alerta/despertador) em loop.
*   **RF07.3:** Qualquer detecção de input (movimento de mouse ou tecla) deve **cancelar imediatamente** o Modo Punição (parar som, voltar tela ao normal).
*   **RF07.4:** Se o usuário der um input e voltar a ficar inativo, o timer de 2 minutos do Modo Punição **reinicia do zero**.

---

## 3. Requisitos Não-Funcionais (RNF)

### RNF01. Integração com Sistema Operacional (Linux)
*   **RNF01.1:** O app deve ser compatível com ambientes X11 e Wayland.
*   **RNF01.2:** A detecção de inatividade (idle) não deve depender de hooks globais de teclado/mouse (que são bloqueados no Wayland), mas sim consultar o gerenciador de sessões via **D-Bus** (ex: `org.gnome.Mutter.IdleMonitor` ou `org.freedesktop.ScreenSaver`).

### RNF02. Persistência de Dados
*   **RNF02.1:** Todos os dados (histórico de tarefas, respostas de relevância, contadores de continuação) devem ser salvos localmente.
*   **RNF02.2:** O banco de dados local recomendado é **SQLite** (via `rusqlite`), armazenado no diretório `~/.local/share/[nome_do_app]/`.

### RNF03. Configuração Externa
*   **RNF03.1:** As credenciais do Bot do Telegram (Token e Chat ID) e o caminho do arquivo de som devem ser configuráveis via um arquivo `.toml` ou `.json` na pasta de configuração do usuário (`~/.config/[nome_do_app]/`).

### RNF04. Performance
*   **RNF04.1:** Quando em estado de "Foco" (tela liberada), o consumo de CPU deve ser próximo de zero (apenas o timer e o monitor de idle via D-Bus rodando).

---

## 4. Arquitetura e Stack Tecnológica

A linguagem base será **Rust**, escolhida pela segurança de memória, performance e excelente ecossistema para integração com sistemas Linux.

### 4.1. Bibliotecas (Crates) Principais
*   **Interface Gráfica (UI):** `egui` + `eframe` (Imediato, fácil de criar telas fullscreen customizadas e transições de cor). *Nota: Se a overlay nativa do Wayland for estritamente necessária no futuro, migraremos para `smithay` / `wlr-layer-shell`.*
*   **Assincronismo e Timers:** `tokio` (Gerenciamento do loop principal, timers de 10/20 min e timers de 2 min do nagging).
*   **Integração OS / Idle:** `zbus` (Comunicação com D-Bus para detectar inatividade do sistema).
*   **Telegram:** `teloxide` ou `reqwest` (Chamadas REST para a API do Telegram).
*   **Áudio:** `rodio` (Playback do som de alerta).
*   **Banco de Dados:** `rusqlite` (Persistência local SQLite).
*   **Configuração:** `config` ou `serde` + `toml` (Leitura do arquivo de settings).

---

## 5. Máquina de Estados (State Machine)

O núcleo do aplicativo será uma Máquina de Estados Finita para garantir que não haja transições ilegais:

```text
[BOOT] 
  |-- (10 min) --> [CHECK_IN] 
                     |-- (Preencheu) --> [FOCUS] 
                     |-- (2 min idle) --> [NAGGING] --(Input)--> [CHECK_IN]
                     
[FOCUS] 
  |-- (20 min) --> [REVIEW]
                     |-- (Preencheu) --> [FOCUS] (Loop)
                     |-- (2 min idle) --> [NAGGING] --(Input)--> [REVIEW]
```

---

## 6. Escopo Negado (Fora do MVP)
Para garantir a entrega rápida, as seguintes funcionalidades **NÃO** serão implementadas nesta versão:
1.  **Dashboard de Analytics:** Nenhuma tela para visualizar gráficos de produtividade, humores ou ambientes.
2.  **Organização Automática:** O app não usará as respostas de "relevância" para reorganizar a agenda do usuário automaticamente.
3.  **Integração com Calendário:** O app não lê nem escreve em calendários externos (Google Calendar, Outlook).
4.  **Múltiplas Tarefas:** O app não permite definir uma lista de tarefas para o dia; ele lida estritamente com a tarefa do ciclo atual de 20 minutos.
