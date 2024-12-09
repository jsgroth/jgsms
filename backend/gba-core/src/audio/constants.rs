#![allow(clippy::excessive_precision)]

pub const SOURCE_FREQUENCY: f64 = 262_144.0;

pub const LPF_TAPS: usize = 99;

// Generated for source frequency of 262144 Hz (max PWM sample cycle)
pub const LPF_COEFFICIENTS: [f64; LPF_TAPS] = [
    1.339821233396029e-05,
    -0.0001882002256060123,
    -0.000383415775882795,
    -0.0005522781755406921,
    -0.0006696855586518925,
    -0.0007065521515551827,
    -0.0006341085870343052,
    -0.0004310901334356564,
    -9.252455941566792e-05,
    0.0003620153056227914,
    0.0008832531944451171,
    0.001392533719668835,
    0.001789202667173295,
    0.00196525080873308,
    0.001825745957832981,
    0.001311914011858384,
    0.0004224610242009128,
    -0.0007718045362731821,
    -0.002124558845385507,
    -0.003424817097699539,
    -0.004422788852816747,
    -0.004868614237662881,
    -0.004558969954247713,
    -0.0033841661286696,
    -0.001367036412362484,
    0.001314957869980491,
    0.004331271825792213,
    0.007227133641867067,
    0.009479100500086818,
    0.01057198222976309,
    0.01008742430551465,
    0.007791204245946147,
    0.003704870929300158,
    -0.001851833923270376,
    -0.008258212901927775,
    -0.01463482761384264,
    -0.01992955333575138,
    -0.02303948284253261,
    -0.02295450771653119,
    -0.01890392686673801,
    -0.01048547500442246,
    0.002242808534408131,
    0.01872342407237073,
    0.03792195586007354,
    0.05841207266190167,
    0.07851501745340571,
    0.09647769244770496,
    0.1106675250144708,
    0.1197593212594183,
    0.1228897873667648,
    0.1197593212594183,
    0.1106675250144708,
    0.09647769244770496,
    0.07851501745340574,
    0.05841207266190168,
    0.03792195586007354,
    0.01872342407237073,
    0.002242808534408132,
    -0.01048547500442246,
    -0.01890392686673801,
    -0.02295450771653119,
    -0.02303948284253261,
    -0.01992955333575138,
    -0.01463482761384264,
    -0.008258212901927775,
    -0.001851833923270376,
    0.003704870929300158,
    0.007791204245946147,
    0.01008742430551465,
    0.0105719822297631,
    0.00947910050008682,
    0.007227133641867064,
    0.004331271825792212,
    0.001314957869980491,
    -0.001367036412362485,
    -0.003384166128669603,
    -0.004558969954247718,
    -0.004868614237662882,
    -0.00442278885281675,
    -0.003424817097699539,
    -0.002124558845385507,
    -0.0007718045362731825,
    0.0004224610242009132,
    0.001311914011858384,
    0.001825745957832983,
    0.00196525080873308,
    0.001789202667173298,
    0.001392533719668838,
    0.0008832531944451186,
    0.0003620153056227911,
    -9.252455941566793e-05,
    -0.0004310901334356565,
    -0.0006341085870343056,
    -0.0007065521515551831,
    -0.0006696855586518927,
    -0.000552278175540692,
    -0.0003834157758827953,
    -0.0001882002256060123,
    1.33982123339603e-05,
];
