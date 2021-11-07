#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/enzyme.rs"));

// TODO check where we should change the generated bindings and remove the mut. Apparently it's added everywhere (?), but enzyme handles quite a few args as const.


pub mod tree;
pub mod typeinfo;

use std::ffi::CString;
use std::ptr;

pub fn SafeEnzymeSetCLBool(ptr: *mut ::std::os::raw::c_void, val: bool) {
    let val: u8 = if val { 1 } else { 0 };
    unsafe {
        EnzymeSetCLBool(ptr, val);
    }
}

pub fn createEmptyTypeAnalysis() -> EnzymeTypeAnalysisRef {
    let platform: String = std::env::var("TARGET").unwrap();
    let tripple = CString::new(platform).unwrap().into_raw();
    unsafe {
      CreateTypeAnalysis(tripple, std::ptr::null_mut(), std::ptr::null_mut(), 0)
    }
}

pub struct AutoDiff {
    logic_ref: EnzymeLogicRef,
    type_analysis: EnzymeTypeAnalysisRef
}

impl AutoDiff {
    pub fn new(type_analysis: EnzymeTypeAnalysisRef) -> AutoDiff {
        
        let logic_ref = unsafe { CreateEnzymeLogic() };
        AutoDiff { logic_ref, type_analysis }
    }

    pub fn create_primal_and_gradient(&self, context: *mut LLVMOpaqueContext, fnc_todiff: LLVMValueRef, ret_type: CDIFFE_TYPE) -> LLVMValueRef {
        let tree_tmp = tree::TypeTree::from_type(CConcreteType::DT_Float, context)
            .prepend(0);

        let mut args_tree = vec![tree_tmp.inner];

        let mut args_activity = vec![CDIFFE_TYPE::DFT_OUT_DIFF];
        let mut args_uncachable = vec![0];

        let ret = tree::TypeTree::from_type(CConcreteType::DT_Float, context)
            .prepend(0);

        let kv_tmp = IntList {
            data: ptr::null_mut(),
            size: 0,
        };

        let mut known_values = vec![kv_tmp];

        let dummy_type = CFnTypeInfo {
            Arguments: args_tree.as_mut_ptr(),
            Return: ret.inner,
            KnownValues: known_values.as_mut_ptr(),
        };

        unsafe {
            EnzymeCreatePrimalAndGradient(
                self.logic_ref, // Logic
                fnc_todiff, ret_type, // LLVM function, return type
                args_activity.as_mut_ptr(), 1, // constant arguments
                self.type_analysis, // type analysis struct
                0, 0, CDerivativeMode::DEM_ReverseModeCombined, // return value, dret_used, top_level which was 1
                ptr::null_mut(), dummy_type, // additional_arg, type info (return + args)
                args_uncachable.as_mut_ptr(), 1, // unreachable arguments
                ptr::null_mut(), // write augmented function to this
                0, 1 // atomic_add, post_opt
            )
        }
    }
}

impl Drop for AutoDiff {
    fn drop(&mut self) {
        unsafe { FreeEnzymeLogic(self.logic_ref) }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use llvm_sys::core::LLVMModuleCreateWithName;
    use std::ffi::CString;

    #[test]
    fn empty_tree() {
        let _ = unsafe {
            EnzymeNewTypeTree()
        };
    }


    #[test]
    fn new_type_analysis() {
      let _ta = createEmptyTypeAnalysis();
    }

    #[test]
    fn new_autodiff() {
      let ta = createEmptyTypeAnalysis();
      let _ad = AutoDiff::new(ta);
    }

    #[test]
    fn get_LLVM_Module() {
        let _dummy_module = unsafe {
            LLVMModuleCreateWithName(CString::new("dummy").unwrap().into_raw())
        } as *mut LLVMOpaqueModule;
    }
    #[test]
    fn basic_autodiff() {
      2;
    }

    fn square(x: f32) -> f32 {
      x * x
    }
  
    /*
    #[test]
    fn dsquare() {
      let epsilon = 1e-3;
      let v1 = __enzyme_autodiff(square, 1.);
      let v2 = __enzyme_autodiff(square, 2.);
      let v3 = __enzyme_autodiff(square, 2.5);
      assert!(v1- 2. < epsilon);
      assert!(v1- 4. < epsilon);
      assert!(v1- 5. < epsilon);
    }
    */
}

pub mod utils;
