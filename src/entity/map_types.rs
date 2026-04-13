//! 地图类型模块
//! 
//! 定义地图相关的枚举和结构体

/// 地图数据类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MapDataType {
    /// 未知
    Unknown,
    /// 城镇
    Town,
    /// 普通地图
    Normal,
    /// 副本
    Dungeon,
    /// Boss 房间
    BossRoom,
}

impl Default for MapDataType {
    fn default() -> Self {
        MapDataType::Unknown
    }
}

/// 地图遍历类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MapTraversalType {
    /// 顺序遍历
    Sequential,
    /// 逆序遍历
    Reverse,
    /// 随机遍历
    Random,
}

impl Default for MapTraversalType {
    fn default() -> Self {
        MapTraversalType::Sequential
    }
}

/// 坐标类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CoordinateType {
    /// 绝对坐标
    Absolute,
    /// 相对坐标
    Relative,
    /// 偏移坐标
    Offset,
}

impl Default for CoordinateType {
    fn default() -> Self {
        CoordinateType::Absolute
    }
}

/// 地图节点类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MapNodeType {
    /// 起点
    Start,
    /// 路径点
    Waypoint,
    /// 怪物点
    Monster,
    /// 物品点
    Item,
    /// Boss 点
    Boss,
    /// 传送门
    Portal,
}

impl Default for MapNodeType {
    fn default() -> Self {
        MapNodeType::Waypoint
    }
}

/// 游戏地图类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GameMapType {
    /// 格兰之森
    GelanForest,
    /// 天空之城
    SkyTower,
    /// 天帷巨兽
    Behemoth,
    /// 诺伊佩拉
    Noiera,
    /// 暗黑城
    DarkCity,
    /// 其他
    Other,
}

impl Default for GameMapType {
    fn default() -> Self {
        GameMapType::Other
    }
}

/// 地图坐标结构体
#[derive(Debug, Clone, Copy, Default)]
pub struct MapCoordinate {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl MapCoordinate {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
    
    /// 计算距离
    pub fn distance(&self, other: &MapCoordinate) -> f32 {
        let dx = (self.x - other.x) as f32;
        let dy = (self.y - other.y) as f32;
        let dz = (self.z - other.z) as f32;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

/// 地图节点结构体
#[derive(Debug, Clone)]
pub struct MapNode {
    pub node_type: MapNodeType,
    pub coordinate: MapCoordinate,
    pub visited: bool,
}

impl MapNode {
    pub fn new(node_type: MapNodeType, x: i32, y: i32, z: i32) -> Self {
        Self {
            node_type,
            coordinate: MapCoordinate::new(x, y, z),
            visited: false,
        }
    }
}

/// 地图数据路由
#[derive(Debug, Clone, Default)]
pub struct MapRoute {
    pub nodes: Vec<MapNode>,
    pub current_index: usize,
}

impl MapRoute {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn add_node(&mut self, node: MapNode) {
        self.nodes.push(node);
    }
    
    pub fn get_current(&self) -> Option<&MapNode> {
        self.nodes.get(self.current_index)
    }
    
    pub fn next(&mut self) -> Option<&MapNode> {
        if self.current_index < self.nodes.len() {
            self.current_index += 1;
            self.nodes.get(self.current_index - 1)
        } else {
            None
        }
    }
    
    pub fn reset(&mut self) {
        self.current_index = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_coordinate_distance() {
        let a = MapCoordinate::new(0, 0, 0);
        let b = MapCoordinate::new(3, 4, 0);
        assert!((a.distance(&b) - 5.0).abs() < 0.001);
    }
    
    #[test]
    fn test_map_route() {
        let mut route = MapRoute::new();
        route.add_node(MapNode::new(MapNodeType::Start, 0, 0, 0));
        route.add_node(MapNode::new(MapNodeType::Waypoint, 10, 10, 0));
        
        assert!(route.get_current().is_some());
        assert_eq!(route.get_current().unwrap().node_type, MapNodeType::Start);
        
        route.next();
        assert_eq!(route.get_current().unwrap().node_type, MapNodeType::Waypoint);
    }
}
