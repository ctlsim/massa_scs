use wasm_bindgen::prelude::wasm_bindgen;

use serde::{Deserialize, Serialize};
// use wasm_bindgen::JsValue;
use wasmparser::{Encoding, ExternalKind, Parser, Payload, TypeRef};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum WasmPayload {
    Version(WasmPayloadVersion),
    Import(WasmPayloadImport),
    Export(WasmPayloadExport),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WasmPayloadVersion {
    /// The version number found in the header.
    num: u16,
    /// The encoding format is a WebAssembly module if true else a WebAssembly component
    is_module: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
enum WasmImportKind {
    Func,
    Table,
    Memory,
    Global,
    Tag,
}

impl From<&TypeRef> for WasmImportKind {
    fn from(value: &TypeRef) -> Self {
        match value {
            TypeRef::Func(_) => WasmImportKind::Func,
            TypeRef::Table(_) => WasmImportKind::Table,
            TypeRef::Memory(_) => WasmImportKind::Memory,
            TypeRef::Global(_) => WasmImportKind::Global,
            TypeRef::Tag(_) => WasmImportKind::Tag,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WasmPayloadImport {
    module: String,
    name: String,
    kind: WasmImportKind,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
enum WasmExportKind {
    Func,
    Table,
    Memory,
    Global,
    Tag,
}

impl From<&ExternalKind> for WasmExportKind {
    fn from(value: &ExternalKind) -> Self {
        match value {
            ExternalKind::Func => WasmExportKind::Func,
            ExternalKind::Table => WasmExportKind::Table,
            ExternalKind::Memory => WasmExportKind::Memory,
            ExternalKind::Global => WasmExportKind::Global,
            ExternalKind::Tag => WasmExportKind::Tag,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WasmPayloadExport {
    name: String,
    kind: WasmExportKind,
    index: u32,
}

#[wasm_bindgen]
pub fn read_wasm(bytecode: &[u8]) -> String {
    let mut payloads = vec![];

    for payload in Parser::new(0).parse_all(bytecode) {
        match payload.unwrap() {
            Payload::Version { num, encoding, .. } => {
                payloads.push(WasmPayload::Version(WasmPayloadVersion {
                    num: num,
                    is_module: if let Encoding::Module = encoding {
                        true
                    } else {
                        false
                    },
                }))
            }
            Payload::ExportSection(s) => {
                for export_ in s {
                    if let Ok(export) = export_ {
                        payloads.push(WasmPayload::Export(WasmPayloadExport {
                            name: export.name.to_string(),
                            kind: WasmExportKind::from(&export.kind),
                            index: export.index,
                        }));
                    }
                }
            }
            Payload::ImportSection(s) => {
                for import_ in s {
                    if let Ok(import) = import_ {
                        payloads.push(WasmPayload::Import(WasmPayloadImport {
                            module: import.module.to_string(),
                            name: import.name.to_string(),
                            kind: WasmImportKind::from(&import.ty),
                        }))
                    }
                }
            }
            _other => {
                // println!("found payload {:?}", _other);
            }
        }
    }

    serde_json::to_string(&payloads).unwrap_or_default()
}

/*
// Called when the wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_inner_html("Hello from Rust!");

    body.append_child(&val)?;

    Ok(())
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::{console_log, wasm_bindgen_test};

    fn find_versions(payloads: &Vec<WasmPayload>) -> Vec<WasmPayloadVersion> {
        payloads
            .iter()
            .filter_map(|wp| {
                if let WasmPayload::Version(wpv) = wp {
                    Some(wpv.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    fn find_imports_sorted(payloads: &Vec<WasmPayload>) -> Vec<WasmPayloadImport> {
        let mut res: Vec<WasmPayloadImport> = payloads
            .iter()
            .filter_map(|wp| {
                if let WasmPayload::Import(wpi) = wp {
                    Some(wpi.clone())
                } else {
                    None
                }
            })
            .collect();
        res.sort_by_key(|k| k.module.clone() + k.name.clone().as_str());
        res
    }

    fn find_exports_sorted(payloads: &Vec<WasmPayload>) -> Vec<WasmPayloadExport> {
        let mut res: Vec<WasmPayloadExport> = payloads
            .iter()
            .filter_map(|wp| {
                if let WasmPayload::Export(wpe) = wp {
                    Some(wpe.clone())
                } else {
                    None
                }
            })
            .collect();
        res.sort_by_key(|k| k.index.to_string() + k.name.clone().as_str());
        res
    }

    #[wasm_bindgen_test]
    fn wasm_parse_wasm_test_01() {
        let bytecode = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tools/wasm_test_01/build/wasm_test_01.wasm"
        ));

        console_log!("wasm_test_01");

        let res = read_wasm(&bytecode[..]);
        // console_log!("res len: {}", res.len());
        // console_log!("res: {}", res);

        let payloads: Vec<WasmPayload> = serde_json::from_str(&res).unwrap();

        let wasm_versions = crate::parser::tests::find_versions(&payloads);
        assert_eq!(wasm_versions.len(), 1);
        assert_eq!(wasm_versions[0].num, 1);
        assert!(wasm_versions[0].is_module);

        let wasm_imports = find_imports_sorted(&payloads);
        // console_log!("wasm imports: {:?}", wasm_imports);

        assert_eq!(wasm_imports.len(), 7);
        assert_eq!(wasm_imports[0].module, "env");
        assert_eq!(wasm_imports[0].name, "abort");
        assert_eq!(wasm_imports[0].kind, WasmImportKind::Func);

        assert_eq!(wasm_imports[1].module, "massa");
        assert_eq!(wasm_imports[1].name, "assembly_script_caller_has_write_access");
        assert_eq!(wasm_imports[1].kind, WasmImportKind::Func);
        assert_eq!(wasm_imports[2].module, "massa");
        assert_eq!(wasm_imports[2].name, "assembly_script_generate_event");
        assert_eq!(wasm_imports[2].kind, WasmImportKind::Func);
        assert_eq!(wasm_imports[3].module, "massa");
        assert_eq!(wasm_imports[3].name, "assembly_script_get_call_stack");
        assert_eq!(wasm_imports[3].kind, WasmImportKind::Func);
        assert_eq!(wasm_imports[4].module, "massa");
        assert_eq!(wasm_imports[4].name, "assembly_script_get_data");
        assert_eq!(wasm_imports[4].kind, WasmImportKind::Func);

        let wasm_exports = find_exports_sorted(&payloads);
        // console_log!("wasm exports: {:?}", wasm_exports);
        assert_eq!(wasm_exports.len(), 8);
        assert_eq!(wasm_exports[0].name, "memory");
        assert_eq!(wasm_exports[1].name, "__pin");
        assert_eq!(wasm_exports[2].name, "__unpin");
        assert_eq!(wasm_exports[3].name, "__collect");
        assert_eq!(wasm_exports[4].name, "main");
        assert_eq!(wasm_exports[5].name, "constructor");
        assert_eq!(wasm_exports[6].name, "__rtti_base");
        assert_eq!(wasm_exports[7].name, "__new");


    }

    #[wasm_bindgen_test]
    fn wasm_parse_wasm_test_02() {
        let bytecode = include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tools/wasm_test_02/pkg/wasm_test_02_bg.wasm"
        ));

        let res = read_wasm(&bytecode[..]);
        // console_log!("res len: {}", res.len());
        // console_log!("res: {}", res);

        let payloads: Vec<WasmPayload> = serde_json::from_str(&res).unwrap();

        let wasm_versions = crate::parser::tests::find_versions(&payloads);
        assert_eq!(wasm_versions.len(), 1);
        assert_eq!(wasm_versions[0].num, 1);
        assert!(wasm_versions[0].is_module);

        let wasm_imports = find_imports_sorted(&payloads);

        assert_eq!(wasm_imports.len(), 1);
        assert_eq!(wasm_imports[0].module, "./wasm_test_02_bg.js");
        assert!(wasm_imports[0].name.starts_with("__wbg_alert_"));
        assert_eq!(wasm_imports[0].kind, WasmImportKind::Func);

        let wasm_exports = find_exports_sorted(&payloads);
        // console_log!("wasm exports: {:?}", wasm_exports);
        assert_eq!(wasm_exports.len(), 5);
        assert_eq!(wasm_exports[0].name, "memory");
        assert_eq!(wasm_exports[1].kind, WasmExportKind::Func);
        assert_eq!(wasm_exports[1].name, "greet");
        assert_eq!(wasm_exports[2].name, "data");
        assert_eq!(wasm_exports[2].kind, WasmExportKind::Global);
    }
}
