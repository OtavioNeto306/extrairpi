Prompt Completo para IA: Criação de Extrator de RPI com Tauri
Título do Projeto: Extrator de Deferimentos de RPI (Revista da Propriedade Industrial) com Tauri.

1. Objetivo Principal

Criar um aplicativo de desktop local e multiplataforma (Windows, macOS, Linux) usando o framework Tauri. O objetivo do aplicativo é permitir que um usuário faça o upload de um arquivo PDF (especificamente, uma RPI) e extraia automaticamente todos os registros de "Deferimento do pedido".

A regra de negócio crucial é: o sistema deve filtrar e exibir apenas os registros que não possuem um "Procurador" listado.

2. Tecnologias Requeridas

Framework: Tauri.

Backend: Rust (para os comandos principais do Tauri).

Frontend: JavaScript, HTML5 e CSS (para a interface). Não é necessário nenhum framework de frontend complexo (como React ou Vue) para esta V1; HTML/JS/CSS puros são suficientes.

Bibliotecas (Crates) Rust: Você precisará de serde (para serialização de dados), serde_json e uma biblioteca para leitura de PDF (como pdf-extract ou lopdf).

3. Funcionalidade Principal (Visão do Usuário)

O usuário abre o aplicativo de desktop.

Ele vê uma interface limpa com uma área para "arrastar e soltar" um arquivo PDF ou um botão de "Carregar Arquivo".

O usuário seleciona e carrega o arquivo PDF da RPI.

O aplicativo exibe um indicador de "processando...".

Após alguns segundos, a interface é preenchida com uma lista de todos os deferimentos de pedido que não possuem um procurador.

Cada item na lista exibe claramente: Número do Processo, Titular, NCL e a Especificação.

O usuário pode copiar facilmente as informações de um ou de todos os resultados.

4. Requisitos do Backend (Lógica em Rust)

Isto será implementado em src-tauri/src/main.rs.

Comando Tauri: Crie um comando Tauri assíncrono chamado processar_pdf que aceita o caminho (path) do arquivo PDF como argumento.

Leitura do PDF: Use uma biblioteca (crate) Rust para extrair o texto bruto de todas as páginas do PDF carregado.

Lógica de Parsing (Análise):

O texto extraído deve ser dividido em blocos de registro. Um novo registro começa com a string "Deferimento do pedido".

Cada bloco de registro (do início de um "Deferimento" até o início do próximo) deve ser analisado individualmente.

Lógica de Filtragem (A Regra Principal):

Para cada bloco de registro, verifique se a string "Procurador:" existe.

Se "Procurador:" existir dentro daquele bloco, o registro inteiro deve ser ignorado.

Se "Procurador:" não existir, o registro deve ser processado para extração.

Extração de Dados: Para os registros que passarem pelo filtro (sem procurador), extraia as seguintes informações (provavelmente usando Regex ou busca de string):


processo: O número do processo (ex: "934256136").


titular: O texto que segue "Titular:" (ex: "NL SOLUÇÕES DIGITAIS LTDA [BR/BA]").


ncl: O texto que segue "NCL(12):" (ex: "9").


especificacao: O texto completo que segue "Especificação:" até o final do bloco daquele registro (ex: "APLICATIVOS, BAIXÁVEIS...").

Estrutura de Retorno: Defina uma struct Rust chamada RegistroDeferido que será usada para enviar os dados ao frontend. Ela deve ser serializável (usando serde::Serialize).

Rust

#[derive(serde::Serialize, Clone)]
struct RegistroDeferido {
    processo: String,
    titular: String,
    ncl: String,
    especificacao: String,
}
O comando processar_pdf deve retornar Vec<RegistroDeferido> (um array de registros) para o frontend.

5. Requisitos do Frontend (HTML/JS/CSS)

index.html:

Um div que servirá como área de "drag-and-drop" (soltar arquivo).

Um <input type="file" id="upload-pdf" accept=".pdf"> (que pode estar oculto e ser ativado pela área de drop ou por um botão).

Um div com id "feedback" (para mostrar "Processando..." ou "Nenhum resultado encontrado.").

Um div com id "container-resultados" onde os resultados filtrados serão renderizados dinamicamente.

main.js:

Importar as funções invoke do Tauri.

Adicionar um "event listener" ao input de arquivo (ou ao evento "drop").

Quando um arquivo for selecionado, pegar seu caminho (path) e chamar o backend: invoke('processar_pdf', { caminho: arquivo.path }).

Aguardar a promessa (Promise) ser resolvida.

Receber o array Vec<RegistroDeferido>.

Limpar o container-resultados.

Iterar (fazer um loop) sobre o array de resultados e criar o HTML para cada registro (ex: um "card" com os 4 campos de dados).

Adicionar a lógica de "Copiar para área de transferência" para cada card.

style.css: (Opcional, mas útil)

Estilização básica para a área de upload e para os cards de resultado, tornando a leitura agradável.

6. Exemplo de Lógica de Decisão (Baseado no arquivo-fonte)

Exemplo 1 (Deve ser IGNORADO):


Entrada: "934256063... Titular: PROSPERITA FUNDO DE INVESTIMENTO... Procurador: A Provincia Marcas e Patentes Ltda..." 

Motivo: Contém a string "Procurador:".

Exemplo 2 (Deve ser PROCESSADO):


Entrada: "934256136... Titular: NL SOLUÇÕES DIGITAIS LTDA... NCL(12): 9... Especificação: APLICATIVOS..." 

Motivo: Não contém a string "Procurador:".

Saída Esperada (JSON):

JSON

{
  "processo": "934256136",
  "titular": "NL SOLUÇÕES DIGITAIS LTDA [BR/BA]",
  "ncl": "9",
  "especificacao": "APLICATIVOS, BAIXÁVEIS PROGRAMAS DE COMPUTADOR BAIXÁVEIS PROGRAMAS DE COMPUTADOR, GRAVADOS SOFTWARES DE COMPUTADOR, GRAVADOS (DA CLASSE 9)"
}
Exemplo 3 (Deve ser IGNORADO):


Entrada: "934256080... Titular: MULTITÉCNICA INDUSTRIAL LTDA... Procurador: Lancaster Marcas e Patentes..." 

Motivo: Contém a string "Procurador:".

7. Solicitação Final

Por favor, gere a estrutura de arquivos e o código-fonte completo para os seguintes arquivos para criar a Versão 1.0 deste aplicativo:

src-tauri/Cargo.toml (com as dependências tauri, serde, serde_json e pdf-extract).

src-tauri/src/main.rs (com a struct RegistroDeferido, a função main e o comando #[tauri::command] async fn processar_pdf(...) contendo toda a lógica de leitura, parsing e filtragem).

index.html (com o HTML para a interface de upload e a área de resultados).

main.js (com o JavaScript para lidar com o upload, chamar o backend Tauri e renderizar os resultados no HTML).

style.css (com estilos básicos para a aplicação).


Instruções Cruciais para a IA Desenvolvedora
1. O Ponto Mais Importante: Delimitação de Registros

O sistema não deve ler o PDF linha por linha. Ele deve primeiro extrair o texto completo de todas as 300+ páginas e, em seguida, dividir esse texto em "blocos" de registro.


Instrução: "Antes de qualquer filtragem, use Expressão Regular (Regex) para 'fatiar' o texto inteiro do PDF. O melhor delimitador (ponto de corte) para iniciar um novo registro é o número do processo, que é um número de 9 dígitos que sempre aparece no início de um registro (ex: 934256063 ). Cada "bloco" para análise irá do início de um número de processo até o início do próximo."


Por que isso é vital: Se você não fizer isso, o seu filtro de "Procurador" pode falhar. Você precisa garantir que, ao procurar por "Procurador:", você está olhando apenas dentro do bloco daquele registro específico, e não no registro seguinte.

2. A Lógica de Filtro Precisa ser "Tolerante a Falhas"

A extração de texto de PDFs raramente é perfeita. A palavra "Procurador:" pode vir com quebras de linha estranhas, espaços duplos ou erros.

Instrução: "Dentro de cada 'bloco' de registro, antes de verificar o filtro, normalize o texto: converta tudo para minúsculas e remova espaços múltiplos e quebras de linha. Então, verifique se o bloco contém a string procurador:."

Por que isso é vital: Sem isso, o filtro pode falhar se o PDF extrair o texto como Procurador : ou Procu\nrador:, e você acabará listando registros que deveria ignorar.

3. A Extração da "Especificação" é Multi-linha

A parte da "Especificação" é a mais longa e complexa, podendo ter várias linhas.


Instrução: "Ao extrair o campo Especificação:, a Expressão Regular (Regex) precisa estar no modo 'multi-linha' (dotall ou (?s)). Ela deve capturar tudo desde a palavra Especificação: até o final do bloco (ou seja, até o ponto onde o próximo registro começaria, que é o próximo número de processo de 9 dígitos)."

Por que isso é vital: Uma Regex simples pararia na primeira quebra de linha, e você extrairia apenas a primeira frase da especificação.

4. Feedback Visual é Obrigatório (Função Assíncrona)

Processar um PDF de 300 páginas não será instantâneo (pode levar de 2 a 10 segundos). A interface não pode simplesmente "congelar".

Instrução: "A função Rust processar_pdf deve ser assíncrona (async) para não travar a interface. No JavaScript, assim que o arquivo for enviado, mostre imediatamente uma mensagem de 'Processando... aguarde.' e desabilite o botão de upload. Ao receber os resultados (ou um erro) do Rust, esconda a mensagem e reative o botão."

Por que isso é vital: Sem isso, o usuário vai pensar que o aplicativo travou, vai clicar várias vezes e ter uma experiência ruim.

5. Tratamento de Erros

O que acontece se o usuário enviar um arquivo .JPG, um PDF corrompido ou um PDF que não é um RPI?

Instrução: "A função Rust deve ter tratamento de erros. Se ela não conseguir ler o PDF ou não encontrar nenhum registro de 'Deferimento do pedido', ela não deve travar. Ela deve retornar um erro ou um array vazio. O JavaScript deve saber como lidar com isso, mostrando uma mensagem clara para o usuário, como 'Erro ao ler o arquivo.' ou 'Nenhum registro encontrado neste PDF.'"

Por que isso é vital: Isso torna o aplicativo profissional e impede que ele "quebre" (crash) com uma entrada inesperada.