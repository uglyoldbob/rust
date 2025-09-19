use crate::spec::{PanicStrategy, RelocModel, Target, TargetMetadata, base};

pub(crate) fn target() -> Target {
    let mut base = base::doors::opts();
    base.cpu = "x86-64".into();
    base.disable_redzone = true;
    base.panic_strategy = PanicStrategy::Abort;
    base.features = "-mmx,-sse,+soft-float".into();
    base.relocation_model = RelocModel::Pic;
    base.link_script = Some(
        r#"
MEMORY
{
  ram (!rx) : org = 0x8000000000, l = 3M
}

ENTRY(_start)

SECTIONS
{
  . = 0x8000000000;
  .text :
  {
    KEEP(*(.text._start))
    *(.text);
    *(.text.*);
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
        llvm_target: "x86_64-unknown-none".into(),
        pointer_width: 64,
        data_layout:
            "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-f80:128-n8:16:32:64-S128".into(),
        arch: "x86_64".into(),
        options: base,
        metadata: TargetMetadata {
            description: None,
            tier: None,
            host_tools: None,
            std: Some(true),
        },
    }
}
