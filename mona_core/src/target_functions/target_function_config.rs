use serde::{Deserialize, Serialize};

use crate::common::{Element, SkillType};
use crate::target_functions::target_functions::cryo::rosaria_default::RosariaDefaultTFConfig;

#[derive(Serialize, Deserialize)]
pub enum TargetFunctionConfig {
    PyroDamage { t: usize },
    CryoDamage { t: usize },
    HydroDamage { t: usize },
    ElectroDamage { t: usize },
    AnemoDamage { t: usize },
    GeoDamage { t: usize },
    DendroDamage { t: usize },
    PhysicalDamage { t: usize },
    MaxVaporize { t: usize, skill: SkillType },
    MaxMelt { t: usize, skill: SkillType },
    ExpectVaporize { t: usize, skill: SkillType },
    ExpectMelt { t: usize, skill: SkillType },

    BennettDefault { recharge_demand: f64 },
    GanyuDefault { melt_rate: f64 },
    GorouDefault { recharge_demand: f64 },
    DilucDefault { melt_rate: f64, vaporize_rate: f64 },
    DionaDefault { recharge_demand: f64 },
    // ExpectationConfig { element: Element, skill_type: SkillType }
    HuTaoDefault { vaporize_rate: f64, melt_rate: f64 },
    JeanDefault { recharge_demand: f64, damage_weight: f64 },
    KaedeharaKazuhaDamage { recharge_demand: f64, other_dmg_ratio: f64, swirl_rate: f64 },
    KaedeharaKazuhaDefault { recharge_demand: f64 },
    KamisatoAyakaDefault { recharge_demand: f64 },
    KleeDefault { recharge_demand: f64 },
    LisaDefault { recharge_demand: f64 },
    MonaDefault { recharge_demand: f64 },
    QiqiDefault { recharge_demand: f64 },
    RaidenShogunDefault { recharge_demand: f64 },
    RosariaDefault(RosariaDefaultTFConfig),
    SayuDefault { recharge_demand: f64 },
    ShenheDefault { recharge_demand: f64 },
    SucroseDefault { recharge_demand: f64 },
    ThomaDefault { recharge_demand: f64 },
    VentiDefault { swirl_rate: f64 },
    XianglingDefault { recharge_demand: f64, melt_rate: f64, vaporize_rate: f64, overload_rate: f64 },
    XingqiuDefault { recharge_demand: f64 },
    XinyanDefault { recharge_demand: f64, damage_demand: f64 },
    YaeMikoDefault { recharge_demand: f64, electro_charged_times: f64, overload_times: f64 },
    YelanDefault { recharge_demand: f64, vaporize_rate: f64 },
    YoimiyaDefault { vaporize_rate: f64, melt_rate: f64 },
    YunjinDefault { recharge_demand: f64 },
    ZhongliDefault { recharge_demand: f64 },
    KujouSaraDamage { recharge_demand: f64 },
    KukiShinobuDefault { e_ratio: f64 },
    TighnariDefault { spread_rate: f64 },
    CynoDefault { recharge_requirement: f64, combo: usize, until_expire: bool, aggravate_rate: f64, elecharged_rate: f64, overload_rate: f64, hyperbloom_rate: f64 },
    KeqingDefault { aggravate_rate: f64 },
    NilouDefault { e_ratio: f64, q_ratio: f64, bloom_ratio: f64, other_em: f64, other_bloom_ratio: f64 },
    NahidaDefault { em_requirement: usize, spread_rate: f64, bloom_count: f64, burn_duration: f64, pryo_teammate_count: usize },
    WandererDefault { e_hydro: bool, e_pyro: bool, e_cryo: bool, spd_extra: f64, spd_comp: f64, dash_count: usize, q_count:usize, swirl_count: usize,  },

    BennettDamage { recharge_demand: f64, other_dmg_ratio: f64 },
    FaruzanDamage { recharge_demand: f64 },

    NoConfig,
}
