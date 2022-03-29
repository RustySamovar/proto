use std::io::Result;

fn main() -> Result<()> {
    let proto_dir = "protobuf";

    let protos = vec![
        // Dispatch protocol
        "QueryRegionListHttpRsp",
        "QueryCurrRegionHttpRsp",
        "RegionSimpleInfo",
        "RegionInfo",

        // Game protocol
        "packet_header",
        // Different game enumerations
        "PlatformType",
        //"GrowCurveType",
        "ArithType",

        "UnionCmdNotify",
        "GetPlayerTokenReq",
        "GetPlayerTokenRsp",
        "PlayerLoginReq",
        "OpenStateUpdateNotify",
        "StoreWeightLimitNotify",
        "PlayerStoreNotify",
        "AvatarDataNotify",
        "PlayerEnterSceneNotify",
        "PlayerLoginRsp",
        "EnterSceneReadyReq",
        "EnterSceneReadyRsp",
        "SceneInitFinishReq",
        "EnterScenePeerNotify",
        "WorldDataNotify",
        "WorldPlayerInfoNotify",
        "ScenePlayerInfoNotify",
        "PlayerEnterSceneInfoNotify",
        "PlayerGameTimeNotify",
        "SceneTimeNotify",
        "SceneDataNotify",
        "HostPlayerNotify",
        "SceneTeamUpdateNotify",
        "SceneInitFinishRsp",
        "EnterSceneDoneReq",
        "SceneEntityAppearNotify",
        "SceneEntityDisappearNotify",
        "EnterSceneDoneRsp",
        "PostEnterSceneReq",
        "PostEnterSceneRsp",
        
        "WorldPlayerRTTNotify",
        "PingReq",
        "PingRsp",
        "PlayerDataNotify",

        "EnterWorldAreaReq",
        "EnterWorldAreaRsp",

        "CombatInvocationsNotify",

        "NpcTalkReq",
        "NpcTalkRsp",

        "GetShopReq",
        "GetShopRsp",

        "BuyGoodsReq",
        "BuyGoodsRsp",

        "GetSceneAreaReq",
        "GetSceneAreaRsp",
        "GetScenePointReq",
        "GetScenePointRsp",

        "PlayerSetPauseReq",
        "PlayerSetPauseRsp",

        "GetPlayerSocialDetailReq",
        "GetPlayerSocialDetailRsp",
        "GetPlayerBlacklistReq",
        "GetPlayerBlacklistRsp",
        "GetPlayerFriendListReq",
        "GetPlayerFriendListRsp",

        "StoreItemChangeNotify",
        "StoreItemDelNotify",
        "ItemAddHintNotify",

        // Internal CIN data
        "EntityMoveInfo",
        "EvtSetAttackTargetInfo",
        "EvtFaceToDirInfo",
        "EvtBeingHitInfo",

        "OpenStateType",
        "FightPropType",
        "PropType",
        "ActionReasonType",
    ];

    let annotated_enums = vec![
        "GrowCurveType",
        "ArithType",
        "FightPropType",
    ];

    let protos: Vec<String> = protos.iter().map(|&x| format!("{}/{}.proto", proto_dir, x)).collect();

    let mut config = prost_build::Config::new();

    config.type_attribute(".", "#[derive(serde::Deserialize)]");

    for e in annotated_enums.iter() {
        // This doesn't add but OVERRIDE type attribute
        // Very confusing
        config.type_attribute(format!(".Proto.{}", e), "#[derive(serde::Deserialize)] #[serde(rename_all = \"SCREAMING_SNAKE_CASE\")]");
    }

    let ret = config.compile_protos(&protos, &[format!("{}/", proto_dir)]);

    match ret {
        Ok(_) => return Ok(()),
        Err(e) => panic!("{}", e),
    }
}
