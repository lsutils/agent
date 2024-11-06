#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KubernetesApiInfo {
    #[prost(string, optional, tag = "1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub info: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bytes = "vec", optional, tag = "3")]
    pub compressed_info: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
/// TODO: To be deleted in the future
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TridentType {
    TtUnknown = 0,
    /// Agent in KVM
    TtProcess = 1,
    /// Agent in a dedicated VM on ESXi
    TtVm = 2,
    /// Agent in Cloud host (VM)
    TtPublicCloud = 3,
    /// _ = 4;                              // --deprecated--
    ///
    /// Agent in Cloud host (BM), or legacy host
    TtPhysicalMachine = 5,
    /// Agent in a dedicated host to receive mirror traffic
    TtDedicatedPhysicalMachine = 6,
    /// Agent in K8s Node (Cloud BM, or legacy host)
    TtHostPod = 7,
    /// Agent in K8s Node (Cloud VM)
    TtVmPod = 8,
    /// Agent in a dedicated host to decap tunnel traffic
    TtTunnelDecapsulation = 9,
    /// Agent in Hyper-V Compute Node
    TtHyperVCompute = 10,
    /// Agent in Hyper-V Network Node
    TtHyperVNetwork = 11,
    /// Agent in K8s POD
    TtK8sSidecar = 12,
}
impl TridentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TridentType::TtUnknown => "TT_UNKNOWN",
            TridentType::TtProcess => "TT_PROCESS",
            TridentType::TtVm => "TT_VM",
            TridentType::TtPublicCloud => "TT_PUBLIC_CLOUD",
            TridentType::TtPhysicalMachine => "TT_PHYSICAL_MACHINE",
            TridentType::TtDedicatedPhysicalMachine => "TT_DEDICATED_PHYSICAL_MACHINE",
            TridentType::TtHostPod => "TT_HOST_POD",
            TridentType::TtVmPod => "TT_VM_POD",
            TridentType::TtTunnelDecapsulation => "TT_TUNNEL_DECAPSULATION",
            TridentType::TtHyperVCompute => "TT_HYPER_V_COMPUTE",
            TridentType::TtHyperVNetwork => "TT_HYPER_V_NETWORK",
            TridentType::TtK8sSidecar => "TT_K8S_SIDECAR",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TT_UNKNOWN" => Some(Self::TtUnknown),
            "TT_PROCESS" => Some(Self::TtProcess),
            "TT_VM" => Some(Self::TtVm),
            "TT_PUBLIC_CLOUD" => Some(Self::TtPublicCloud),
            "TT_PHYSICAL_MACHINE" => Some(Self::TtPhysicalMachine),
            "TT_DEDICATED_PHYSICAL_MACHINE" => Some(Self::TtDedicatedPhysicalMachine),
            "TT_HOST_POD" => Some(Self::TtHostPod),
            "TT_VM_POD" => Some(Self::TtVmPod),
            "TT_TUNNEL_DECAPSULATION" => Some(Self::TtTunnelDecapsulation),
            "TT_HYPER_V_COMPUTE" => Some(Self::TtHyperVCompute),
            "TT_HYPER_V_NETWORK" => Some(Self::TtHyperVNetwork),
            "TT_K8S_SIDECAR" => Some(Self::TtK8sSidecar),
            _ => None,
        }
    }
}
