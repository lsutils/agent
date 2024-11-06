#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncRequest {
    #[prost(uint32, optional, tag = "1")]
    pub boot_time: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "2", default = "true")]
    pub config_accepted: ::core::option::Option<bool>,
    #[prost(enumeration = "State", optional, tag = "4")]
    pub state: ::core::option::Option<i32>,
    /// agent用于self-update
    #[prost(string, optional, tag = "5")]
    pub revision: ::core::option::Option<::prost::alloc::string::String>,
    /// agent exception status
    #[prost(uint64, optional, tag = "6", default = "0")]
    pub exception: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "7")]
    pub process_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "PacketCaptureType", optional, tag = "8", default = "Local")]
    pub packet_capture_type: ::core::option::Option<i32>,
    /// only platform data
    #[prost(uint64, optional, tag = "9", default = "0")]
    pub version_platform_data: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "10", default = "0")]
    pub version_acls: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "11", default = "0")]
    pub version_groups: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "12")]
    pub current_k8s_image: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "21")]
    pub ctrl_ip: ::core::option::Option<::prost::alloc::string::String>,
    /// 表示hostname，操作系统的原始主机名，注册和信息同步使用
    #[prost(string, optional, tag = "22")]
    pub host: ::core::option::Option<::prost::alloc::string::String>,
    /// 仅作为注册使用
    #[prost(string, repeated, tag = "23")]
    pub host_ips: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "25")]
    pub ctrl_mac: ::core::option::Option<::prost::alloc::string::String>,
    /// 支持 agent 自动加入组
    #[prost(string, optional, tag = "26")]
    pub agent_group_id_request: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "27", default = "false")]
    pub kubernetes_force_watch: ::core::option::Option<bool>,
    #[prost(enumeration = "AgentIdentifier", optional, tag = "28", default = "IpAndMac")]
    pub agent_unique_identifier: ::core::option::Option<i32>,
    /// agent team identity
    #[prost(string, optional, tag = "29")]
    pub team_id: ::core::option::Option<::prost::alloc::string::String>,
    /// 运行环境基本信息
    #[prost(uint32, optional, tag = "32")]
    pub cpu_num: ::core::option::Option<u32>,
    /// 单位：Bytes
    #[prost(uint64, optional, tag = "33")]
    pub memory_size: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "34")]
    pub arch: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "35")]
    pub os: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "36")]
    pub kernel_version: ::core::option::Option<::prost::alloc::string::String>,
    /// 仅对容器类型的 agent 有意义
    #[prost(string, optional, tag = "45")]
    pub kubernetes_cluster_id: ::core::option::Option<::prost::alloc::string::String>,
    /// 仅对容器类型的 agent 有意义
    #[prost(string, optional, tag = "46")]
    pub kubernetes_cluster_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CaptureNetworkType {
    #[prost(uint32, optional, tag = "1")]
    pub capture_network_type: ::core::option::Option<u32>,
    #[prost(enumeration = "PacketType", optional, tag = "2")]
    pub packet_type: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub vlan: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "4")]
    pub source_ip: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "5")]
    pub capture_network_port: ::core::option::Option<u32>,
}
/// e.g. single LAN area
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Segment {
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
    #[prost(string, repeated, tag = "2")]
    pub mac: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// mac对应的Interface id
    #[prost(uint32, repeated, packed = "false", tag = "3")]
    pub interface_id: ::prost::alloc::vec::Vec<u32>,
    /// if interface vmac is not null, vmac = interface vmac; else vmac = interface mac
    #[prost(string, repeated, tag = "4")]
    pub vmac: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IpResource {
    #[prost(string, optional, tag = "1")]
    pub ip: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "2", default = "32")]
    pub masklen: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3", default = "0")]
    pub subnet_id: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Interface {
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
    #[prost(enumeration = "DeviceType", optional, tag = "2")]
    pub device_type: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "3")]
    pub if_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4")]
    pub epc_id: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "5")]
    pub ip_resources: ::prost::alloc::vec::Vec<IpResource>,
    #[prost(uint32, optional, tag = "6")]
    pub region_id: ::core::option::Option<u32>,
    /// 0x0123456789ab = 01:23:45:67:89:ab, 为0时if_type为WAN的数据
    #[prost(uint64, optional, tag = "7")]
    pub mac: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "8")]
    pub pod_node_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "9")]
    pub pod_cluster_id: ::core::option::Option<u32>,
    /// 目前仅微软MUX设配为true
    #[prost(bool, optional, tag = "10", default = "false")]
    pub is_vip_interface: ::core::option::Option<bool>,
}
/// 字段含义查看README
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Group {
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2", default = "0")]
    pub epc_id: ::core::option::Option<u32>,
    #[prost(enumeration = "GroupType", optional, tag = "3")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(string, repeated, tag = "5")]
    pub ips: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "6")]
    pub ip_ranges: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Groups {
    #[prost(message, repeated, tag = "1")]
    pub groups: ::prost::alloc::vec::Vec<Group>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeerConnection {
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub local_epc_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3")]
    pub remote_epc_id: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cidr {
    #[prost(string, optional, tag = "1")]
    pub prefix: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "CidrType", optional, tag = "2")]
    pub r#type: ::core::option::Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub epc_id: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "4")]
    pub region_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5")]
    pub tunnel_id: ::core::option::Option<u32>,
    #[prost(bool, optional, tag = "20", default = "false")]
    pub is_vip: ::core::option::Option<bool>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Container {
    #[prost(uint32, optional, tag = "1")]
    pub pod_id: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "2")]
    pub container_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PlatformData {
    #[prost(message, repeated, tag = "1")]
    pub interfaces: ::prost::alloc::vec::Vec<Interface>,
    #[prost(message, repeated, tag = "3")]
    pub peer_connections: ::prost::alloc::vec::Vec<PeerConnection>,
    #[prost(message, repeated, tag = "4")]
    pub cidrs: ::prost::alloc::vec::Vec<Cidr>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NpbAction {
    #[prost(enumeration = "TunnelType", optional, tag = "1", default = "Vxlan")]
    pub tunnel_type: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2")]
    pub tunnel_id: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "3")]
    pub tunnel_ip: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "PacketCaptureSide", optional, tag = "4")]
    pub packet_capture_side: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "5", default = "65535")]
    pub payload_slice: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6")]
    pub npb_acl_group_id: ::core::option::Option<u32>,
    /// 分发点id, 限制在64000
    #[prost(uint32, optional, tag = "7")]
    pub tunnel_ip_id: ::core::option::Option<u32>,
    #[prost(enumeration = "Direction", optional, tag = "8", default = "All")]
    pub direction: ::core::option::Option<i32>,
}
/// 字段含义查看README
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlowAcl {
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2")]
    pub capture_network_type: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3", default = "256")]
    pub protocol: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "4")]
    pub src_ports: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub dst_ports: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "6")]
    pub npb_actions: ::prost::alloc::vec::Vec<NpbAction>,
    #[prost(int32, repeated, packed = "false", tag = "7")]
    pub src_group_ids: ::prost::alloc::vec::Vec<i32>,
    #[prost(int32, repeated, packed = "false", tag = "8")]
    pub dst_group_ids: ::prost::alloc::vec::Vec<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlowAcls {
    #[prost(message, repeated, tag = "1")]
    pub flow_acl: ::prost::alloc::vec::Vec<FlowAcl>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SkipInterface {
    /// 若该接口对应的虚拟机内已经部署 agent,
    /// 发送此接口给虚拟机所在宿主机 agent
    #[prost(uint64, optional, tag = "1")]
    pub mac: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DynamicConfig {
    #[prost(bool, optional, tag = "1")]
    pub kubernetes_api_enabled: ::core::option::Option<bool>,
    /// Region ID of the deepflow-agent or Region ID of the data node
    #[prost(uint32, optional, tag = "10")]
    pub region_id: ::core::option::Option<u32>,
    /// Cluster ID of the container where the deepflow-agent is located
    #[prost(uint32, optional, tag = "11")]
    pub pod_cluster_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "12")]
    pub vpc_id: ::core::option::Option<u32>,
    /// range: [0, 64000]
    #[prost(uint32, optional, tag = "13")]
    pub agent_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "14")]
    pub team_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "15")]
    pub organize_id: ::core::option::Option<u32>,
    /// secret key for dataplane
    #[prost(string, optional, tag = "20")]
    pub secret_key: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncResponse {
    #[prost(enumeration = "Status", optional, tag = "1")]
    pub status: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "2")]
    pub user_config: ::core::option::Option<::prost::alloc::string::String>,
    /// 指定升级的目标revision
    #[prost(string, optional, tag = "3")]
    pub revision: ::core::option::Option<::prost::alloc::string::String>,
    /// 指定升级的URL路径
    #[prost(string, optional, tag = "4")]
    pub self_update_url: ::core::option::Option<::prost::alloc::string::String>,
    /// only platform data
    #[prost(uint64, optional, tag = "5", default = "0")]
    pub version_platform_data: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "6", default = "0")]
    pub version_acls: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag = "7", default = "0")]
    pub version_groups: ::core::option::Option<u64>,
    /// The controller sends a container list to each agent, which contains a list of
    /// containers in the operating system that the agent is running on (Note that only
    /// the local container will be issued, not other machines)
    /// =================================================================================
    /// 控制器向每个 Agent 下发一个 container list，其内容为该 Agent 运行操作
    /// 系统中的 container 列表（注意仅会下发本机的 container，不会包含其他机器的）
    #[prost(message, repeated, tag = "8")]
    pub containers: ::prost::alloc::vec::Vec<Container>,
    #[prost(message, repeated, tag = "9")]
    pub local_segments: ::prost::alloc::vec::Vec<Segment>,
    #[prost(message, repeated, tag = "10")]
    pub remote_segments: ::prost::alloc::vec::Vec<Segment>,
    /// serialized result of `message PlatformData`, transmitted only when the content changes
    #[prost(bytes = "vec", optional, tag = "11")]
    pub platform_data: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// serialized result of `message FlowAcls`, transmitted only when the content changes
    #[prost(bytes = "vec", optional, tag = "12")]
    pub flow_acls: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// serialized result of `message Groups`, transmitted only when the content changes
    #[prost(bytes = "vec", optional, tag = "13")]
    pub groups: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag = "14")]
    pub capture_network_types: ::prost::alloc::vec::Vec<CaptureNetworkType>,
    #[prost(message, repeated, tag = "15")]
    pub skip_interface: ::prost::alloc::vec::Vec<SkipInterface>,
    #[prost(message, optional, tag = "20")]
    pub dynamic_config: ::core::option::Option<DynamicConfig>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeRequest {
    #[prost(string, optional, tag = "1")]
    pub ctrl_ip: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub ctrl_mac: ::core::option::Option<::prost::alloc::string::String>,
    /// agent team identity
    #[prost(string, optional, tag = "3")]
    pub team_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradeResponse {
    /// 调用是否成功
    #[prost(enumeration = "Status", optional, tag = "1")]
    pub status: ::core::option::Option<i32>,
    /// 数据
    #[prost(bytes = "vec", optional, tag = "2")]
    pub content: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// 文件MD5
    #[prost(string, optional, tag = "3")]
    pub md5: ::core::option::Option<::prost::alloc::string::String>,
    /// 数据总长
    #[prost(uint64, optional, tag = "4")]
    pub total_len: ::core::option::Option<u64>,
    /// 包总个数
    #[prost(uint32, optional, tag = "5")]
    pub pkt_count: ::core::option::Option<u32>,
    /// When k8s_image is not empty, ignore content
    #[prost(string, optional, tag = "6")]
    pub k8s_image: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NtpRequest {
    /// 请求端的控制口IP
    #[prost(string, optional, tag = "1")]
    pub ctrl_ip: ::core::option::Option<::prost::alloc::string::String>,
    /// 数据
    #[prost(bytes = "vec", optional, tag = "10")]
    pub request: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NtpResponse {
    /// 数据
    #[prost(bytes = "vec", optional, tag = "1")]
    pub response: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PluginConfig {
    /// latest epoch of all configured plugins
    #[prost(uint32, optional, tag = "1", default = "0")]
    pub update_time: ::core::option::Option<u32>,
    #[prost(string, repeated, tag = "2")]
    pub wasm_plugins: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub so_plugins: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PluginRequest {
    #[prost(string, optional, tag = "1")]
    pub ctrl_ip: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub ctrl_mac: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "PluginType", optional, tag = "3")]
    pub plugin_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "4")]
    pub plugin_name: ::core::option::Option<::prost::alloc::string::String>,
    /// agent team identity
    #[prost(string, optional, tag = "5")]
    pub team_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PluginResponse {
    /// 调用是否成功
    #[prost(enumeration = "Status", optional, tag = "1")]
    pub status: ::core::option::Option<i32>,
    /// 数据
    #[prost(bytes = "vec", optional, tag = "2")]
    pub content: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// 文件MD5
    #[prost(string, optional, tag = "3")]
    pub md5: ::core::option::Option<::prost::alloc::string::String>,
    /// 数据总长
    #[prost(uint64, optional, tag = "4")]
    pub total_len: ::core::option::Option<u64>,
    /// 包总个数
    #[prost(uint32, optional, tag = "5")]
    pub pkt_count: ::core::option::Option<u32>,
    /// plugin update epoch
    #[prost(uint32, optional, tag = "6", default = "0")]
    pub update_time: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisPlatformData {
    #[prost(message, repeated, tag = "7")]
    pub ips: ::prost::alloc::vec::Vec<Ip>,
    #[prost(bool, optional, tag = "10")]
    pub platform_enabled: ::core::option::Option<bool>,
    #[prost(string, optional, tag = "11")]
    pub raw_hostname: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "12")]
    pub raw_all_vm_xml: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "13")]
    pub raw_vm_states: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "14")]
    pub raw_ovs_interfaces: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "15")]
    pub raw_ovs_ports: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "16")]
    pub raw_brctl_show: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "17")]
    pub raw_vlan_config: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "20")]
    pub lldp_info: ::prost::alloc::vec::Vec<Lldp>,
    #[prost(string, repeated, tag = "30")]
    pub raw_ip_netns: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "31")]
    pub raw_ip_addrs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "32")]
    pub interfaces: ::prost::alloc::vec::Vec<InterfaceInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Ip {
    #[prost(uint32, optional, tag = "2")]
    pub last_seen: ::core::option::Option<u32>,
    #[prost(uint64, optional, tag = "3")]
    pub mac: ::core::option::Option<u64>,
    #[prost(bytes = "vec", optional, tag = "4")]
    pub ip: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(string, optional, tag = "9")]
    pub port_uuid: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Lldp {
    #[prost(string, optional, tag = "1")]
    pub interface: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub system_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub management_address: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "10")]
    pub port_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "11")]
    pub port_description: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InterfaceInfo {
    #[prost(uint64, optional, tag = "1")]
    pub mac: ::core::option::Option<u64>,
    /// ip/masklen
    #[prost(string, repeated, tag = "2")]
    pub ip: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub device_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "5")]
    pub tap_index: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "6")]
    pub device_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "7", default = "")]
    pub netns: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "8", default = "0")]
    pub netns_id: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "9", default = "")]
    pub if_type: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Tag {
    #[prost(string, optional, tag = "1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProcessInfo {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "2")]
    pub pid: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "3")]
    pub process_name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "4")]
    pub cmdline: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "5")]
    pub user: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "6")]
    pub start_time: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7", default = "0")]
    pub netns_id: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "8", default = "")]
    pub container_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "11")]
    pub os_app_tags: ::prost::alloc::vec::Vec<Tag>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisProcessData {
    #[prost(message, repeated, tag = "5")]
    pub process_entries: ::prost::alloc::vec::Vec<ProcessInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisSyncRequest {
    #[prost(uint64, optional, tag = "1", default = "0")]
    pub version: ::core::option::Option<u64>,
    #[prost(enumeration = "AgentType", optional, tag = "2")]
    pub agent_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "3")]
    pub source_ip: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "4")]
    pub agent_id: ::core::option::Option<u32>,
    /// 仅对容器类型的 agent 有意义
    #[prost(string, optional, tag = "5")]
    pub kubernetes_cluster_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "6")]
    pub nat_ip: ::core::option::Option<::prost::alloc::string::String>,
    /// agent team identity
    #[prost(string, optional, tag = "7")]
    pub team_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "9")]
    pub platform_data: ::core::option::Option<GenesisPlatformData>,
    #[prost(message, optional, tag = "10")]
    pub process_data: ::core::option::Option<GenesisProcessData>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisSyncResponse {
    #[prost(uint64, optional, tag = "1", default = "0")]
    pub version: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KubernetesApiSyncRequest {
    #[prost(string, optional, tag = "1")]
    pub cluster_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "2")]
    pub version: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "3")]
    pub error_msg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "4")]
    pub agent_id: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "5")]
    pub source_ip: ::core::option::Option<::prost::alloc::string::String>,
    /// agent team identity
    #[prost(string, optional, tag = "6")]
    pub team_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "10")]
    pub entries: ::prost::alloc::vec::Vec<super::common::KubernetesApiInfo>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KubernetesApiSyncResponse {
    #[prost(uint64, optional, tag = "1")]
    pub version: ::core::option::Option<u64>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KubernetesClusterIdRequest {
    /// md5 of /run/secrets/[kubernetes.io/serviceaccount/ca.crt
    #[prost(string, optional, tag = "1")]
    pub ca_md5: ::core::option::Option<::prost::alloc::string::String>,
    /// agent team identity
    #[prost(string, optional, tag = "2")]
    pub team_id: ::core::option::Option<::prost::alloc::string::String>,
    /// get that from deepflow-agent.yaml
    #[prost(string, optional, tag = "46")]
    pub kubernetes_cluster_name: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KubernetesClusterIdResponse {
    #[prost(string, optional, tag = "1")]
    pub error_msg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub cluster_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pcap {
    #[prost(uint64, optional, tag = "1")]
    pub flow_id: ::core::option::Option<u64>,
    /// ns
    #[prost(uint64, optional, tag = "2")]
    pub start_time: ::core::option::Option<u64>,
    /// ns
    #[prost(uint64, optional, tag = "3")]
    pub end_time: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "4")]
    pub packet_count: ::core::option::Option<u32>,
    #[prost(bytes = "vec", optional, tag = "5")]
    pub packet_records: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, repeated, packed = "false", tag = "6")]
    pub acl_gids: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PcapBatch {
    #[prost(uint32, optional, tag = "1")]
    pub magic: ::core::option::Option<u32>,
    #[prost(message, repeated, tag = "2")]
    pub batches: ::prost::alloc::vec::Vec<Pcap>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GpidSyncEntry {
    /// ANY means compressed
    #[prost(enumeration = "ServiceProtocol", optional, tag = "1", default = "Any")]
    pub protocol: ::core::option::Option<i32>,
    /// server side
    ///
    /// u16
    #[prost(uint32, optional, tag = "2", default = "0")]
    pub epc_id_1: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3", default = "0")]
    pub ipv4_1: ::core::option::Option<u32>,
    /// u16
    #[prost(uint32, optional, tag = "4", default = "0")]
    pub port_1: ::core::option::Option<u32>,
    /// pid or gpid
    #[prost(uint32, optional, tag = "5", default = "0")]
    pub pid_1: ::core::option::Option<u32>,
    /// client side
    ///
    /// u16
    #[prost(uint32, optional, tag = "6", default = "0")]
    pub epc_id_0: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7", default = "0")]
    pub ipv4_0: ::core::option::Option<u32>,
    /// u16
    #[prost(uint32, optional, tag = "8", default = "0")]
    pub port_0: ::core::option::Option<u32>,
    /// pid or gpid
    #[prost(uint32, optional, tag = "9", default = "0")]
    pub pid_0: ::core::option::Option<u32>,
    /// real ip (before or after NAT)
    ///
    /// u16
    #[prost(uint32, optional, tag = "10", default = "0")]
    pub epc_id_real: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "11", default = "0")]
    pub ipv4_real: ::core::option::Option<u32>,
    /// u16
    #[prost(uint32, optional, tag = "12", default = "0")]
    pub port_real: ::core::option::Option<u32>,
    /// pid or gpid
    #[prost(uint32, optional, tag = "13", default = "0")]
    pub pid_real: ::core::option::Option<u32>,
    /// role of real ip, ROLE_NONE means compressed
    #[prost(enumeration = "RoleType", optional, tag = "14", default = "RoleNone")]
    pub role_real: ::core::option::Option<i32>,
    /// the net namespace index,  neither netns id nor netns inode
    ///
    /// u16
    #[prost(uint32, optional, tag = "15", default = "0")]
    pub netns_idx: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GpidSyncRequest {
    #[prost(string, optional, tag = "1")]
    pub ctrl_ip: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub ctrl_mac: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "3")]
    pub agent_id: ::core::option::Option<u32>,
    #[prost(
        enumeration = "GpidSyncCompressAlgorithm",
        optional,
        tag = "4",
        default = "CompressAlroNone"
    )]
    pub entries_compress_algorithm: ::core::option::Option<i32>,
    /// agent team identity
    #[prost(string, optional, tag = "5")]
    pub team_id: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "10")]
    pub entries: ::prost::alloc::vec::Vec<GpidSyncEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GpidSyncResponse {
    #[prost(
        enumeration = "GpidSyncCompressAlgorithm",
        optional,
        tag = "1",
        default = "CompressAlroNone"
    )]
    pub entries_compress_algorithm: ::core::option::Option<i32>,
    #[prost(message, repeated, tag = "2")]
    pub entries: ::prost::alloc::vec::Vec<GpidSyncEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GlobalGpidEntry {
    /// ANY means compressed
    #[prost(enumeration = "ServiceProtocol", optional, tag = "1", default = "Any")]
    pub protocol: ::core::option::Option<i32>,
    /// server side
    #[prost(uint32, optional, tag = "2", default = "0")]
    pub agent_id_1: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3", default = "0")]
    pub epc_id_1: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4", default = "0")]
    pub ipv4_1: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5", default = "0")]
    pub port_1: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6", default = "0")]
    pub pid_1: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7", default = "0")]
    pub gpid_1: ::core::option::Option<u32>,
    /// client side
    #[prost(uint32, optional, tag = "8", default = "0")]
    pub agent_id_0: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "9", default = "0")]
    pub epc_id_0: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "10", default = "0")]
    pub ipv4_0: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "11", default = "0")]
    pub port_0: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "12", default = "0")]
    pub pid_0: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "13", default = "0")]
    pub gpid_0: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "14", default = "0")]
    pub netns_idx: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GpidGlobalData {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<GlobalGpidEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealClientToRealServer {
    #[prost(uint32, optional, tag = "1", default = "0")]
    pub epc_id_0: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "2", default = "0")]
    pub ipv4_0: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3", default = "0")]
    pub port_0: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4", default = "0")]
    pub epc_id_1: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5", default = "0")]
    pub ipv4_1: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6", default = "0")]
    pub port_1: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "7", default = "0")]
    pub epc_id_real: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "8", default = "0")]
    pub ipv4_real: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "9", default = "0")]
    pub port_real: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "10", default = "0")]
    pub pid_real: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "11", default = "0")]
    pub agent_id_real: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealGlobalData {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<RealClientToRealServer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RipToVip {
    /// ANY means compressed
    #[prost(enumeration = "ServiceProtocol", optional, tag = "1", default = "Any")]
    pub protocol: ::core::option::Option<i32>,
    #[prost(uint32, optional, tag = "2", default = "0")]
    pub epc_id: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "3", default = "0")]
    pub r_ipv4: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "4", default = "0")]
    pub r_port: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "5", default = "0")]
    pub v_ipv4: ::core::option::Option<u32>,
    #[prost(uint32, optional, tag = "6", default = "0")]
    pub v_port: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RvData {
    #[prost(message, repeated, tag = "1")]
    pub entries: ::prost::alloc::vec::Vec<RipToVip>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GpidAgentData {
    #[prost(uint32, optional, tag = "1", default = "0")]
    pub update_time: ::core::option::Option<u32>,
    #[prost(message, optional, tag = "2")]
    pub sync_request: ::core::option::Option<GpidSyncRequest>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgentCacheRequest {
    #[prost(string, optional, tag = "1")]
    pub ctrl_ip: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub ctrl_mac: ::core::option::Option<::prost::alloc::string::String>,
    /// agent team identity
    #[prost(string, optional, tag = "3")]
    pub team_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgentCacheResponse {
    /// json数据
    #[prost(bytes = "vec", optional, tag = "1")]
    pub content: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgentId {
    #[prost(string, optional, tag = "1")]
    pub ip: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub mac: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub team_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandParam {
    #[prost(string, optional, tag = "1")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "2")]
    pub regex: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(bool, optional, tag = "3")]
    pub required: ::core::option::Option<bool>,
    #[prost(enumeration = "ParamType", optional, tag = "4")]
    pub param_type: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "5")]
    pub description: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteCommand {
    #[prost(string, optional, tag = "2")]
    pub cmd: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(enumeration = "OutputFormat", optional, tag = "4")]
    pub output_format: ::core::option::Option<i32>,
    #[prost(string, optional, tag = "6")]
    pub ident: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "7")]
    pub params: ::prost::alloc::vec::Vec<CommandParam>,
    #[prost(string, optional, tag = "8")]
    pub type_name: ::core::option::Option<::prost::alloc::string::String>,
    /// deprecated, use `ident` instead
    #[prost(uint32, optional, tag = "1")]
    pub id: ::core::option::Option<u32>,
    /// deprecated, use `params` instead
    #[prost(string, repeated, tag = "3")]
    pub param_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// deprecated, use `type_name` instead
    #[prost(enumeration = "CommandType", optional, tag = "5")]
    pub cmd_type: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinuxNamespace {
    #[prost(uint64, optional, tag = "1")]
    pub id: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "2")]
    pub ns_type: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(string, optional, tag = "3")]
    pub user: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, optional, tag = "4")]
    pub pid: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "5")]
    pub cmd: ::core::option::Option<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandResult {
    #[prost(int32, optional, tag = "1")]
    pub errno: ::core::option::Option<i32>,
    #[prost(bytes = "vec", optional, tag = "2")]
    pub content: ::core::option::Option<::prost::alloc::vec::Vec<u8>>,
    /// will only be populated in the last segment
    /// also used as end of result
    #[prost(string, optional, tag = "3")]
    pub md5: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint64, optional, tag = "4")]
    pub total_len: ::core::option::Option<u64>,
    #[prost(uint32, optional, tag = "5")]
    pub pkt_count: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Parameter {
    #[prost(string, optional, tag = "1")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
    /// accepts \[A-Za-z0-9-_\]
    #[prost(string, optional, tag = "2")]
    pub value: ::core::option::Option<::prost::alloc::string::String>,
}
/// message from server to agent
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteExecRequest {
    #[prost(uint64, optional, tag = "1")]
    pub request_id: ::core::option::Option<u64>,
    #[prost(enumeration = "ExecutionType", optional, tag = "2")]
    pub exec_type: ::core::option::Option<i32>,
    /// parameters to use in commands
    #[prost(message, repeated, tag = "4")]
    pub params: ::prost::alloc::vec::Vec<Parameter>,
    /// execute command in agent namespace if null
    #[prost(uint32, optional, tag = "5")]
    pub linux_ns_pid: ::core::option::Option<u32>,
    /// batch len of command execution results, min 1024
    #[prost(uint32, optional, tag = "6", default = "1048576")]
    pub batch_len: ::core::option::Option<u32>,
    #[prost(string, optional, tag = "7")]
    pub command_ident: ::core::option::Option<::prost::alloc::string::String>,
    /// deprecated, use `command_ident` instead
    #[prost(uint32, optional, tag = "3")]
    pub command_id: ::core::option::Option<u32>,
}
/// message from agent to server
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteExecResponse {
    #[prost(message, optional, tag = "1")]
    pub agent_id: ::core::option::Option<AgentId>,
    #[prost(uint64, optional, tag = "2")]
    pub request_id: ::core::option::Option<u64>,
    #[prost(string, optional, tag = "3")]
    pub errmsg: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "4")]
    pub commands: ::prost::alloc::vec::Vec<RemoteCommand>,
    #[prost(message, repeated, tag = "5")]
    pub linux_namespaces: ::prost::alloc::vec::Vec<LinuxNamespace>,
    #[prost(message, optional, tag = "6")]
    pub command_result: ::core::option::Option<CommandResult>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AgentType {
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
impl AgentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AgentType::TtUnknown => "TT_UNKNOWN",
            AgentType::TtProcess => "TT_PROCESS",
            AgentType::TtVm => "TT_VM",
            AgentType::TtPublicCloud => "TT_PUBLIC_CLOUD",
            AgentType::TtPhysicalMachine => "TT_PHYSICAL_MACHINE",
            AgentType::TtDedicatedPhysicalMachine => "TT_DEDICATED_PHYSICAL_MACHINE",
            AgentType::TtHostPod => "TT_HOST_POD",
            AgentType::TtVmPod => "TT_VM_POD",
            AgentType::TtTunnelDecapsulation => "TT_TUNNEL_DECAPSULATION",
            AgentType::TtHyperVCompute => "TT_HYPER_V_COMPUTE",
            AgentType::TtHyperVNetwork => "TT_HYPER_V_NETWORK",
            AgentType::TtK8sSidecar => "TT_K8S_SIDECAR",
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum State {
    /// 检查运行环境
    EnvironmentCheck = 0,
    /// 禁用
    Disabled = 1,
    /// 正常运行
    Running = 2,
    /// 因配置变更等缘故触发重启
    Rebooting = 3,
    /// 负载太大产生丢包
    Stressed = 4,
    /// 占用过多系统资源
    Restricted = 5,
}
impl State {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            State::EnvironmentCheck => "ENVIRONMENT_CHECK",
            State::Disabled => "DISABLED",
            State::Running => "RUNNING",
            State::Rebooting => "REBOOTING",
            State::Stressed => "STRESSED",
            State::Restricted => "RESTRICTED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ENVIRONMENT_CHECK" => Some(Self::EnvironmentCheck),
            "DISABLED" => Some(Self::Disabled),
            "RUNNING" => Some(Self::Running),
            "REBOOTING" => Some(Self::Rebooting),
            "STRESSED" => Some(Self::Stressed),
            "RESTRICTED" => Some(Self::Restricted),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Exception {
    Normal = 0,
    DiskNotEnough = 1,
    MemNotEnough = 2,
    CorefileTooMany = 4,
    NpbFuse = 8,
    NpbBpsThresholdExceeded = 16,
    NpbNoGwArp = 32,
    RxPpsThresholdExceeded = 64,
    AnalyzerNoGwArp = 128,
    InvalidConfiguration = 256,
    ThreadThresholdExceeded = 512,
    ProcessThresholdExceeded = 1024,
    /// _  = 2048; // deprecate
    TooManyPolicies = 4096,
    FreeMemExceeded = 8192,
    LogFileExceeded = 16384,
    ControllerSocketError = 32768,
    AnalyzerSocketError = 65536,
    NpbSocketError = 131072,
    IntegrationSocketError = 262144,
    CgroupsConfigError = 524288,
    /// 2^31及以下由 agent ，agent 最大可用异常是2^31，顺序从前往后
    /// 2^32及以上由控制器使用，顺序从后往前
    SystemLoadCircuitBreaker = 1048576,
}
impl Exception {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Exception::Normal => "NORMAL",
            Exception::DiskNotEnough => "DISK_NOT_ENOUGH",
            Exception::MemNotEnough => "MEM_NOT_ENOUGH",
            Exception::CorefileTooMany => "COREFILE_TOO_MANY",
            Exception::NpbFuse => "NPB_FUSE",
            Exception::NpbBpsThresholdExceeded => "NPB_BPS_THRESHOLD_EXCEEDED",
            Exception::NpbNoGwArp => "NPB_NO_GW_ARP",
            Exception::RxPpsThresholdExceeded => "RX_PPS_THRESHOLD_EXCEEDED",
            Exception::AnalyzerNoGwArp => "ANALYZER_NO_GW_ARP",
            Exception::InvalidConfiguration => "INVALID_CONFIGURATION",
            Exception::ThreadThresholdExceeded => "THREAD_THRESHOLD_EXCEEDED",
            Exception::ProcessThresholdExceeded => "PROCESS_THRESHOLD_EXCEEDED",
            Exception::TooManyPolicies => "TOO_MANY_POLICIES",
            Exception::FreeMemExceeded => "FREE_MEM_EXCEEDED",
            Exception::LogFileExceeded => "LOG_FILE_EXCEEDED",
            Exception::ControllerSocketError => "CONTROLLER_SOCKET_ERROR",
            Exception::AnalyzerSocketError => "ANALYZER_SOCKET_ERROR",
            Exception::NpbSocketError => "NPB_SOCKET_ERROR",
            Exception::IntegrationSocketError => "INTEGRATION_SOCKET_ERROR",
            Exception::CgroupsConfigError => "CGROUPS_CONFIG_ERROR",
            Exception::SystemLoadCircuitBreaker => "SYSTEM_LOAD_CIRCUIT_BREAKER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NORMAL" => Some(Self::Normal),
            "DISK_NOT_ENOUGH" => Some(Self::DiskNotEnough),
            "MEM_NOT_ENOUGH" => Some(Self::MemNotEnough),
            "COREFILE_TOO_MANY" => Some(Self::CorefileTooMany),
            "NPB_FUSE" => Some(Self::NpbFuse),
            "NPB_BPS_THRESHOLD_EXCEEDED" => Some(Self::NpbBpsThresholdExceeded),
            "NPB_NO_GW_ARP" => Some(Self::NpbNoGwArp),
            "RX_PPS_THRESHOLD_EXCEEDED" => Some(Self::RxPpsThresholdExceeded),
            "ANALYZER_NO_GW_ARP" => Some(Self::AnalyzerNoGwArp),
            "INVALID_CONFIGURATION" => Some(Self::InvalidConfiguration),
            "THREAD_THRESHOLD_EXCEEDED" => Some(Self::ThreadThresholdExceeded),
            "PROCESS_THRESHOLD_EXCEEDED" => Some(Self::ProcessThresholdExceeded),
            "TOO_MANY_POLICIES" => Some(Self::TooManyPolicies),
            "FREE_MEM_EXCEEDED" => Some(Self::FreeMemExceeded),
            "LOG_FILE_EXCEEDED" => Some(Self::LogFileExceeded),
            "CONTROLLER_SOCKET_ERROR" => Some(Self::ControllerSocketError),
            "ANALYZER_SOCKET_ERROR" => Some(Self::AnalyzerSocketError),
            "NPB_SOCKET_ERROR" => Some(Self::NpbSocketError),
            "INTEGRATION_SOCKET_ERROR" => Some(Self::IntegrationSocketError),
            "CGROUPS_CONFIG_ERROR" => Some(Self::CgroupsConfigError),
            "SYSTEM_LOAD_CIRCUIT_BREAKER" => Some(Self::SystemLoadCircuitBreaker),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Status {
    Success = 0,
    Failed = 1,
    Heartbeat = 2,
    ClusterIdNotFound = 10,
}
impl Status {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Status::Success => "SUCCESS",
            Status::Failed => "FAILED",
            Status::Heartbeat => "HEARTBEAT",
            Status::ClusterIdNotFound => "CLUSTER_ID_NOT_FOUND",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SUCCESS" => Some(Self::Success),
            "FAILED" => Some(Self::Failed),
            "HEARTBEAT" => Some(Self::Heartbeat),
            "CLUSTER_ID_NOT_FOUND" => Some(Self::ClusterIdNotFound),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PacketCaptureType {
    /// 部署在宿主机之上，抓取本地虚拟接口流量
    Local = 0,
    /// 部署在虚拟机之上，抓取镜像而来的流量
    Mirror = 1,
    /// 部署在专属服务器 agent 之上
    Analyzer = 2,
    /// 隧道解封装 agent
    Decap = 3,
}
impl PacketCaptureType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PacketCaptureType::Local => "LOCAL",
            PacketCaptureType::Mirror => "MIRROR",
            PacketCaptureType::Analyzer => "ANALYZER",
            PacketCaptureType::Decap => "DECAP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LOCAL" => Some(Self::Local),
            "MIRROR" => Some(Self::Mirror),
            "ANALYZER" => Some(Self::Analyzer),
            "DECAP" => Some(Self::Decap),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AgentIdentifier {
    IpAndMac = 1,
    Ip = 2,
}
impl AgentIdentifier {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AgentIdentifier::IpAndMac => "IP_AND_MAC",
            AgentIdentifier::Ip => "IP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IP_AND_MAC" => Some(Self::IpAndMac),
            "IP" => Some(Self::Ip),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IfMacSource {
    IfMac = 0,
    IfName = 1,
    /// 从libvirt的xml文件中获取
    IfLibvirtXml = 2,
}
impl IfMacSource {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IfMacSource::IfMac => "IF_MAC",
            IfMacSource::IfName => "IF_NAME",
            IfMacSource::IfLibvirtXml => "IF_LIBVIRT_XML",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "IF_MAC" => Some(Self::IfMac),
            "IF_NAME" => Some(Self::IfName),
            "IF_LIBVIRT_XML" => Some(Self::IfLibvirtXml),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SocketType {
    RawUdp = 0,
    Tcp = 1,
    Udp = 2,
    File = 3,
}
impl SocketType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SocketType::RawUdp => "RAW_UDP",
            SocketType::Tcp => "TCP",
            SocketType::Udp => "UDP",
            SocketType::File => "FILE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "RAW_UDP" => Some(Self::RawUdp),
            "TCP" => Some(Self::Tcp),
            "UDP" => Some(Self::Udp),
            "FILE" => Some(Self::File),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PacketType {
    Packet = 1,
    Sflow = 2,
    NetflowV5 = 3,
    NetstreamV5 = 4,
    NetflowV9 = 5,
    NetstreamV9 = 6,
}
impl PacketType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PacketType::Packet => "PACKET",
            PacketType::Sflow => "SFLOW",
            PacketType::NetflowV5 => "NETFLOW_V5",
            PacketType::NetstreamV5 => "NETSTREAM_V5",
            PacketType::NetflowV9 => "NETFLOW_V9",
            PacketType::NetstreamV9 => "NETSTREAM_V9",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PACKET" => Some(Self::Packet),
            "SFLOW" => Some(Self::Sflow),
            "NETFLOW_V5" => Some(Self::NetflowV5),
            "NETSTREAM_V5" => Some(Self::NetstreamV5),
            "NETFLOW_V9" => Some(Self::NetflowV9),
            "NETSTREAM_V9" => Some(Self::NetstreamV9),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CaptureSocketType {
    Auto = 0,
    AfPacketV1 = 1,
    AfPacketV2 = 2,
    AfPacketV3 = 3,
}
impl CaptureSocketType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CaptureSocketType::Auto => "AUTO",
            CaptureSocketType::AfPacketV1 => "AF_PACKET_V1",
            CaptureSocketType::AfPacketV2 => "AF_PACKET_V2",
            CaptureSocketType::AfPacketV3 => "AF_PACKET_V3",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUTO" => Some(Self::Auto),
            "AF_PACKET_V1" => Some(Self::AfPacketV1),
            "AF_PACKET_V2" => Some(Self::AfPacketV2),
            "AF_PACKET_V3" => Some(Self::AfPacketV3),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum VlanMode {
    None = 0,
    Vlan = 1,
    Qinq = 2,
}
impl VlanMode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            VlanMode::None => "NONE",
            VlanMode::Vlan => "VLAN",
            VlanMode::Qinq => "QINQ",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NONE" => Some(Self::None),
            "VLAN" => Some(Self::Vlan),
            "QINQ" => Some(Self::Qinq),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DecapType {
    None = 0,
    Vxlan = 1,
    Ipip = 2,
    Tencent = 3,
    Geneve = 4,
}
impl DecapType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DecapType::None => "DECAP_TYPE_NONE",
            DecapType::Vxlan => "DECAP_TYPE_VXLAN",
            DecapType::Ipip => "DECAP_TYPE_IPIP",
            DecapType::Tencent => "DECAP_TYPE_TENCENT",
            DecapType::Geneve => "DECAP_TYPE_GENEVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DECAP_TYPE_NONE" => Some(Self::None),
            "DECAP_TYPE_VXLAN" => Some(Self::Vxlan),
            "DECAP_TYPE_IPIP" => Some(Self::Ipip),
            "DECAP_TYPE_TENCENT" => Some(Self::Tencent),
            "DECAP_TYPE_GENEVE" => Some(Self::Geneve),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SystemLoadMetric {
    Load1 = 0,
    Load5 = 1,
    Load15 = 2,
}
impl SystemLoadMetric {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SystemLoadMetric::Load1 => "Load1",
            SystemLoadMetric::Load5 => "Load5",
            SystemLoadMetric::Load15 => "Load15",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Load1" => Some(Self::Load1),
            "Load5" => Some(Self::Load5),
            "Load15" => Some(Self::Load15),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DeviceType {
    Unknown = 0,
    Vm = 1,
    Vgw = 2,
    ThirdPartyDevice = 3,
    Vmwaf = 4,
    NspVgateway = 5,
    HostDevice = 6,
    NetworkDevice = 7,
    DhcpPort = 9,
    Pod = 10,
    PodService = 11,
    RedisInstance = 12,
    RdsInstance = 13,
    PodNode = 14,
    LoadBalance = 15,
    NatGateway = 16,
}
impl DeviceType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DeviceType::Unknown => "DEVICE_TYPE_UNKNOWN",
            DeviceType::Vm => "DEVICE_TYPE_VM",
            DeviceType::Vgw => "DEVICE_TYPE_VGW",
            DeviceType::ThirdPartyDevice => "DEVICE_TYPE_THIRD_PARTY_DEVICE",
            DeviceType::Vmwaf => "DEVICE_TYPE_VMWAF",
            DeviceType::NspVgateway => "DEVICE_TYPE_NSP_VGATEWAY",
            DeviceType::HostDevice => "DEVICE_TYPE_HOST_DEVICE",
            DeviceType::NetworkDevice => "DEVICE_TYPE_NETWORK_DEVICE",
            DeviceType::DhcpPort => "DEVICE_TYPE_DHCP_PORT",
            DeviceType::Pod => "DEVICE_TYPE_POD",
            DeviceType::PodService => "DEVICE_TYPE_POD_SERVICE",
            DeviceType::RedisInstance => "DEVICE_TYPE_REDIS_INSTANCE",
            DeviceType::RdsInstance => "DEVICE_TYPE_RDS_INSTANCE",
            DeviceType::PodNode => "DEVICE_TYPE_POD_NODE",
            DeviceType::LoadBalance => "DEVICE_TYPE_LOAD_BALANCE",
            DeviceType::NatGateway => "DEVICE_TYPE_NAT_GATEWAY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEVICE_TYPE_UNKNOWN" => Some(Self::Unknown),
            "DEVICE_TYPE_VM" => Some(Self::Vm),
            "DEVICE_TYPE_VGW" => Some(Self::Vgw),
            "DEVICE_TYPE_THIRD_PARTY_DEVICE" => Some(Self::ThirdPartyDevice),
            "DEVICE_TYPE_VMWAF" => Some(Self::Vmwaf),
            "DEVICE_TYPE_NSP_VGATEWAY" => Some(Self::NspVgateway),
            "DEVICE_TYPE_HOST_DEVICE" => Some(Self::HostDevice),
            "DEVICE_TYPE_NETWORK_DEVICE" => Some(Self::NetworkDevice),
            "DEVICE_TYPE_DHCP_PORT" => Some(Self::DhcpPort),
            "DEVICE_TYPE_POD" => Some(Self::Pod),
            "DEVICE_TYPE_POD_SERVICE" => Some(Self::PodService),
            "DEVICE_TYPE_REDIS_INSTANCE" => Some(Self::RedisInstance),
            "DEVICE_TYPE_RDS_INSTANCE" => Some(Self::RdsInstance),
            "DEVICE_TYPE_POD_NODE" => Some(Self::PodNode),
            "DEVICE_TYPE_LOAD_BALANCE" => Some(Self::LoadBalance),
            "DEVICE_TYPE_NAT_GATEWAY" => Some(Self::NatGateway),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GroupType {
    Named = 0,
    Anonymous = 1,
}
impl GroupType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GroupType::Named => "NAMED",
            GroupType::Anonymous => "ANONYMOUS",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "NAMED" => Some(Self::Named),
            "ANONYMOUS" => Some(Self::Anonymous),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CidrType {
    Wan = 1,
    Lan = 2,
}
impl CidrType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CidrType::Wan => "WAN",
            CidrType::Lan => "LAN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "WAN" => Some(Self::Wan),
            "LAN" => Some(Self::Lan),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Action {
    /// 包存储（pcap）
    PacketCapturing = 1,
}
impl Action {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Action::PacketCapturing => "PACKET_CAPTURING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PACKET_CAPTURING" => Some(Self::PacketCapturing),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PacketCaptureSide {
    Src = 1,
    Dst = 2,
    Both = 3,
}
impl PacketCaptureSide {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PacketCaptureSide::Src => "SRC",
            PacketCaptureSide::Dst => "DST",
            PacketCaptureSide::Both => "BOTH",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SRC" => Some(Self::Src),
            "DST" => Some(Self::Dst),
            "BOTH" => Some(Self::Both),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TunnelType {
    Vxlan = 0,
    GreErspan = 1,
    Pcap = 2,
    NpbDrop = 3,
}
impl TunnelType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TunnelType::Vxlan => "VXLAN",
            TunnelType::GreErspan => "GRE_ERSPAN",
            TunnelType::Pcap => "PCAP",
            TunnelType::NpbDrop => "NPB_DROP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VXLAN" => Some(Self::Vxlan),
            "GRE_ERSPAN" => Some(Self::GreErspan),
            "PCAP" => Some(Self::Pcap),
            "NPB_DROP" => Some(Self::NpbDrop),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Direction {
    All = 1,
    Forward = 2,
    Backward = 3,
}
impl Direction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Direction::All => "ALL",
            Direction::Forward => "FORWARD",
            Direction::Backward => "BACKWARD",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ALL" => Some(Self::All),
            "FORWARD" => Some(Self::Forward),
            "BACKWARD" => Some(Self::Backward),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PluginType {
    Wasm = 1,
    So = 2,
}
impl PluginType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PluginType::Wasm => "WASM",
            PluginType::So => "SO",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "WASM" => Some(Self::Wasm),
            "SO" => Some(Self::So),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RoleType {
    RoleNone = 0,
    RoleClient = 1,
    RoleServer = 2,
}
impl RoleType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            RoleType::RoleNone => "ROLE_NONE",
            RoleType::RoleClient => "ROLE_CLIENT",
            RoleType::RoleServer => "ROLE_SERVER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ROLE_NONE" => Some(Self::RoleNone),
            "ROLE_CLIENT" => Some(Self::RoleClient),
            "ROLE_SERVER" => Some(Self::RoleServer),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ServiceProtocol {
    Any = 0,
    TcpService = 1,
    UdpService = 2,
}
impl ServiceProtocol {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ServiceProtocol::Any => "ANY",
            ServiceProtocol::TcpService => "TCP_SERVICE",
            ServiceProtocol::UdpService => "UDP_SERVICE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ANY" => Some(Self::Any),
            "TCP_SERVICE" => Some(Self::TcpService),
            "UDP_SERVICE" => Some(Self::UdpService),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum GpidSyncCompressAlgorithm {
    CompressAlroNone = 0,
    /// Before assigning a value to an array, we use the following algorithm to
    /// compress the original data:
    /// 1) Data is sorted by the order of fields in the entry of the array. The
    ///     order of the fields is carefully designed. The more likely a field is
    ///     repeated, the higher its order will be.
    /// 2) For each entry, if one of its fields is equal to the corresponding
    ///     field of the previous entry, then its field is assigned a value of 0.
    ///     When a field in Protobuf have a default value of 0, this allows us
    ///     to achieve very efficient compression.
    CompressAlgoIgnoreIfEqualToPrevious = 1,
}
impl GpidSyncCompressAlgorithm {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            GpidSyncCompressAlgorithm::CompressAlroNone => "COMPRESS_ALRO_NONE",
            GpidSyncCompressAlgorithm::CompressAlgoIgnoreIfEqualToPrevious => {
                "COMPRESS_ALGO_IGNORE_IF_EQUAL_TO_PREVIOUS"
            }
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "COMPRESS_ALRO_NONE" => Some(Self::CompressAlroNone),
            "COMPRESS_ALGO_IGNORE_IF_EQUAL_TO_PREVIOUS" => {
                Some(Self::CompressAlgoIgnoreIfEqualToPrevious)
            }
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OutputFormat {
    Text = 0,
    Binary = 1,
}
impl OutputFormat {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OutputFormat::Text => "TEXT",
            OutputFormat::Binary => "BINARY",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TEXT" => Some(Self::Text),
            "BINARY" => Some(Self::Binary),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CommandType {
    Linux = 0,
    Kubernetes = 1,
}
impl CommandType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CommandType::Linux => "LINUX",
            CommandType::Kubernetes => "KUBERNETES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LINUX" => Some(Self::Linux),
            "KUBERNETES" => Some(Self::Kubernetes),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ParamType {
    PfText = 0,
    PfBoolean = 1,
}
impl ParamType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ParamType::PfText => "PF_TEXT",
            ParamType::PfBoolean => "PF_BOOLEAN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "PF_TEXT" => Some(Self::PfText),
            "PF_BOOLEAN" => Some(Self::PfBoolean),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ExecutionType {
    ListCommand = 0,
    ListNamespace = 1,
    RunCommand = 2,
}
impl ExecutionType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ExecutionType::ListCommand => "LIST_COMMAND",
            ExecutionType::ListNamespace => "LIST_NAMESPACE",
            ExecutionType::RunCommand => "RUN_COMMAND",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LIST_COMMAND" => Some(Self::ListCommand),
            "LIST_NAMESPACE" => Some(Self::ListNamespace),
            "RUN_COMMAND" => Some(Self::RunCommand),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod synchronizer_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct SynchronizerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SynchronizerClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SynchronizerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SynchronizerClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            SynchronizerClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn sync(
            &mut self,
            request: impl tonic::IntoRequest<super::SyncRequest>,
        ) -> Result<tonic::Response<super::SyncResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/agent.Synchronizer/Sync");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn push(
            &mut self,
            request: impl tonic::IntoRequest<super::SyncRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::SyncResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/agent.Synchronizer/Push");
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn upgrade(
            &mut self,
            request: impl tonic::IntoRequest<super::UpgradeRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::UpgradeResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agent.Synchronizer/Upgrade",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        pub async fn query(
            &mut self,
            request: impl tonic::IntoRequest<super::NtpRequest>,
        ) -> Result<tonic::Response<super::NtpResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/agent.Synchronizer/Query");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn genesis_sync(
            &mut self,
            request: impl tonic::IntoRequest<super::GenesisSyncRequest>,
        ) -> Result<tonic::Response<super::GenesisSyncResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agent.Synchronizer/GenesisSync",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn kubernetes_api_sync(
            &mut self,
            request: impl tonic::IntoRequest<super::KubernetesApiSyncRequest>,
        ) -> Result<tonic::Response<super::KubernetesApiSyncResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agent.Synchronizer/KubernetesAPISync",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_kubernetes_cluster_id(
            &mut self,
            request: impl tonic::IntoRequest<super::KubernetesClusterIdRequest>,
        ) -> Result<tonic::Response<super::KubernetesClusterIdResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agent.Synchronizer/GetKubernetesClusterID",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn gpid_sync(
            &mut self,
            request: impl tonic::IntoRequest<super::GpidSyncRequest>,
        ) -> Result<tonic::Response<super::GpidSyncResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agent.Synchronizer/GPIDSync",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn plugin(
            &mut self,
            request: impl tonic::IntoRequest<super::PluginRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::PluginResponse>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agent.Synchronizer/Plugin",
            );
            self.inner.server_streaming(request.into_request(), path, codec).await
        }
        /// because gRPC cannot be initiated by server, the req/resp of this rpc is reversed
        pub async fn remote_execute(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::RemoteExecResponse,
            >,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::RemoteExecRequest>>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agent.Synchronizer/RemoteExecute",
            );
            self.inner.streaming(request.into_streaming_request(), path, codec).await
        }
    }
}
/// Generated client implementations.
pub mod debug_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// debug service
    #[derive(Debug, Clone)]
    pub struct DebugClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl DebugClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> DebugClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> DebugClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            DebugClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        pub async fn debug_gpid_global_data(
            &mut self,
            request: impl tonic::IntoRequest<super::GpidSyncRequest>,
        ) -> Result<tonic::Response<super::GpidGlobalData>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agent.Debug/DebugGPIDGlobalData",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn debug_gpid_agent_data(
            &mut self,
            request: impl tonic::IntoRequest<super::GpidSyncRequest>,
        ) -> Result<tonic::Response<super::GpidAgentData>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agent.Debug/DebugGPIDAgentData",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn debug_real_global_data(
            &mut self,
            request: impl tonic::IntoRequest<super::GpidSyncRequest>,
        ) -> Result<tonic::Response<super::RealGlobalData>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agent.Debug/DebugRealGlobalData",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn debug_rip_to_vip(
            &mut self,
            request: impl tonic::IntoRequest<super::GpidSyncRequest>,
        ) -> Result<tonic::Response<super::RvData>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agent.Debug/DebugRIPToVIP",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn debug_agent_cache(
            &mut self,
            request: impl tonic::IntoRequest<super::AgentCacheRequest>,
        ) -> Result<tonic::Response<super::AgentCacheResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/agent.Debug/DebugAgentCache",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
