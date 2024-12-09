pub const HPF_CHARGE_FACTOR: f64 = 0.9946028448191855;

pub const LPF_TAPS: usize = 59;

// (4 + 1) * ~32040 Hz = ~160200 Hz
pub const ZERO_PADDING: u32 = 4;

// Generated using cutoff frequency of 16000 Hz (in practice closer to 15000 Hz)
#[allow(clippy::excessive_precision)]
pub const LPF_COEFFICIENTS: [f64; LPF_TAPS] = [
    -0.0005923818230584685,
    -0.0009206372737931884,
    -0.0009782934869905454,
    -0.0006425222408461217,
    0.0001502108256140527,
    0.001277861535230595,
    0.0023467994732787,
    0.002750146190105863,
    0.001913047741215444,
    -0.0003458598823977889,
    -0.003513865761438726,
    -0.006352822217744295,
    -0.0072621064618351,
    -0.004980358510688322,
    0.0006324820318355614,
    0.008112928903062732,
    0.01454913381103295,
    0.01648912714647353,
    0.01138725708461903,
    -0.0009280654518873831,
    -0.01740893263771281,
    -0.03207108697826812,
    -0.03752903447876763,
    -0.02740907910647264,
    0.001147896640758836,
    0.04582845843870548,
    0.09905708999166349,
    0.1496824176136328,
    0.1860066980262327,
    0.1992069817168787,
    0.1860066980262327,
    0.1496824176136328,
    0.09905708999166347,
    0.04582845843870548,
    0.001147896640758836,
    -0.02740907910647265,
    -0.03752903447876765,
    -0.03207108697826812,
    -0.01740893263771281,
    -0.0009280654518873834,
    0.01138725708461903,
    0.01648912714647354,
    0.01454913381103295,
    0.008112928903062734,
    0.0006324820318355617,
    -0.004980358510688323,
    -0.007262106461835107,
    -0.0063528222177443,
    -0.003513865761438725,
    -0.0003458598823977889,
    0.001913047741215443,
    0.002750146190105863,
    0.002346799473278699,
    0.001277861535230595,
    0.0001502108256140529,
    -0.0006425222408461217,
    -0.0009782934869905454,
    -0.000920637273793189,
    -0.0005923818230584685,
];
