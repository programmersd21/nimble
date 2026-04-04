use cranelift::prelude::*;
use cranelift_jit::{JITBuilder, JITModule};
use cranelift_module::{Linkage, Module};

pub struct JIT {
    module: JITModule,
    ctx: codegen::Context,
}

impl JIT {
    pub fn new() -> Self {
        let builder = JITBuilder::new(cranelift_module::default_libcall_names()).unwrap();
        // Register builtins here if needed
        let module = JITModule::new(builder);
        Self {
            module,
            ctx: codegen::Context::new(),
        }
    }

    pub fn compile(
        &mut self,
        name: &str,
        ir_builder: impl FnOnce(&mut FunctionBuilder),
    ) -> *const u8 {
        let mut builder_context = FunctionBuilderContext::new();
        let mut builder = FunctionBuilder::new(&mut self.ctx.func, &mut builder_context);
        ir_builder(&mut builder);
        builder.finalize();

        let id = self
            .module
            .declare_function(name, Linkage::Export, &self.ctx.func.signature)
            .unwrap();
        self.module.define_function(id, &mut self.ctx).unwrap();
        self.module.clear_context(&mut self.ctx);
        self.module.finalize_definitions().unwrap();
        self.module.get_finalized_function(id)
    }
}
