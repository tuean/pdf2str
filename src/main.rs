use liteparse::{LiteParse, LiteParseConfig, OutputFormat};
use std::env;
use std::process;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("用法: pdf2str <文件路径> [选项]");
        eprintln!("选项:");
        eprintln!("  --ocr           启用 OCR（扫描版 PDF 必须）");
        eprintln!("  --pages <范围>  指定页码范围，如 1-5,10");
        eprintln!("  --lang <语言>   OCR 语言，默认 chi_sim+eng");
        process::exit(1);
    }

    let file_path = &args[1];

    let use_ocr = args.iter().any(|a| a == "--ocr");
    let target_pages = args.iter().position(|a| a == "--pages").map(|i| {
        args.get(i + 1).cloned().unwrap_or_else(|| {
            eprintln!("错误: --pages 需要指定页码范围");
            process::exit(1);
        })
    });
    let ocr_language = args
        .iter()
        .position(|a| a == "--lang")
        .map(|i| {
            args.get(i + 1).cloned().unwrap_or_else(|| {
                eprintln!("错误: --lang 需要指定语言代码");
                process::exit(1);
            })
        })
        .unwrap_or_else(|| "chi_sim+eng".to_string());

    let config = LiteParseConfig {
        ocr_enabled: use_ocr,
        target_pages,
        ocr_language,
        output_format: OutputFormat::Text,
        quiet: false,
        ..Default::default()
    };

    let parser = LiteParse::new(config);

    match parser.parse(file_path).await {
        Ok(result) => {
            println!("{}", result.text);
            eprintln!("\n--- 共解析 {} 页 ---", result.pages.len());
        }
        Err(e) => {
            eprintln!("解析失败: {}", e);
            process::exit(1);
        }
    }
}
