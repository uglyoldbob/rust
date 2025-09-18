use crate::spec::{Cc, LinkerFlavor, Lld, RelocModel, StackProbeType, TargetOptions};

pub(crate) fn opts() -> TargetOptions {
    TargetOptions {
        os: "doors".into(),
        families: crate::spec::Cow::from(vec!["doors".into()]),
        linker: Some("rust-lld".into()),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        stack_probes: StackProbeType::Inline,
        relocation_model: RelocModel::Static,
        ..Default::default()
    }
}