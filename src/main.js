console.log('🔵 [DEBUG] ===== ARQUIVO MAIN.JS CARREGADO =====');
console.log('🔵 [DEBUG] Timestamp:', new Date().toISOString());

// Função para aguardar a API do Tauri estar disponível
async function aguardarTauriAPI() {
    console.log('🔵 [DEBUG] Aguardando API do Tauri...');
    
    let tentativas = 0;
    const maxTentativas = 50; // Máximo 5 segundos (50 * 100ms)
    
    // No Tauri v2, a API é fornecida nativamente via window.__TAURI_INTERNALS__
    while (!window.__TAURI_INTERNALS__ && tentativas < maxTentativas) {
        console.log(`🔵 [DEBUG] Aguardando API do Tauri... (tentativa ${tentativas + 1}/${maxTentativas})`);
        await new Promise(resolve => setTimeout(resolve, 100));
        tentativas++;
    }
    
    if (!window.__TAURI_INTERNALS__) {
        console.error('🔴 [DEBUG] TIMEOUT: API do Tauri não carregou após 5 segundos');
        console.log('🔵 [DEBUG] Verificando APIs disponíveis...');
        console.log('🔵 [DEBUG] window.__TAURI_INTERNALS__:', !!window.__TAURI_INTERNALS__);
        console.log('🔵 [DEBUG] window.__TAURI__:', !!window.__TAURI__);
        console.log('🔵 [DEBUG] Propriedades do window:', Object.keys(window).filter(k => k.includes('TAURI')));
        throw new Error('API do Tauri não disponível após timeout');
    }
    
    console.log('🔵 [DEBUG] API do Tauri carregada!', window.__TAURI_INTERNALS__);
    return window.__TAURI_INTERNALS__;
}

// Função principal
async function main() {
    console.log('🔵 [DEBUG] Iniciando aplicação...');
    
    try {
        // Aguarda a API do Tauri estar disponível
        const tauri = await aguardarTauriAPI();
        
        console.log('🔵 [DEBUG] Tauri API disponível:', tauri);
        console.log('🔵 [DEBUG] Propriedades disponíveis:', Object.keys(tauri));
        
        // --- Referências aos elementos HTML ---
        console.log('🔵 [DEBUG] Buscando elementos HTML...');
        const dropZone = document.getElementById('drop-zone');
        const fileInput = document.getElementById('file-input');
        const selectFileBtn = document.getElementById('select-file-btn');
        const feedback = document.getElementById('feedback');
        const feedbackText = document.getElementById('feedback-text');
        const containerResultados = document.getElementById('container-resultados');
        const resultsList = document.getElementById('results-list');
        const copyAllBtn = document.getElementById('copy-all-btn');
        const exportBtn = document.getElementById('export-btn');
        
        // --- Configuração dos Event Listeners ---
        function setupEventListeners() {
            console.log('🔵 [DEBUG] Configurando event listeners...');
            console.log('🔵 [DEBUG] selectFileBtn encontrado:', !!selectFileBtn);
            console.log('🔵 [DEBUG] selectFileBtn elemento:', selectFileBtn);
            
            if (selectFileBtn) {
                console.log('🔵 [DEBUG] Adicionando event listener ao botão');
                selectFileBtn.addEventListener('click', (event) => {
                    console.log('🔵 [DEBUG] Botão clicado! Event:', event);
                    alert('🔵 [DEBUG] Botão foi clicado! Verificando se o evento funciona...');
                    console.log('🔵 [DEBUG] Chamando selecionarEProcessarArquivo...');
                    selecionarEProcessarArquivo();
                });
                console.log('🔵 [DEBUG] Event listener adicionado com sucesso');
            } else {
                console.error('🔴 [DEBUG] Botão selectFileBtn NÃO encontrado!');
            }
            
            if (dropZone) {
                dropZone.addEventListener('dragover', handleDragOver);
                dropZone.addEventListener('dragleave', handleDragLeave);
                dropZone.addEventListener('drop', handleDrop);
            }
            if (fileInput) {
                fileInput.addEventListener('change', handleFileSelect);
            }
            if (copyAllBtn) {
                copyAllBtn.addEventListener('click', copyAllResults);
            }
            if (exportBtn) {
                exportBtn.addEventListener('click', exportResults);
            }
            document.addEventListener('dragover', e => e.preventDefault());
            document.addEventListener('drop', e => e.preventDefault());
        }
        
        // Configura os event listeners (agora as variáveis já estão declaradas)
        setupEventListeners();
        
        console.log('🔵 [DEBUG] Elementos encontrados:', {
            dropZone: !!dropZone,
            fileInput: !!fileInput,
            selectFileBtn: !!selectFileBtn,
            feedback: !!feedback,
            feedbackText: !!feedbackText,
            containerResultados: !!containerResultados,
            resultsList: !!resultsList,
            copyAllBtn: !!copyAllBtn,
            exportBtn: !!exportBtn
        });
        
        if (!selectFileBtn) {
            console.error('🔴 [DEBUG] CRÍTICO: Botão select-file-btn não encontrado!');
            console.log('🔵 [DEBUG] Todos os elementos com ID:', 
                Array.from(document.querySelectorAll('[id]')).map(el => el.id)
            );
        }
        
        // --- Funções de Feedback e UI (dentro do escopo) ---
        
        function showFeedback(message, isError = false) {
            feedbackText.textContent = message;
            feedback.classList.remove('hidden');
            if (isError) {
                feedback.classList.add('error');
            } else {
                feedback.classList.remove('error');
            }
        }

        function hideFeedback() {
            feedback.classList.add('hidden');
        }

        function displayResults(results) {
            currentResults = results;
            currentPage = 1; // Reset para primeira página
            renderCurrentPage();
        }

        function renderCurrentPage() {
            resultsList.innerHTML = ''; // Limpa resultados anteriores

            if (currentResults && currentResults.length > 0) {
                // Calcula índices da página atual
                const startIndex = (currentPage - 1) * itemsPerPage;
                const endIndex = startIndex + itemsPerPage;
                const currentPageResults = currentResults.slice(startIndex, endIndex);

                // Renderiza os registros da página atual
                currentPageResults.forEach((reg, index) => {
                    const globalIndex = startIndex + index + 1; // Numeração global
                    const li = document.createElement('li');
                    li.innerHTML = `
                        <div class="result-item">
                            <div class="registro-numero">
                                <span class="numero-registro">#${globalIndex}</span>
                            </div>
                            <div class="registro-info">
                                <strong>Processo:</strong> ${reg.processo || 'N/A'}<br>
                                <strong>Titular:</strong> ${reg.titular || 'N/A'}<br>
                                <strong>NCL:</strong> ${reg.ncl || 'N/A'}<br>
                                <strong>Especificação:</strong> ${reg.especificacao || 'N/A'}
                            </div>
                            <button class="copy-btn" onclick="copyToClipboard('Processo: ${reg.processo}\\nTitular: ${reg.titular}\\nNCL: ${reg.ncl}\\nEspecificação: ${reg.especificacao}')">📋</button>
                        </div>
                    `;
                    resultsList.appendChild(li);
                });

                // Renderiza controles de paginação
                renderPaginationControls();
                containerResultados.classList.remove('hidden');
            } else {
                containerResultados.classList.add('hidden');
            }
        }

        function renderPaginationControls() {
            const totalPages = Math.ceil(currentResults.length / itemsPerPage);
            
            // Remove controles existentes
            const existingPagination = document.querySelector('.pagination-controls');
            if (existingPagination) {
                existingPagination.remove();
            }

            // Só mostra paginação se houver mais de uma página
            if (totalPages > 1) {
                const paginationDiv = document.createElement('div');
                paginationDiv.className = 'pagination-controls';
                
                paginationDiv.innerHTML = `
                    <div class="pagination-info">
                        <span>Página ${currentPage} de ${totalPages} (${currentResults.length} registros)</span>
                    </div>
                    <div class="pagination-buttons">
                        <button class="pagination-btn" onclick="goToPage(${currentPage - 1})" ${currentPage === 1 ? 'disabled' : ''}>
                            ← Anterior
                        </button>
                        <button class="pagination-btn" onclick="goToPage(${currentPage + 1})" ${currentPage === totalPages ? 'disabled' : ''}>
                            Próximo →
                        </button>
                    </div>
                `;
                
                containerResultados.appendChild(paginationDiv);
            }
        }

        function goToPage(page) {
            const totalPages = Math.ceil(currentResults.length / itemsPerPage);
            if (page >= 1 && page <= totalPages) {
                currentPage = page;
                renderCurrentPage();
            }
        }

        function copyToClipboard(texto) {
            navigator.clipboard.writeText(texto).then(() => {
                showFeedback('Copiado para a área de transferência!');
                setTimeout(hideFeedback, 2000);
            }).catch(err => {
                console.error('Erro ao copiar:', err);
                showFeedback('Erro ao copiar para a área de transferência', true);
                setTimeout(hideFeedback, 3000);
            });
        }

        function copyAllResults() {
            if (!currentResults || currentResults.length === 0) {
                showFeedback('Nenhum resultado para copiar', true);
                setTimeout(hideFeedback, 3000);
                return;
            }

            const texto = currentResults.map(reg => 
                `Processo: ${reg.processo}\nTitular: ${reg.titular}\nNCL: ${reg.ncl}\nEspecificação: ${reg.especificacao}\n---`
            ).join('\n');

            copyToClipboard(texto);
        }

        function exportResults() {
            if (!currentResults || currentResults.length === 0) {
                showFeedback('Nenhum resultado para exportar', true);
                setTimeout(hideFeedback, 3000);
                return;
            }

            const csvContent = 'Processo,Titular,NCL,Especificação\n' + 
                currentResults.map(reg => 
                    `"${reg.processo}","${reg.titular}","${reg.ncl}","${reg.especificacao}"`
                ).join('\n');

            const blob = new Blob([csvContent], { type: 'text/csv;charset=utf-8;' });
            const link = document.createElement('a');
            const url = URL.createObjectURL(blob);
            link.setAttribute('href', url);
            link.setAttribute('download', 'registros_rpi.csv');
            link.style.visibility = 'hidden';
            document.body.appendChild(link);
            link.click();
            document.body.removeChild(link);

            showFeedback('Arquivo CSV exportado com sucesso!');
            setTimeout(hideFeedback, 3000);
        }

        function handleDragOver(e) {
            e.preventDefault();
            dropZone.classList.add('drag-over');
        }

        function handleDragLeave(e) {
            e.preventDefault();
            dropZone.classList.remove('drag-over');
        }

        function handleDrop(e) {
            e.preventDefault();
            dropZone.classList.remove('drag-over');
            const files = e.dataTransfer.files;
            if (files.length > 0) {
                processarArquivo(files[0]);
            }
        }

        function handleFileSelect(e) {
            const file = e.target.files[0];
            if (file) {
                processarArquivo(file);
            }
        }
        
        // --- Função Principal de Seleção de Arquivo ---
        
        async function selecionarEProcessarArquivo() {
            console.log('🔵 [DEBUG] === INÍCIO selecionarEProcessarArquivo ===');
            console.log('🔵 [DEBUG] window.__TAURI_INTERNALS__ disponível:', !!window.__TAURI_INTERNALS__);
            console.log('🔵 [DEBUG] window.__TAURI_INTERNALS__.invoke disponível:', !!(window.__TAURI_INTERNALS__ && window.__TAURI_INTERNALS__.invoke));
            
            try {
                console.log('🔵 [DEBUG] Mostrando feedback...');
                showFeedback('Abrindo seletor de arquivos...');
                
                console.log('🔵 [DEBUG] Chamando comando abrir_dialogo_arquivo...');
                // Usar o comando backend que já está implementado
                const selected = await window.__TAURI_INTERNALS__.invoke('abrir_dialogo_arquivo');
                
                console.log('🔵 [DEBUG] Resultado do diálogo:', selected);
                console.log('🔵 [DEBUG] Tipo do resultado:', typeof selected);

                if (selected) {
                    console.log('🔵 [DEBUG] Arquivo selecionado, processando...');
                    showFeedback('Processando PDF... aguarde.');
                    
                    console.log('🔵 [DEBUG] Chamando comando processar_pdf...');
                    const result = await window.__TAURI_INTERNALS__.invoke('processar_pdf', {
                        caminho: selected
                    });
                    
                    console.log('🔵 [DEBUG] Resultado do processamento:', result);
                    displayResults(result);
                } else {
                    console.log('🔵 [DEBUG] Nenhum arquivo selecionado (usuário cancelou)');
                }
            } catch (error) {
                console.error('🔴 [DEBUG] ERRO na função:', error);
                console.error('🔴 [DEBUG] Stack trace:', error.stack);
                console.error('🔴 [DEBUG] Tipo do erro:', typeof error);
                console.error('🔴 [DEBUG] Propriedades do erro:', Object.keys(error));
                showFeedback(`Erro: ${error}`, true);
            } finally {
                console.log('🔵 [DEBUG] Finalizando função...');
                hideFeedback();
            }
            
            console.log('🔵 [DEBUG] === FIM selecionarEProcessarArquivo ===');
        }

        async function handleFile(file) {
            if (file.type !== 'application/pdf') {
                showFeedback('Erro: Apenas arquivos PDF são permitidos.', true);
                return;
            }

            showFeedback('Processando PDF... aguarde.');

            try {
                const fileBytes = await file.arrayBuffer();
                const result = await window.__TAURI_INTERNALS__.invoke('processar_pdf_bytes', {
                    bytes: Array.from(new Uint8Array(fileBytes))
                });
                displayResults(result);
            } catch (error) {
                console.error('Erro ao processar arquivo:', error);
                showFeedback(`Erro: ${error}`, true);
            } finally {
                hideFeedback();
            }
        }
        
        // Variável global para resultados
        let currentResults = [];
        
        // Variáveis de paginação
        let currentPage = 1;
        const itemsPerPage = 10;
        
        // Torna as funções globais para serem acessíveis pelos event listeners
        window.showFeedback = showFeedback;
        window.hideFeedback = hideFeedback;
        window.displayResults = displayResults;
        window.copyToClipboard = copyToClipboard;
        window.copyAllResults = copyAllResults;
        window.exportResults = exportResults;
        window.handleDragOver = handleDragOver;
        window.handleDragLeave = handleDragLeave;
        window.handleDrop = handleDrop;
        window.handleFileSelect = handleFileSelect;
        window.selecionarEProcessarArquivo = selecionarEProcessarArquivo;
        window.handleFile = handleFile;
        window.goToPage = goToPage;
        
    } catch (error) {
        console.error('🔴 [DEBUG] Erro ao inicializar aplicação:', error);
    }
}

// Inicia a aplicação
main();