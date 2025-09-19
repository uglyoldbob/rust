use crate::spec::{Cc, CodeModel, LinkerFlavor, Lld, StackProbeType, TargetOptions};

pub(crate) fn opts() -> TargetOptions {
    TargetOptions {
        os: "doors".into(),
        families: crate::spec::Cow::from(vec!["doors".into()]),
        linker: Some("rust-lld".into()),
        code_model: Some(CodeModel::Large),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        stack_probes: StackProbeType::Inline,
        has_thread_local: false,
        ..Default::default()
    }
}
