extern crate anyhow;
extern crate wasmtime;
extern crate reqwest;

use std::process::Command;
use std::str;
use std::ptr;
use std::collections::HashMap;
use anyhow::Result;
use wasmtime::*;

struct Wasm {
    store: Store,
    module: Module,
    instance: Instance,

    cacheget_uint8_memory0: Option<Memory>,
    cacheget_int32_memory0: Option<Memory>,
    wasm_vector_len: usize,
}

impl Wasm {
    fn new_frombytes(bytes: &[u8]) -> Wasm {
        let store = Store::default();
        let module = Module::new(store.engine(), bytes).unwrap();
        Wasm::init(store, module)
    }

    fn new_fromfile(filepath: &str) -> Wasm {
        let store = Store::default();
        let module = Module::from_file(store.engine(), filepath).unwrap();
        Wasm::init(store, module)
    }

    fn init(store: Store, module: Module) -> Wasm {
        let sh_func = Func::wrap(&store, |arg0: i32, arg1: i32, arg2: i32| {
            // TODO: WebAssembly 側のメモリを受け取れるようにする
            let ret = sh("ls");
            println!("{}", ret);
            Ok(())
        });
    
        let instance = Instance::new(&store, &module, &[sh_func.into()]).unwrap();
        
        Wasm {
            store: store,
            module: module,
            instance: instance,

            cacheget_uint8_memory0: None,
            cacheget_int32_memory0: None,
            wasm_vector_len: 0,
        }
    }

    fn entry_point(&mut self, arg: &str) -> String {
        let entry_point_func = self.instance
            .get_func("entry_point")
            .ok_or(anyhow::format_err!("failed to find `entry_point` function export")).unwrap()
            .get3::<i32, i32, i32, ()>().unwrap();
    
        let retptr = self.instance
            .get_global("__wbindgen_export_2")
            .ok_or(anyhow::format_err!("failed to find `__wbindgen_export_2` global export")).unwrap()
            .get()
            .unwrap_i32();
        let retptr = retptr - 16;
    
        let malloc = self.instance
            .get_func("__wbindgen_malloc")
            .ok_or(anyhow::format_err!("failed to find `__wbindgen_malloc` function export")).unwrap()
            .get1::<i32, i32>().unwrap();
    
        let realloc = self.instance
            .get_func("__wbindgen_realloc")
            .ok_or(anyhow::format_err!("failed to find `__wbindgen_realloc` function export")).unwrap()
            .get3::<i32, i32, i32, i32>().unwrap();

        let ptr0 = self.pass_string_to_wasm0(arg, malloc, realloc);
        let len0 = self.wasm_vector_len as i32;
        entry_point_func(retptr, ptr0, len0).unwrap();
        let r0 = unsafe { self.get_int32_memory0().data_unchecked()[(retptr / 4 + 0) as usize] };
        let r1 = unsafe { self.get_int32_memory0().data_unchecked()[(retptr / 4 + 1) as usize] };
        self.get_string_from_wasm0(r0 as i32, r1 as i32).to_string()
    }

    fn get_int32_memory0(&mut self) -> Memory {
        // TODO: メモリ周り正しくうごくように
        let cacheget_int32_memory0 = self.instance
            .get_memory("memory")
            .ok_or(anyhow::format_err!("failed to find `memory` memory export")).unwrap();

        let t = self.cacheget_int32_memory0.clone();

        match &t {
            None => {
                    self.cacheget_int32_memory0 = Some(cacheget_int32_memory0.clone());
                    cacheget_int32_memory0
                },
            Some(x) if x.data_ptr() != cacheget_int32_memory0.data_ptr() => {
                    self.cacheget_int32_memory0 = Some(cacheget_int32_memory0.clone());
                    cacheget_int32_memory0
                },
            Some(x) => x.clone(),
        }
    }

    fn get_uint8_memory0(&mut self) -> Memory {
        // TODO: メモリ周り正しくうごくように
        let cacheget_uint8_memory0 = self.instance
            .get_memory("memory")
            .ok_or(anyhow::format_err!("failed to find `memory` memory export")).unwrap();

        let t = self.cacheget_uint8_memory0.clone();

        match &t {
            None => {
                    self.cacheget_uint8_memory0 = Some(cacheget_uint8_memory0.clone());
                    cacheget_uint8_memory0
                },
            Some(x) if x.data_ptr() != cacheget_uint8_memory0.data_ptr() => {
                    self.cacheget_uint8_memory0 = Some(cacheget_uint8_memory0.clone());
                    cacheget_uint8_memory0
                },
            Some(x) => x.clone(),
        }
    }

    fn get_string_from_wasm0(&mut self, ptr: i32, len: i32) -> String {
        str::from_utf8(unsafe { &self.get_uint8_memory0().data_unchecked()[ptr as usize .. (ptr + len) as usize] }).unwrap().to_string()
    }

    fn pass_string_to_wasm0<F, G>(&mut self, arg: &str, malloc: F, realloc: G) -> i32
    where
        F: Fn(i32) -> Result<i32, Trap>,
        G: Fn(i32, i32, i32) -> Result<i32, Trap>,
    {
        let len = arg.len();
        let mut ptr = malloc(len as i32).unwrap();

        let mem = self.get_uint8_memory0();

        let mut offset: usize = 0;

        while offset < len {
            let code = arg.as_bytes()[offset];
            if code > 0x7F { break }
            unsafe { mem.data_unchecked_mut()[ptr as usize + offset] = code; }
            offset = offset + 1;
        }
    
        if offset != len {
            let mut t_arg = arg;
            if offset != 0 {
                t_arg = &arg[0..offset];
            }
            let arg = t_arg;
            ptr = realloc(ptr, len as i32, (offset + arg.len() * 3) as i32).unwrap();
            let len = offset + arg.len() * 3;
            let t_array = &mut self.get_uint8_memory0();
            let view = unsafe { &mut t_array.data_unchecked_mut()[(ptr + offset as i32) as usize .. (ptr + len as i32) as usize] };
            let ret = self.encode_string(arg, view);

            offset += ret.1;
        }
        
        self.wasm_vector_len = offset;
        ptr
    }

    fn encode_string(&self, arg: &str, view: &mut [u8]) -> (usize, usize) {
        let read = arg.len();
        let mut buf = String::from_utf8(arg.to_string().into_bytes()).unwrap();
        let written = buf.len();
        unsafe { ptr::copy_nonoverlapping::<u8>(view.as_mut_ptr(), buf.as_mut_ptr(), written) }
        (read, written)
    }
}

fn sh(s: &str) -> String {
    let mut iter = s.split_whitespace();
    let command = iter.next().unwrap();
    let args = iter.collect::<Vec<&str>>();

    let output = Command::new(command)
                        .args(args)
                        .output()
                        .expect(&format!("failed to start {}", command));

    format!("{}", String::from_utf8_lossy(&output.stdout))
}

fn main() {
    let resp = reqwest::blocking::get("http://localhost:5000/hello-world").unwrap();
    assert!(resp.status().is_success());

    // let filepath = "server/assets/hello-world/pkg/hello_world_bg.wasm";
    // let mut wasm = Wasm::new_fromfile(filepath);
    let mut wasm = Wasm::new_frombytes(&resp.bytes().unwrap());
    let result = wasm.entry_point("");
    println!("{}", result);
}

