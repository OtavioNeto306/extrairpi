# Extrator de RPI

Aplicativo desktop multiplataforma para extrair e filtrar deferimentos de RPI (Registro de Propriedade Intelectual) de arquivos PDF, removendo automaticamente registros que não possuem procurador.

## 🚀 Características

- **Interface Moderna**: Interface responsiva com drag-and-drop para arquivos PDF
- **Processamento Inteligente**: Extração automática de dados usando expressões regulares
- **Filtragem Automática**: Remove registros sem procurador conforme especificado
- **Exportação**: Suporte para exportação em formato CSV
- **Multiplataforma**: Funciona no Windows, macOS e Linux
- **Performance**: Backend em Rust para processamento rápido e eficiente

## 📋 Pré-requisitos

- [Rust](https://rustup.rs/) (versão 1.70 ou superior)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/prerequisites)

### Instalação do Tauri CLI

```bash
cargo install tauri-cli
```

## 🛠️ Instalação e Execução

1. **Clone o repositório**:
   ```bash
   git clone <url-do-repositorio>
   cd extrairpi
   ```

2. **Configure as variáveis de ambiente** (opcional):
   ```bash
   cp .env.example .env
   # Edite o arquivo .env conforme necessário
   ```

3. **Execute em modo de desenvolvimento**:
   ```bash
   cargo tauri dev
   ```

4. **Compile para produção**:
   ```bash
   cargo tauri build
   ```

## 📁 Estrutura do Projeto

```
extrairpi/
├── src/                    # Frontend (HTML, CSS, JS)
│   ├── index.html         # Interface principal
│   ├── main.js           # Lógica JavaScript
│   └── style.css         # Estilos CSS
├── src-tauri/            # Backend Rust
│   ├── src/
│   │   ├── lib.rs        # Lógica principal
│   │   └── main.rs       # Ponto de entrada
│   ├── Cargo.toml        # Dependências Rust
│   └── tauri.conf.json   # Configuração Tauri
├── .env.example          # Exemplo de variáveis de ambiente
├── .gitignore           # Arquivos ignorados pelo Git
└── README.md            # Este arquivo
```

## 🔧 Funcionalidades

### Extração de Dados

O aplicativo extrai os seguintes campos de cada registro:

- **Processo**: Número do processo
- **Titular**: Nome do titular do registro
- **NCL**: Classificação NCL (Nice Classification)
- **Especificação**: Descrição detalhada do produto/serviço

### Filtragem Inteligente

- Remove automaticamente registros que não possuem procurador
- Mantém apenas registros com representação legal válida
- Processa múltiplos registros em um único arquivo PDF

### Interface do Usuário

- **Drag & Drop**: Arraste arquivos PDF diretamente para a interface
- **Seleção de Arquivos**: Botão para seleção manual de arquivos
- **Feedback Visual**: Indicadores de progresso durante o processamento
- **Resultados Interativos**: Visualização clara dos dados extraídos
- **Exportação**: Botões para copiar e exportar resultados

## 🎯 Como Usar

1. **Inicie a aplicação** executando `cargo tauri dev`
2. **Carregue um arquivo PDF**:
   - Arraste e solte o arquivo na área indicada, ou
   - Clique em "Selecionar Arquivo" para escolher manualmente
3. **Aguarde o processamento** - o feedback visual mostrará o progresso
4. **Visualize os resultados** na lista de registros extraídos
5. **Exporte os dados**:
   - Copie registros individuais ou todos de uma vez
   - Exporte para CSV usando o botão "Exportar CSV"

## ⚙️ Configuração

### Variáveis de Ambiente

O arquivo `.env.example` contém todas as configurações disponíveis:

- `RUST_LOG`: Nível de log (trace, debug, info, warn, error)
- `MAX_PDF_SIZE_MB`: Tamanho máximo do arquivo PDF em MB
- `PDF_PROCESSING_TIMEOUT`: Timeout para processamento em segundos
- `DEFAULT_EXPORT_FORMAT`: Formato padrão de exportação (csv, json)

### Personalização

- **Regex Patterns**: Modifique as expressões regulares em `src-tauri/src/lib.rs`
- **Interface**: Customize cores e layout em `src/style.css`
- **Funcionalidades**: Adicione novos recursos em `src/main.js`

## 🔍 Detalhes Técnicos

### Backend (Rust)

- **pdf-extract**: Extração de texto de arquivos PDF
- **regex**: Processamento de padrões de texto
- **serde**: Serialização/deserialização de dados
- **tokio**: Runtime assíncrono

### Frontend

- **HTML5**: Estrutura semântica
- **CSS3**: Estilos modernos com gradientes e animações
- **JavaScript ES6+**: Lógica interativa e comunicação com backend
- **Tauri API**: Integração entre frontend e backend

### Arquitetura

- **Separação de Responsabilidades**: Backend para processamento, frontend para interface
- **Comunicação Assíncrona**: Comandos Tauri para comunicação segura
- **Tratamento de Erros**: Validação robusta em todas as camadas
- **Performance**: Processamento otimizado para arquivos grandes

## 🐛 Solução de Problemas

### Problemas Comuns

1. **Erro de compilação**:
   - Verifique se o Rust está atualizado: `rustup update`
   - Limpe o cache: `cargo clean`

2. **Arquivo PDF não processa**:
   - Verifique se o arquivo não está corrompido
   - Confirme se o tamanho está dentro do limite configurado

3. **Interface não carrega**:
   - Verifique se os arquivos estão no diretório `src/`
   - Confirme a configuração em `tauri.conf.json`

### Logs de Debug

Para ativar logs detalhados:

```bash
RUST_LOG=debug cargo tauri dev
```

## 📄 Licença

Este projeto está sob a licença MIT. Veja o arquivo `LICENSE` para mais detalhes.

## 🤝 Contribuição

Contribuições são bem-vindas! Por favor:

1. Faça um fork do projeto
2. Crie uma branch para sua feature (`git checkout -b feature/AmazingFeature`)
3. Commit suas mudanças (`git commit -m 'Add some AmazingFeature'`)
4. Push para a branch (`git push origin feature/AmazingFeature`)
5. Abra um Pull Request

## 📞 Suporte

Para suporte e dúvidas:

- Abra uma issue no repositório
- Consulte a documentação do [Tauri](https://tauri.app/)
- Verifique a documentação do [Rust](https://doc.rust-lang.org/)

---

**Desenvolvido com ❤️ usando Tauri + Rust**