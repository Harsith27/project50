use clang_sys::*;
use std::ffi::{CStr, CString};

#[derive(Debug)]
pub struct FunctionSignature {
    pub return_type: String,
    pub params: Vec<String>,
    pub is_variadic: bool,
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub location: (String, u32, u32),
    pub signature: FunctionSignature,
}

fn func_from_cursor(cursor: CXCursor) -> Function {
    let name = unsafe { clang_getCString(clang_getCursorSpelling(cursor)) };
    let name = unsafe { CStr::from_ptr(name) }
        .to_str()
        .unwrap()
        .to_string();

    let mut params = Vec::new();
    for i in 0..unsafe { clang_Cursor_getNumArguments(cursor) } as u32 {
        let param = unsafe { clang_getCursorType(clang_Cursor_getArgument(cursor, i)) };
        let param = unsafe { clang_getCString(clang_getTypeSpelling(param)) };
        let param = unsafe { CStr::from_ptr(param) }
            .to_str()
            .unwrap()
            .to_string();
        params.push(param);
    }
    if params.is_empty() {
        params.push("void".to_string());
    }

    let return_type = unsafe { clang_getCursorResultType(cursor) };
    let return_type = unsafe { clang_getCString(clang_getTypeSpelling(return_type)) };
    let return_type = unsafe { CStr::from_ptr(return_type) }
        .to_str()
        .unwrap()
        .to_string();

    let location = unsafe { clang_getCursorLocation(cursor) };
    let mut file = std::ptr::null_mut();
    let mut line = 0;
    let mut col = 0;

    unsafe {
        clang_getSpellingLocation(
            location,
            &mut file,
            &mut line,
            &mut col,
            std::ptr::null_mut(),
        );
    }

    let file = unsafe { clang_getFileName(file) };
    let file = unsafe { CStr::from_ptr(clang_getCString(file)) }
        .to_str()
        .unwrap()
        .to_string();

    let variadic = unsafe { clang_Cursor_isVariadic(cursor) } != 0;

    Function {
        name,
        location: (file, line, col),
        signature: FunctionSignature {
            params,
            return_type,
            is_variadic: variadic,
        },
    }
}

extern "C" fn get_funcs(
    cursor: CXCursor,
    _parent: CXCursor,
    clien_data: CXClientData,
) -> CXChildVisitResult {
    let funcs = unsafe { &mut *(clien_data as *mut Vec<Function>) };
    let is_from_main = unsafe { clang_Location_isFromMainFile(clang_getCursorLocation(cursor)) };
    if is_from_main == 0 {
        return CXChildVisit_Continue;
    }

    let kind = unsafe { clang_getCursorKind(cursor) };
    if kind == CXCursor_FunctionDecl {
        let func = func_from_cursor(cursor);
        funcs.push(func);
    }
    return CXChildVisit_Continue;
}

pub fn parse_file(file_name: String) -> Option<Vec<Function>> {
    let src_file = CString::new(file_name).unwrap();
    let index = unsafe { clang_createIndex(0, 0) };
    let mut tu = std::ptr::null_mut();
    let err = unsafe {
        clang_parseTranslationUnit2(
            index,
            src_file.as_ptr(),
            std::ptr::null(),
            0,
            std::ptr::null_mut(),
            0,
            0,
            &mut tu,
        )
    };
    if err != CXError_Success {
        println!("Error: {}", err);
        return None;
    }

    let num_diags = unsafe { clang_getNumDiagnostics(tu) };
    if num_diags > 0 {
        println!("Diagnostics:");
        for i in 0..num_diags {
            let diag = unsafe { clang_getDiagnostic(tu, i) };
            let diag_obj_str = unsafe { clang_formatDiagnostic(diag, 0) };
            let diag_str = unsafe { clang_getCString(diag_obj_str) };
            println!("{:?}", diag_str);
            unsafe { clang_disposeString(diag_obj_str) };
        }
    } else {
        println!("No diagnostics");
    }

    let cursor = unsafe { clang_getTranslationUnitCursor(tu) };
    let mut funcs = Vec::new();

    unsafe {
        clang_visitChildren(
            cursor,
            get_funcs,
            &mut funcs as *mut Vec<Function> as *mut std::os::raw::c_void,
        );
    }

    Some(funcs)
}
