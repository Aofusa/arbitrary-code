extern crate anyhow;
extern crate wasmtime;
extern crate reqwest;

use std::process::Command;
use std::str;
use std::ptr;
use anyhow::Result;
use wasmtime::*;

struct Wasm {
    // store: Store,
    // module: Module,
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
        let callback_type = FuncType::new(
            [ValType::I32, ValType::I32, ValType::I32].iter().cloned(),
            [].iter().cloned(),
        );
        let callback_func = Func::new(&store, callback_type, |caller: Caller<'_>, args, _results| {
            let memory = match caller.get_export("memory") {
                Some(Extern::Memory(mem)) => mem,
                _ => return Err(Trap::new("failed to find host memory")),
            };
            let malloc = match caller.get_export("__wbindgen_malloc") {
                Some(Extern::Func(mem)) => mem.get1::<i32, i32>().unwrap(),
                _ => return Err(Trap::new("failed to find host malloc")),
            };
            let realloc = match caller.get_export("__wbindgen_realloc") {
                Some(Extern::Func(mem)) => mem.get3::<i32, i32, i32, i32>().unwrap(),
                _ => return Err(Trap::new("failed to find host realloc")),
            };

            let arg0 = args[0].unwrap_i32();
            let arg1 = args[1].unwrap_i32();
            let arg2 = args[2].unwrap_i32();

            // 引数の文字列を取得する
            let response = unsafe {
                let data = memory.data_unchecked()
                    .get(arg1 as u32 as usize..)
                    .and_then(|arr| arr.get(..arg2 as u32 as usize));
                let string = match data {
                    Some(data) => match str::from_utf8(data) {
                        Ok(s) => s,
                        Err(_) => return Err(Trap::new("invalid utf-8")),
                    },
                    None => return Err(Trap::new("pointer/length out of bounds")),
                };
                sh(string)
            };
    
            // レスポンスをメモリに書き込む
            let (ptr0, len0) = unsafe {
                let len = response.len();
                let mut ptr = malloc(len as i32).unwrap();

                let mem = {
                    let cacheget_uint8_memory0 = match caller.get_export("memory") {
                        Some(Extern::Memory(mem)) => mem,
                        _ => return Err(Trap::new("failed to find host memory")),
                    };
        
                    let t = memory.clone();
            
                    if t.data_ptr() != cacheget_uint8_memory0.data_ptr() {
                        cacheget_uint8_memory0
                    } else {
                        t
                    }
                };

                let mut offset: usize = 0;

                while offset < len {
                    let code = response.as_bytes()[offset];
                    if code > 0x7F { break }
                    mem.data_unchecked_mut()[ptr as usize + offset] = code;
                    offset = offset + 1;
                }

                if offset != len {
                    let mut t_arg = response.as_str();
                    if offset != 0 {
                        t_arg = &response[0..offset];
                    }
                    let arg = t_arg;
                    ptr = realloc(ptr, len as i32, (offset + arg.len() * 3) as i32).unwrap();
                    let len = offset + arg.len() * 3;

                    let t_array = {
                        let cacheget_uint8_memory0 = match caller.get_export("memory") {
                            Some(Extern::Memory(mem)) => mem,
                            _ => return Err(Trap::new("failed to find host memory")),
                        };
            
                        let t = mem.clone();
                
                        if t.data_ptr() != cacheget_uint8_memory0.data_ptr() {
                            cacheget_uint8_memory0
                        } else {
                            t
                        }
                    };

                    let view = &mut t_array.data_unchecked_mut()[(ptr + offset as i32) as usize .. (ptr + len as i32) as usize];
                    let ret = {
                        let read = arg.len();
                        let mut buf = String::from_utf8(arg.to_string().into_bytes()).unwrap();
                        let written = buf.len();
                        ptr::copy_nonoverlapping::<u8>(buf.as_mut_ptr(), view.as_mut_ptr(), written);
                        (read, written)
                    };

                    offset += ret.1;
                }

                (ptr, offset)
            };

            unsafe {
                let cacheget_int32_memory0 = match caller.get_export("memory") {
                    Some(Extern::Memory(mem)) => mem,
                    _ => return Err(Trap::new("failed to find host memory")),
                };
    
                let t = memory.clone();
        
                let int32_memory = if t.data_ptr() != cacheget_int32_memory0.data_ptr() {
                    cacheget_int32_memory0
                } else {
                    t
                };

                int32_memory.data_unchecked_mut()[(arg0 + 0 + 1 * 4) as usize] = len0.to_le_bytes()[0];
                int32_memory.data_unchecked_mut()[(arg0 + 1 + 1 * 4) as usize] = len0.to_le_bytes()[1];
                int32_memory.data_unchecked_mut()[(arg0 + 2 + 1 * 4) as usize] = len0.to_le_bytes()[2];
                int32_memory.data_unchecked_mut()[(arg0 + 3 + 1 * 4) as usize] = len0.to_le_bytes()[3];
            }

            unsafe {
                let cacheget_int32_memory0 = match caller.get_export("memory") {
                    Some(Extern::Memory(mem)) => mem,
                    _ => return Err(Trap::new("failed to find host memory")),
                };
    
                let t = memory.clone();
        
                let int32_memory = if t.data_ptr() != cacheget_int32_memory0.data_ptr() {
                    cacheget_int32_memory0
                } else {
                    t
                };

                int32_memory.data_unchecked_mut()[(arg0 + 0) as usize] = ptr0.to_le_bytes()[0];
                int32_memory.data_unchecked_mut()[(arg0 + 1) as usize] = ptr0.to_le_bytes()[1];
                int32_memory.data_unchecked_mut()[(arg0 + 2) as usize] = ptr0.to_le_bytes()[2];
                int32_memory.data_unchecked_mut()[(arg0 + 3) as usize] = ptr0.to_le_bytes()[3];
            }

            Ok(())
        });
    
        let instance = Instance::new(&store, &module, &[callback_func.into()]).unwrap();
        
        Wasm {
            // store: store,
            // module: module,
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
    
        let export_2 = self.instance
            .get_global("__wbindgen_export_2")
            .ok_or(anyhow::format_err!("failed to find `__wbindgen_export_2` global export")).unwrap();
        let retptr = export_2.get().unwrap_i32() - 16;
        export_2.set(Val::I32(retptr)).unwrap();
        
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
        
        let r0 = unsafe {
            i32::from_le_bytes([
                self.get_int32_memory0().data_unchecked()[(retptr + 0) as usize],
                self.get_int32_memory0().data_unchecked()[(retptr + 1) as usize],
                self.get_int32_memory0().data_unchecked()[(retptr + 2) as usize],
                self.get_int32_memory0().data_unchecked()[(retptr + 3) as usize],
            ])
        };
        let r1 = unsafe {
            i32::from_le_bytes([
                self.get_int32_memory0().data_unchecked()[(retptr + 0 + 4) as usize],
                self.get_int32_memory0().data_unchecked()[(retptr + 1 + 4) as usize],
                self.get_int32_memory0().data_unchecked()[(retptr + 2 + 4) as usize],
                self.get_int32_memory0().data_unchecked()[(retptr + 3 + 4) as usize],
            ])
        };

        self.get_string_from_wasm0(r0 as i32, r1 as i32).to_string()
    }

    fn get_int32_memory0(&mut self) -> Memory {
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
        unsafe { ptr::copy_nonoverlapping::<u8>(buf.as_mut_ptr(), view.as_mut_ptr(), written) }
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
    let mut wasm = Wasm::new_frombytes(&resp.bytes().unwrap());

    // let filepath = "server/assets/hello-world/pkg/hello_world_bg.wasm";
    // let mut wasm = Wasm::new_fromfile(filepath);
    
    let result = wasm.entry_point("abc");
    println!("{}", result);
}

