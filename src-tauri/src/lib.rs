use serde::{Deserialize, Serialize};
use std::fs;
use regex::Regex;

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

    processar_texto_pdf(texto_completo)
}

#[tauri::command]
async fn processar_pdf_bytes(bytes: Vec<u8>) -> Result<Vec<RegistroDeferido>, String> {
    // Extrair texto do PDF usando os bytes recebidos
    let texto_completo = match pdf_extract::extract_text_from_mem(&bytes) {
        Ok(texto) => texto,
        Err(e) => return Err(format!("Erro ao extrair texto do PDF: {}", e)),
    };

    processar_texto_pdf(texto_completo)
}

fn processar_texto_pdf(texto_completo: String) -> Result<Vec<RegistroDeferido>, String> {

    // Dividir o texto em blocos de registro usando números de processo (9 dígitos)
    let regex_processo = match Regex::new(r"(\d{9})") {
        Ok(regex) => regex,
        Err(e) => return Err(format!("Erro ao criar regex: {}", e)),
    };

    let mut registros = Vec::new();
    let posicoes: Vec<_> = regex_processo.find_iter(&texto_completo).collect();
    
    for i in 0..posicoes.len() {
        let inicio = posicoes[i].start();
        let fim = if i + 1 < posicoes.len() {
            posicoes[i + 1].start()
        } else {
            texto_completo.len()
        };
        let bloco = &texto_completo[inicio..fim];

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

        // Filtrar registros que possuem procurador
        if bloco_normalizado.contains("procurador:") {
            continue;
        }

        // Extrair dados do registro
        let processo = extrair_processo(bloco);
        let titular = extrair_titular(bloco);
        let ncl = extrair_ncl(bloco);
        let especificacao = extrair_especificacao(bloco);

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
        cap[1].trim().to_string()
    } else {
        String::new()
    }
}

fn extrair_ncl(texto: &str) -> String {
    let regex = Regex::new(r"(?i)ncl\(12\):\s*([^\n\r]+)").unwrap();
    if let Some(cap) = regex.captures(texto) {
        cap[1].trim().to_string()
    } else {
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
        
        resultado
    } else {
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
