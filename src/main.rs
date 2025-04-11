use enigo::{Enigo, Key, KeyboardControllable};
use scrap::{Capturer, Display};
use std::{thread, time::Duration};
use std::io::ErrorKind::WouldBlock;
use std::time::Instant;

fn main() {
    let display = Display::primary().expect("Erro ao acessar display principal");
    let mut capturer = Capturer::new(display).expect("Erro ao iniciar captura de tela");
    let (w, h) = (capturer.width(), capturer.height());
    println!("Resolução detectada: {}x{}", w, h);

    let mut enigo = Enigo::new();
    let mut ultimo_disparo = Instant::now();

    // Definir constantes para a área de busca
    const X_START: usize = 465;
    const X_END: usize = 943;
    const Y_START: usize = 391;
    const Y_END: usize = 416;
    const AREA_WIDTH: usize = X_END - X_START;
    const AREA_HEIGHT: usize = Y_END - Y_START;

    loop {
        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(ref e) if e.kind() == WouldBlock => continue,
            Err(e) => panic!("Erro na captura de tela: {}", e),
        };

        let stride = buffer.len() / h;
        let mut acertou = false;

        let start_offset = Y_START * stride + X_START * 4;
        let end_offset = Y_END * stride + X_END * 4;
        
        if end_offset >= buffer.len() {
            continue;
        }

        for y in 0..AREA_HEIGHT {
            let y_offset = (Y_START + y) * stride;
            for x in 0..AREA_WIDTH {
                let i = y_offset + (X_START + x) * 4;
                
                // Processar pixel
                let b = buffer[i];
                let g = buffer[i + 1];
                let r = buffer[i + 2];

                if cor_igual((r, g, b), (166, 160, 155), 15) {
                    for dy in 0..=5 {
                        for dx in 0..=5 {
                            let nx = (X_START + x + dx).min(X_END - 1);
                            let ny = (Y_START + y + dy).min(Y_END - 1);
                            let ni = ny * stride + nx * 4;
                            
                            if ni + 3 >= buffer.len() {
                                continue;
                            }
                            
                            let nb = buffer[ni];
                            let ng = buffer[ni + 1];
                            let nr = buffer[ni + 2];
                            
                            if nr >= 30 && nr <= 80 && ng >= 150 && ng <= 220 && nb >= 80 && nb <= 130 {
                                acertou = true;
                                break;
                            }
                        }
                        if acertou { break; }
                    }
                }
                
                if acertou { break; }
            }
            if acertou { break; }
        }

        if acertou && ultimo_disparo.elapsed().as_millis() > 200 {
            enigo.key_click(Key::Layout('f'));
            ultimo_disparo = Instant::now();
        }

        // Aumentar ligeiramente o sleep para reduzir CPU sem perder desempenho
        thread::sleep(Duration::from_micros(500));
    }
}

#[inline(always)]
fn cor_igual((r, g, b): (u8, u8, u8), alvo: (u8, u8, u8), tolerancia: u8) -> bool {
    (r as i16 - alvo.0 as i16).abs() < tolerancia as i16 &&
    (g as i16 - alvo.1 as i16).abs() < tolerancia as i16 &&
    (b as i16 - alvo.2 as i16).abs() < tolerancia as i16
}
