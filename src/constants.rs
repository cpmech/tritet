pub(crate) const TRITET_SUCCESS: i32 = 0;

pub(crate) const TRITET_ERROR_NULL_DATA: i32 = 10;
pub(crate) const TRITET_ERROR_STRING_CONCAT: i32 = 20;
pub(crate) const TRITET_ERROR_INITIALIZE_FAILED: i32 = 30;

pub(crate) const TRITET_ERROR_NULL_POINT_LIST: i32 = 100;
pub(crate) const TRITET_ERROR_NULL_SEGMENT_LIST: i32 = 200;
pub(crate) const TRITET_ERROR_NULL_FACET_LIST: i32 = 300;
pub(crate) const TRITET_ERROR_NULL_FACET_POLYGON_LIST: i32 = 400;
pub(crate) const TRITET_ERROR_NULL_REGION_LIST: i32 = 500;
pub(crate) const TRITET_ERROR_NULL_HOLE_LIST: i32 = 600;

pub(crate) const TRITET_ERROR_INVALID_POINT_INDEX: i32 = 1000;
pub(crate) const TRITET_ERROR_INVALID_SEGMENT_INDEX: i32 = 2000;
pub(crate) const TRITET_ERROR_INVALID_SEGMENT_POINT_ID: i32 = 3000;
pub(crate) const TRITET_ERROR_INVALID_FACET_INDEX: i32 = 4000;
pub(crate) const TRITET_ERROR_INVALID_FACET_POINT_INDEX: i32 = 5000;
pub(crate) const TRITET_ERROR_INVALID_FACET_POINT_ID: i32 = 6000;
pub(crate) const TRITET_ERROR_INVALID_REGION_INDEX: i32 = 7000;
pub(crate) const TRITET_ERROR_INVALID_HOLE_INDEX: i32 = 8000;
pub(crate) const TRITET_ERROR_INVALID_FACET_NUM_POLYGON: i32 = 9000;

/// Maps indices used in this library (tritet) to indices used in Triangle
///
/// ```text
/// This library (tritet)      Triangle
///         NODES               CORNERS
///           2                    2
///          / \                  / \
///         /   \                /   \
///        5     4              4     3
///       /       \            /       \
///      /         \          /         \
///     0-----3-----1        0-----5-----1
/// ```
pub(crate) const TRITET_TO_TRIANGLE: [usize; 6] = [0, 1, 2, 5, 3, 4];

/// Maps indices used in this library (tritet) to indices used in Tetgen
///
/// ```text
///       This library (tritet)                          Tetgen
///               NODES                                  CORNERS
///             |                                         |
///             3                                         3
///            /|`.                                      /|`.
///            ||  `,                                    ||  `,
///           / |    ',                                 / |    ',
///           | |      \                                | |      \
///          /  |       `.                             /  |       `.
///          |  |         `,                           |  |         `,
///         /   7            9                        /   5            4
///         |   |             \                       |   |             \
///        /    |              `.                    /    |              `.
///        |    |                ',                  |    |                ',
///       8     |                  \                8     |                  \
///       |     0 ,,_               `.              |     0 ,,_               `.
///      |     /     ``'-., 6         `.           |     /     ``'-., 9         `.
///      |    /               `''-.,,_  ',         |    /               `''-.,,_  ',
///     |    /                        ``'2 ,,     |    /                        ``'2 ,,
///     |   '                       ,.-``         |   '                       ,.-``
///    |   4                   _,-'`             |   6                   _,-'`
///    ' /                 ,.'`                  ' /                 ,.'`
///   | /             _ 5 `                     | /             _ 7 `
///   '/          ,-'`                          '/          ,-'`
///  |/      ,.-``                             |/      ,.-``
///  /  _,-``                                  /  _,-``
/// 1 '`                                      1 '`
/// ```
pub(crate) const TRITET_TO_TETGEN: [usize; 10] = [0, 1, 2, 3, 6, 7, 9, 5, 8, 4];

/// Defines a set of "light" colors
pub(crate) const LIGHT_COLORS: [&'static str; 17] = [
    "#cbe4f9", "#cdf5f6", "#eff9da", "#f9ebdf", "#f9d8d6", "#d6cdea", "#acddde", "#caf1de",
    "#e1f8dc", "#fef8dd", "#ffe7c7", "#f7d8ba", "#d0fffe", "#fffddb", "#e4ffde", "#ffd3fd",
    "#ffe7d3",
];
