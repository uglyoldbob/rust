use crate::spec::{
    Cc, LinkerFlavor, Lld, RelocModel, StackProbeType, TargetOptions, crt_objects, cvs,
};

pub(crate) fn opts() -> TargetOptions {
    let mut pre_link_args = std::collections::BTreeMap::new();
    pre_link_args.insert(LinkerFlavor::Gnu(Cc::No, Lld::Yes), cvs!["--entry=_start"].to_vec());

    TargetOptions {
        os: "doors".into(),
        families: crate::spec::Cow::from(vec!["doors".into()]),
        linker: Some("rust-lld".into()),
        linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
        stack_probes: StackProbeType::Inline,
        relocation_model: RelocModel::Static,
        pre_link_args,
        pre_link_objects: crt_objects::pre_doors(),
        exe_suffix: "".into(),
        has_thread_local: false,
        position_independent_executables: false,
        static_position_independent_executables: false,
        no_builtins: true,
        ..Default::default()
    }
}
