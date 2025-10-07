use crate::spec::{PanicStrategy, RelocModel, Target, TargetMetadata, base};

pub(crate) fn target() -> Target {
    let mut base = base::doors::opts();
    base.cpu = "i486".into();
    base.disable_redzone = true;
    base.panic_strategy = PanicStrategy::Abort;
    base.features = "+sse".into();
    base.relocation_model = RelocModel::Static;
    base.max_atomic_width = Some(64);
    base.rustc_abi = None;
    base.link_script = Some(
        r#"
MEMORY
{
  ram (!rx) : org = 0x40000000, l = 3M
}

ENTRY(_start)

SECTIONS
{
  . = 0x40000000;
  .text :
  {
    *(.text);
    *(.text.*);
    *(.ltext.*);
    *(.got);
    *(.got.plt);
  } > ram
  .dynamic :
  {

  } > ram
  .strings : {
    *(.dynstr);
  } > ram
  .data : {
    *(.data);
    *(.data.*);
  } > ram
  .rodata : { *(.rodata) } > ram
  .rela : { *(.rela.dyn); } > ram
  .strtab : { *(.strtab); } > ram
  .bss : {
    *(.bss);
    *(.bss.*);
  } > ram
  .eh_frame_hdr : { *(.eh_frame_hdr) } > ram
  .eh_frame : { *(.eh_frame) } > ram
  /DISCARD/ :
  {
    *(.dynamic);
    *(.dynsym);
    *(.gnu.hash);
    *(.comment);
    *(.hash);
  }
}
"#
        .into(),
    );

    Target {
        llvm_target: "i486-unknown-none".into(),
        pointer_width: 32,
        data_layout:
            "e-m:e-p:32:32-p270:32:32-p271:32:32-p272:64:64-i128:128-f64:32:64-f80:32-n8:16:32-S128"
                .into(),
        arch: "x86".into(),
        options: base,
        metadata: TargetMetadata {
            description: None,
            tier: None,
            host_tools: None,
            std: Some(true),
        },
    }
}
