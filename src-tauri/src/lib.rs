use serde::{Deserialize, Serialize};
use std::fs;
use regex::Regex;
use std::fs::{File, OpenOptions};
use std::io::Write;

// Função para escrever em arquivo de log
fn log_to_file(message: &str) {
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open("../debug_log.txt") {
        let _ = writeln!(file, "{}", message);
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct RegistroDeferido {
    processo: String,
    titular: String,
    ncl: String,
    especificacao: String,
}

#[tauri::command]
async fn abrir_dialogo_arquivo(app_handle: tauri::AppHandle) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    
    let file_path = app_handle.dialog()
        .file()
        .add_filter("PDF", &["pdf"])
        .blocking_pick_file();
    
    match file_path {
        Some(path) => Ok(Some(path.to_string())),
        None => Ok(None),
    }
}

#[tauri::command]
async fn processar_pdf(caminho: String) -> Result<Vec<RegistroDeferido>, String> {
    // Ler o arquivo PDF
    let pdf_data = match fs::read(&caminho) {
        Ok(data) => data,
        Err(e) => return Err(format!("Erro ao ler arquivo: {}", e)),
    };

    // Extrair texto do PDF
    let texto_completo = match pdf_extract::extract_text_from_mem(&pdf_data) {
        Ok(texto) => texto,
        Err(e) => return Err(format!("Erro ao extrair texto do PDF: {}", e)),
    };

    // Log para debug - verificar texto extraído
    println!("=== TEXTO EXTRAÍDO DO PDF ===");
    println!("Tamanho total: {} caracteres", texto_completo.len());
    
    // Procurar especificamente pelo registro 934256276
    if texto_completo.contains("934256276") {
        if let Some(inicio) = texto_completo.find("934256276") {
            let texto_a_partir_do_inicio = &texto_completo[inicio..];
            let trecho: String = texto_a_partir_do_inicio.chars().take(1000).collect();
            println!("=== TRECHO DO REGISTRO 934256276 ===");
            println!("{}", trecho);
            println!("===================================");
        }
    }

    processar_texto_pdf(texto_completo)
}

#[tauri::command]
async fn processar_pdf_bytes(bytes: Vec<u8>) -> Result<Vec<RegistroDeferido>, String> {
    // Extrair texto do PDF usando os bytes recebidos
    let texto_completo = match pdf_extract::extract_text_from_mem(&bytes) {
        Ok(texto) => texto,
        Err(e) => return Err(format!("Erro ao extrair texto do PDF: {}", e)),
    };

    // Log para debug - verificar texto extraído
    println!("=== TEXTO EXTRAÍDO DO PDF (BYTES) ===");
    println!("Tamanho total: {} caracteres", texto_completo.len());
    
    // Procurar especificamente pelo registro 934256276
    if texto_completo.contains("934256276") {
        if let Some(inicio) = texto_completo.find("934256276") {
            let texto_a_partir_do_inicio = &texto_completo[inicio..];
            let trecho: String = texto_a_partir_do_inicio.chars().take(1000).collect();
            println!("=== TRECHO DO REGISTRO 934256276 (BYTES) ===");
            println!("{}", trecho);
            println!("==========================================");
        }
    }

    processar_texto_pdf(texto_completo)
}

fn processar_texto_pdf(texto_completo: String) -> Result<Vec<RegistroDeferido>, String> {

    // Limpar o arquivo de log no início do processamento
    let _ = File::create("../debug_log.txt");

    // Dividir o texto em blocos de registro usando números de processo (9 dígitos)
    let regex_processo = match Regex::new(r"(\d{9})") {
        Ok(regex) => regex,
        Err(e) => return Err(format!("Erro ao criar regex: {}", e)),
    };

    let mut registros = Vec::new();
    let mut posicoes: Vec<_> = regex_processo.find_iter(&texto_completo).map(|m| m.start()).collect();

    // Filtrar posições que não parecem ser inícios de registro válidos
    posicoes.retain(|&pos| {
        let trecho_seguinte: String = texto_completo[pos..]
            .chars()
            .take(200)
            .collect();
        trecho_seguinte.to_lowercase().contains("deferimento do pedido") || trecho_seguinte.to_lowercase().contains("indeferimento do pedido")
    });

    for i in 0..posicoes.len() {
        let inicio = posicoes[i];
        let fim = if i + 1 < posicoes.len() {
            posicoes[i + 1]
        } else {
            texto_completo.len()
        };

        let bloco = &texto_completo[inicio..fim];

        // Log para depurar cada bloco (removido para reduzir o tamanho do log)
        // log_to_file(&format!("[BLOCO SENDO PROCESSADO] {}", bloco.chars().take(500).collect::<String>()));

        // Verificar se o bloco contém "Deferimento do pedido"
        if !bloco.to_lowercase().contains("deferimento do pedido") {
            continue;
        }

        // Normalizar o texto do bloco para verificação de procurador
        let bloco_normalizado = bloco
            .to_lowercase()
            .replace('\n', " ")
            .replace('\r', " ")
            .replace("  ", " ")
            .trim()
            .to_string();

        // Debug temporário - verificar se contém procurador
        let contem_procurador_dois_pontos = bloco_normalizado.contains("procurador:");
        let contem_procurador_espaco = bloco_normalizado.contains("procurador ");
        let contem_procurador_quebra = bloco_normalizado.contains("procurador\n") || bloco_normalizado.contains("procurador\r");
        
        // Verificar também variações com maiúscula no texto original
        let contem_procurador_maiusculo = bloco.contains("Procurador:") || bloco.contains("Procurador ") || 
                                         bloco.contains("PROCURADOR:") || bloco.contains("PROCURADOR ");
        
        // Verificar múltiplas variações possíveis de procurador
        let tem_procurador = contem_procurador_dois_pontos || contem_procurador_espaco || contem_procurador_quebra || contem_procurador_maiusculo;

        // Log temporário para debug
        if bloco_normalizado.contains("procurador") {
            println!("=== PROCURADOR DETECTADO ===");
            let bloco_truncado: String = bloco_normalizado.chars().take(300).collect();
            println!("Bloco: {}", bloco_truncado);
            println!("Contém 'procurador:': {}", contem_procurador_dois_pontos);
            println!("Contém 'procurador ': {}", contem_procurador_espaco);
            println!("Contém quebra: {}", contem_procurador_quebra);
            println!("Será filtrado: {}", tem_procurador);
            println!("============================");
        }

        // Debug específico para registros 934256
        if bloco.contains("934256276") || bloco.contains("934256624") {
            log_to_file(&format!("=== REGISTRO {} ENCONTRADO ===", 
                if bloco.contains("934256276") { "934256276" } else { "934256624" }));
            
            // Mostrar o bloco completo
            log_to_file("=== BLOCO COMPLETO ===");
            log_to_file(bloco);
            log_to_file("=== FIM DO BLOCO ===");
            
            // Verificar se contém procurador de forma mais detalhada
             let bloco_lower = bloco.to_lowercase();
             log_to_file(&format!("=== ANÁLISE DE PROCURADOR ==="));
             log_to_file(&format!("Contém 'procurador' (minúsculo): {}", bloco_lower.contains("procurador")));
             log_to_file(&format!("Contém 'procurador:' (minúsculo): {}", bloco_lower.contains("procurador:")));
             log_to_file(&format!("Contém 'procurador ' (minúsculo): {}", bloco_lower.contains("procurador ")));
             log_to_file(&format!("Contém 'Procurador:' (maiúsculo): {}", bloco.contains("Procurador:")));
             log_to_file(&format!("Contém 'Procurador ' (maiúsculo): {}", bloco.contains("Procurador ")));
             log_to_file(&format!("Contém 'PROCURADOR:' (maiúsculo): {}", bloco.contains("PROCURADOR:")));
             log_to_file(&format!("Contém 'PROCURADOR ' (maiúsculo): {}", bloco.contains("PROCURADOR ")));
             
             // Buscar todas as ocorrências de "proc"
             let mut pos = 0;
             while let Some(found_pos) = bloco_lower[pos..].find("proc") {
                 let actual_pos = pos + found_pos;
                 let end_pos = std::cmp::min(actual_pos + 20, bloco_lower.len());
                 log_to_file(&format!("Encontrado 'proc' na posição {}: '{}'", actual_pos, &bloco_lower[actual_pos..end_pos]));
                 pos = actual_pos + 1;
             }
        }

        // Filtrar registros que possuem procurador
        if tem_procurador {
            continue;
        }

        // Extrair dados do registro
        let processo = extrair_processo(bloco);
        let titular = extrair_titular(bloco);
        let ncl = extrair_ncl(bloco);
        let especificacao = extrair_especificacao(bloco);

        // Log específico para debug de extração
        if bloco.contains("934256") {
            println!("=== DEBUG EXTRAÇÃO ===");
            println!("Processo extraído: '{}'", processo);
            println!("Titular extraído: '{}'", titular);
            println!("NCL extraído: '{}'", ncl);
            println!("Especificação extraída: '{}'", especificacao);
            
            // Mostrar trecho do bloco para análise
            let bloco_sample: String = bloco.chars().take(800).collect();
            println!("Amostra do bloco:");
            println!("{}", bloco_sample);
            println!("=====================");
        }

        // Só adicionar se conseguiu extrair pelo menos o processo
        if !processo.is_empty() {
            registros.push(RegistroDeferido {
                processo,
                titular,
                ncl,
                especificacao,
            });
        }
    }

    Ok(registros)
}

fn extrair_processo(texto: &str) -> String {
    let regex = Regex::new(r"(\d{9})").unwrap();
    if let Some(cap) = regex.find(texto) {
        cap.as_str().to_string()
    } else {
        String::new()
    }
}

fn extrair_titular(texto: &str) -> String {
    let regex = Regex::new(r"(?i)titular:\s*([^\n\r]+)").unwrap();
    if let Some(cap) = regex.captures(texto) {
        let resultado = cap[1].trim().to_string();
        if texto.contains("934256") {
            println!("[TITULAR] Encontrado: '{}'", resultado);
        }
        resultado
    } else {
        if texto.contains("934256") {
            println!("[TITULAR] Não encontrado com regex: (?i)titular:\\s*([^\\n\\r]+)");
            // Verificar se existe "titular" no texto
            if texto.to_lowercase().contains("titular") {
                println!("[TITULAR] Palavra 'titular' existe no texto");
            } else {
                println!("[TITULAR] Palavra 'titular' NÃO existe no texto");
            }
        }
        String::new()
    }
}

fn extrair_ncl(texto: &str) -> String {
    let regex = Regex::new(r"(?i)ncl\(12\):\s*([^\n\r]+)").unwrap();
    if let Some(cap) = regex.captures(texto) {
        let resultado = cap[1].trim().to_string();
        if texto.contains("934256") {
            println!("[NCL] Encontrado: '{}'", resultado);
        }
        resultado
    } else {
        if texto.contains("934256") {
            println!("[NCL] Não encontrado com regex: (?i)ncl\\(12\\):\\s*([^\\n\\r]+)");
            // Verificar variações de NCL
            if texto.to_lowercase().contains("ncl") {
                println!("[NCL] Palavra 'ncl' existe no texto");
            } else {
                println!("[NCL] Palavra 'ncl' NÃO existe no texto");
            }
        }
        String::new()
    }
}

fn extrair_especificacao(texto: &str) -> String {
    // Regex sem lookahead - captura tudo após "especificação:" até o final da linha ou próximo campo
    let regex = Regex::new(r"(?is)especificação:\s*(.+?)(?:\n|$)").unwrap();
    if let Some(cap) = regex.captures(texto) {
        let mut resultado = cap[1].trim().to_string();
        
        // Se o resultado contém um número de 9 dígitos, cortar antes dele
        if let Some(pos) = resultado.find(|c: char| c.is_ascii_digit()) {
            let parte_numerica = &resultado[pos..];
            if let Some(regex_num) = Regex::new(r"\d{9}").ok() {
                if let Some(match_num) = regex_num.find(parte_numerica) {
                    let pos_corte = pos + match_num.start();
                    resultado = resultado[..pos_corte].trim().to_string();
                }
            }
        }
        
        if texto.contains("934256") {
            println!("[ESPECIFICACAO] Encontrado: '{}'", resultado);
        }
        resultado
    } else {
        if texto.contains("934256") {
            println!("[ESPECIFICACAO] Não encontrado com regex: (?is)especificação:\\s*(.+?)(?:\\n|$)");
            // Verificar variações de especificação
            if texto.to_lowercase().contains("especificação") {
                println!("[ESPECIFICACAO] Palavra 'especificação' existe no texto");
            } else if texto.to_lowercase().contains("especificacao") {
                println!("[ESPECIFICACAO] Palavra 'especificacao' (sem acento) existe no texto");
            } else {
                println!("[ESPECIFICACAO] Palavra 'especificação' NÃO existe no texto");
            }
        }
        String::new()
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![abrir_dialogo_arquivo, processar_pdf, processar_pdf_bytes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
