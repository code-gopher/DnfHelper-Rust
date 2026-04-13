//! 游戏地址常量模块
//!
//! 定义游戏内存地址常量，与 Java Address.java 完全对应

/// 游戏内存地址常量
pub struct Address;

impl Address {
    // ==================== 人物相关地址 ====================

    /// 新人物基址 (RwAddr)
    pub const RW_ADDR: i64 = 0x14C2503F8;

    /// 人物基址 (RwAddr1)
    pub const RW_ADDR_1: i64 = 0x14C2503F0;

    /// 人物基址 B (RwAddr2)
    pub const RW_ADDR_2: i64 = 0x14B8E1D08;

    /// 人物 CALL (RWCallAddr)
    pub const RW_CALL_ADDR: i64 = 0x144CA54B0;

    /// 角色等级 (JSDjAddr)
    pub const JS_DJ_ADDR: i64 = 0x14B957BE0;

    /// 评分基址 (PFAddr)
    pub const PF_ADDR: i64 = 0x14B971380;

    /// 公告参数 (GGCsAddr)
    pub const GG_CS_ADDR: i64 = 0x14C251758;

    /// 公告 CALL (GGCallAddr)
    pub const GG_CALL_ADDR: i64 = 0x144D712C0;

    /// 背包基址 (BbJzAddr)
    pub const BB_JZ_ADDR: i64 = 0x14B972668;

    /// 角色指针 (JSPtrAddr)
    pub const JS_PTR_ADDR: i64 = 0x14B9723B0;

    /// 城镇大区域 (CzDqyAddr)
    pub const CZ_DQY_ADDR: i64 = 0x14B92F7AC;

    /// 城镇小区域 (CzXqyAddr)
    pub const CZ_XQY_ADDR: i64 = 0x14B92F7B0;

    /// 游戏状态 (YXZTAddr)
    pub const YX_ZT_ADDR: i64 = 0x14B4B3F20;

    /// 司南背包 (SNBBAddr)
    pub const SN_BB_ADDR: i64 = 0x14B9726C0;

    /// 玉荣背包 (YrBbAddr)
    pub const YR_BB_ADDR: i64 = 0x14B9726B8;

    /// 辟邪玉背包 (BxrBbAddr)
    pub const BXR_BB_ADDR: i64 = 0x14B9726B8;

    // ==================== CALL 地址 ====================

    /// 司南添加 CALL (SnAddCallAddr)
    pub const SN_ADD_CALL_ADDR: i64 = 0x141DDF8D0;

    /// 司南进图_Rcx (SnJtRcxAddr)
    pub const SN_JT_RCX_ADDR: i64 = 0x14B920068;

    /// 司南进图 CALL (SnJtCallAddr)
    pub const SN_JT_CALL_ADDR: i64 = 0x141DC2C00;

    /// 取司南添加 RCX (SnAddRcxAddr)
    pub const SN_ADD_RCX_ADDR: i64 = 0x145460CF0;

    /// 玉荣力偏移 (YrlPyAddr)
    pub const YRL_PY_ADDR: i64 = 0x600;

    /// 角色玉荣力 (JsYrlAddr)
    pub const JS_YRL_ADDR: i64 = 0x5408;

    /// 汇编 CALL (HBCallAddr)
    pub const HB_CALL_ADDR: i64 = 0x13FDC0000;

    /// TranslateMessage
    pub const TRANSLATE_MESSAGE: i64 = 0x1477C6CC0;

    /// GameTimeGetTime
    pub const GAME_TIME_GET_TIME: i64 = 0x1477C70F8;

    /// 技能 CALL (JNCallAddr)
    pub const JN_CALL_ADDR: i64 = 0x14480BB70;

    /// 聚物 CALL (JwCallAddr)
    pub const JW_CALL_ADDR: i64 = 0x144AA4D10;

    /// 聚物校验 (JwXyAddr)
    pub const JW_XY_ADDR: i64 = 0xFE24;

    /// 任务基址 (TaskAddr)
    pub const TASK_ADDR: i64 = 0x14B972750;

    /// 接受 CALL (JsCallAddr)
    pub const JS_CALL_ADDR: i64 = 0x1440FCD40;

    /// 完成 CALL (WcCallAddr)
    pub const WC_CALL_ADDR: i64 = 0x1440FD350;

    /// 提交 CALL (TjCallAddr)
    pub const TJ_CALL_ADDR: i64 = 0x1440FCE30;

    /// 跳过 CALL (TgCallAddr)
    pub const TG_CALL_ADDR: i64 = 0x143E98F50;

    /// 按键基址 (AjAddr)
    pub const AJ_ADDR: i64 = 0x14C70EC10;

    /// 对话基址 (DHAddr)
    pub const DH_ADDR: i64 = 0x14C2A42E8;

    /// 对话基址 B (DHAddrB)
    pub const DH_ADDR_B: i64 = 0x14B7A84C0;

    /// Esc 对话基址 (EscDHAddr)
    pub const ESC_DH_ADDR: i64 = 0x14B7A84E0;

    /// 翻牌基址 (FpAddr)
    pub const FP_ADDR: i64 = 0x14B972660;

    /// 副本编号 (FbBhAddr)
    pub const FB_BH_ADDR: i64 = 0x14B957B70;

    /// 时间基址 (SJAddr)
    pub const SJ_ADDR: i64 = 0x20A050;

    /// 房间编号 (FJBHAddr)
    pub const FJ_BH_ADDR: i64 = 0x14B972650;

    /// 最大疲劳 (MaxPlAddr)
    pub const MAX_PL_ADDR: i64 = 0x14C25032C;

    /// 当前疲劳 (CutPlAddr)
    pub const CUT_PL_ADDR: i64 = 0x14C25039C;

    /// 区域参数 (QyParamAddr)
    pub const QY_PARAM_ADDR: i64 = 0x14C2A9830;

    /// 区域 CALL (QyCallAddr)
    pub const QY_CALL_ADDR: i64 = 0x145AB95C0;

    /// 区域偏移 (QyPyAddr)
    pub const QY_PY_ADDR: i64 = 0xA9FA8;

    /// 选图 CALL (XTuCallAddr)
    pub const X_TU_CALL_ADDR: i64 = 0x145AF90E0;

    /// 进图 CALL (JTuCallAddr)
    pub const J_TU_CALL_ADDR: i64 = 0x145B398F0;

    /// 回城 CALL (HChengCallAddr)
    pub const H_CHENG_CALL_ADDR: i64 = 0x14589EE90;

    /// 过图 CALL (GtCallAddr)
    pub const GT_CALL_ADDR: i64 = 0x143C32B80;

    /// 漂移 CALL (PyCall1Addr)
    pub const PY_CALL_1_ADDR: i64 = 0x143A84420;

    /// 漂移 CALL2 (PyCall2Addr)
    pub const PY_CALL_2_ADDR: i64 = 0x145C531D0;

    /// 奔跑 CALL (BpCallAddr)
    pub const BP_CALL_ADDR: i64 = 0x14403E760;

    /// 写入内存 (XrNcCallAddr)
    pub const XR_NC_CALL_ADDR: i64 = 0x144CE04F0;

    /// 奔跑偏移_1 (BpPyAddr1)
    pub const BP_PY_ADDR_1: i64 = 0x1208;

    /// 奔跑偏移_2 (BpPyAddr2)
    pub const BP_PY_ADDR_2: i64 = 0x11F0;

    /// 城镇瞬移_Rdx (CzSyRdxAddr)
    pub const CZ_SY_RDX_ADDR: i64 = 0x14B9435E8;

    /// 城镇瞬移 CALL (CzSyCallAddr)
    pub const CZ_SY_CALL_ADDR: i64 = 0x145B001A0;

    /// 选择角色 CALL (XzJsCallAddr)
    pub const XZ_JS_CALL_ADDR: i64 = 0x1404FD580;

    /// 返回角色 CALL (FhJsCallAddr)
    pub const FH_JS_CALL_ADDR: i64 = 0x144513860;

    /// 冷却判断 CALL (LqCallJudgeAddr)
    pub const LQ_CALL_JUDGE_ADDR: i64 = 0x144C91AE0;

    /// CD 重置 CALL (CdResetCallAddr)
    pub const CD_RESET_CALL_ADDR: i64 = 0x144AF47E0;

    /// 分解 CALL (FjCallAddr)
    pub const FJ_CALL_ADDR: i64 = 0x1448F0FB0;

    /// 整理 CALL (ZlCallAddr)
    pub const ZL_CALL_ADDR: i64 = 0x1448E73A0;

    /// 当前负重 (DqFzAddr)
    pub const DQ_FZ_ADDR: i64 = 0x14C2A5B38;

    /// 最大负重 (ZdFzAddr)
    pub const ZD_FZ_ADDR: i64 = 0x2DB8;

    // ==================== 发包相关地址 ====================

    /// 发包基址 (FbAddr)
    pub const FB_ADDR: i64 = 0x14C2AA440;

    /// 缓冲 CALL (HcCallAddr)
    pub const HC_CALL_ADDR: i64 = 0x145B64E70;

    /// 发包 CALL (FbCallAddr)
    pub const FB_CALL_ADDR: i64 = 0x145B65B60;

    /// 加密包 CALL (JmB1CallAddr)
    pub const JM_B1_CALL_ADDR: i64 = 0x145B65CD0;

    /// 加密包 CALL2 (JmB2CallAddr)
    pub const JM_B2_CALL_ADDR: i64 = 0x145B66050;

    /// 加密包 CALL4 (JmB3CallAddr)
    pub const JM_B3_CALL_ADDR: i64 = 0x145B65CF0;

    /// 加密包 CALL8 (JmB4CallAddr)
    pub const JM_B4_CALL_ADDR: i64 = 0x145B65D10;

    /// 申请内存 (SqNcCallAddr)
    pub const SQ_NC_CALL_ADDR: i64 = 0x143A59560;

    /// BUFF 内存_RCX (BUffMemRcxAddr)
    pub const BUFF_MEM_RCX_ADDR: i64 = 0x14B9725A8;

    /// BUFF 内存 CALL (BUffMemCallAddr)
    pub const BUFF_MEM_CALL_ADDR: i64 = 0x145B81B80;

    /// 调用 BUFFCALL (DyBuffCall)
    pub const DY_BUFF_CALL: i64 = 0x144CDC830;

    /// 生效 CALL (TakeEffectCallAddr)
    pub const TAKE_EFFECT_CALL_ADDR: i64 = 0x144A19C20;

    /// 穿上 CALL (PutOnCallAddr)
    pub const PUT_ON_CALL_ADDR: i64 = 0x144AB7A70;

    /// 透明 CALL (TmCallAddr)
    pub const TM_CALL_ADDR: i64 = 0x145B93BA0;

    /// 创建 CALL (CreateCallAddr)
    pub const CREATE_CALL_ADDR: i64 = 0x144DBCF00;

    /// 物品移动 CALL (WpYdCallAddr)
    pub const WP_YD_CALL_ADDR: i64 = 0x1448DDA40;

    /// 技能三无 (JnSwAddr)
    pub const JN_SW_ADDR: i64 = 0x144A605C1;

    // ==================== 偏移量 ====================

    /// 人物名望 (RwMwAddr)
    pub const RW_MW_ADDR: i64 = 0x11E54;

    /// 物品名称 (WpMcAddr)
    pub const WP_MC_ADDR: i64 = 0x40;

    /// 物品交易类型 (WpJyLxAddr)
    pub const WP_JY_LX_ADDR: i64 = 0xA8;

    /// 动作 ID (DzIDAddr)
    pub const DZ_ID_ADDR: i64 = 0x436C;

    /// 地图开始 2 (DtKs2)
    pub const DT_KS_2: i64 = 0x1B8;

    /// 地图结束 2 (DtJs2)
    pub const DT_JS_2: i64 = 0x1C0;

    /// 地图偏移 (DtPyAddr)
    pub const DT_PY_ADDR: i64 = 0x168;

    /// 类型偏移 (LxPyAddr)
    pub const LX_PY_ADDR: i64 = 0x134;

    /// 方向偏移 (FxPyAddr)
    pub const FX_PY_ADDR: i64 = 0x148;

    /// 评分偏移 (CEPfAddr)
    pub const CE_PF_ADDR: i64 = 0x88;

    /// 发包拾取 (FbSqAddr)
    pub const FB_SQ_ADDR: i64 = 0x13C;

    /// 怪物血量 (GwXlAddr)
    pub const GW_XL_ADDR: i64 = 0x4F78;

    /// 阵营偏移 (ZyPyAddr)
    pub const ZY_PY_ADDR: i64 = 0xEB8;

    /// 地面物品 (DmWpAddr)
    pub const DM_WP_ADDR: i64 = 0x2B70;

    /// 脚下物品 (JxWpAddr)
    pub const JX_WP_ADDR: i64 = 0xF950;

    /// 代码偏移 (DmPyAddr)
    pub const DM_PY_ADDR: i64 = 0x868;

    /// 名称偏移 (McPyAddr)
    pub const MC_PY_ADDR: i64 = 0x870;

    /// 装备品级 (ZbPjAddr)
    pub const ZB_PJ_ADDR: i64 = 0x2B8;

    /// 地图穿透 (DtCtAddr)
    pub const DT_CT_ADDR: i64 = 0x878;

    /// 建筑穿透 (JzCtAddr)
    pub const JZ_CT_ADDR: i64 = 0x87C;

    /// 读取坐标 (DqZbAddr)
    pub const DQ_ZB_ADDR: i64 = 0x328;

    /// 已接任务首地址 (YjRwStartAddr)
    pub const YJ_RW_START_ADDR: i64 = 0x10;

    /// 已接任务尾地址 (YjRwEndAddr)
    pub const YJ_RW_END_ADDR: i64 = 0x18;

    /// 全部任务首地址 (QbRwStartAddr)
    pub const QB_RW_START_ADDR: i64 = 0xA8;

    /// 全部任务尾地址 (QbRwEndAddr)
    pub const QB_RW_END_ADDR: i64 = 0xB0;

    /// 任务类型 (RwLxAddr)
    pub const RW_LX_ADDR: i64 = 0x218;

    /// 任务大小 (RwDxAddr)
    pub const RW_DX_ADDR: i64 = 0x28;

    /// 任务条件 (RwTjAddr)
    pub const RW_TJ_ADDR: i64 = 0x4D0;

    /// 任务等级 (RwDjAddr)
    pub const RW_DJ_ADDR: i64 = 0x328;

    /// 任务副本 (RwFbAddr)
    pub const RW_FB_ADDR: i64 = 0x488;

    /// 是否开门 (SfKmAddr)
    pub const SF_KM_ADDR: i64 = 0x27C;

    /// 当前房间 X (CutRoomXAddr)
    pub const CUT_ROOM_X_ADDR: i64 = 0x1C98;

    /// 当前房间 Y (CutRoomYAddr)
    pub const CUT_ROOM_Y_ADDR: i64 = 0x1C9C;

    /// BOSS 房间 X (BOSSRoomXAddr)
    pub const BOSS_ROOM_X_ADDR: i64 = 0x1D98;

    /// BOSS 房间 Y (BOSSRoomYAddr)
    pub const BOSS_ROOM_Y_ADDR: i64 = 0x1D9C;

    /// 篝火判断 (GouHuoAddr)
    pub const GOU_HUO_ADDR: i64 = 0x1E28;

    /// 索引偏移 (SyPyAddr)
    pub const SY_PY_ADDR: i64 = 0x1D8C;

    /// 门型偏移 (MxPyAddr)
    pub const MX_PY_ADDR: i64 = 0x128;

    /// 宽高偏移 (KgPyAddr)
    pub const KG_PY_ADDR: i64 = 0x890;

    /// 数组偏移 (SzPyAddr)
    pub const SZ_PY_ADDR: i64 = 0x8B0;

    /// 地图名称 (DtMcAddr)
    pub const DT_MC_ADDR: i64 = 0x418;

    /// 顺图偏移 (StPyAddr)
    pub const ST_PY_ADDR: i64 = 0xC0;

    /// 坐标顺图 (ZbStPyAddr)
    pub const ZB_ST_PY_ADDR: i64 = 0x3848;

    /// 方向 ID (FxIdAddr)
    pub const FX_ID_ADDR: i64 = 0xE8;

    /// 物品栏 (WplAddr)
    pub const WPL_ADDR: i64 = 0xFD98;

    /// 物品栏偏移 (WplPyAddr)
    pub const WPL_PY_ADDR: i64 = 0xA8;

    /// 技能栏 (JnlAddr)
    pub const JNL_ADDR: i64 = 0xFD10;

    /// 技能栏偏移 (JnlPyAddr)
    pub const JNL_PY_ADDR: i64 = 0x90;

    // ==================== 动态空白地址 (运行时初始化) ====================

    /// 人物空白地址 (RwKbAddr) - 运行时初始化
    pub static mut RW_KB_ADDR: i64 = 0;

    /// 内存汇编空白地址 (NcBhKbAddr) - 运行时初始化
    pub static mut NC_BH_KB_ADDR: i64 = 0;

    /// 技能 Call 空白地址 (JnKbAddr) - 运行时初始化
    pub static mut JN_KB_ADDR: i64 = 0;

    /// 过图 Call 空白地址 (GtKbAddr) - 运行时初始化
    pub static mut GT_KB_ADDR: i64 = 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_address_constants() {
        // 验证主要地址不为 0
        assert!(Address::RW_ADDR > 0);
        assert!(Address::RW_CALL_ADDR > 0);
        assert!(Address::JN_CALL_ADDR > 0);
        assert!(Address::FB_ADDR > 0);
    }
}
