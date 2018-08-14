use std::os::raw::c_uint;

use {raw, Vector2};

// /// Gesture type
// #[repr(u32)]
// #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
// pub enum Gesture {
//     None = raw::Gestures::GESTURE_NONE,
//     Tap = raw::Gestures::GESTURE_TAP,
//     DoubleTap = raw::Gestures::GESTURE_DOUBLETAP,
//     Hold = raw::Gestures::GESTURE_HOLD,
//     Drag = raw::Gestures::GESTURE_DRAG,
//     SwipeRight = raw::Gestures::GESTURE_SWIPE_RIGHT,
//     SwipeLeft = raw::Gestures::GESTURE_SWIPE_LEFT,
//     SwipeUp = raw::Gestures::GESTURE_SWIPE_UP,
//     SwipeDown = raw::Gestures::GESTURE_SWIPE_DOWN,
//     PinchIn = raw::Gestures::GESTURE_PINCH_IN,
//     PinchOut = raw::Gestures::GESTURE_PINCH_OUT,
// }
// impl Gesture {
//     fn from_raw(raw: c_int) -> Gesture {
//         match raw as u32 {
//             raw::Gestures::GESTURE_NONE => Gesture::None,
//             raw::Gestures::GESTURE_TAP => Gesture::Tap,
//             raw::Gestures::GESTURE_DOUBLETAP => Gesture::DoubleTap,
//             raw::Gestures::GESTURE_HOLD => Gesture::Hold,
//             raw::Gestures::GESTURE_DRAG => Gesture::Drag,
//             raw::Gestures::GESTURE_SWIPE_RIGHT => Gesture::SwipeRight,
//             raw::Gestures::GESTURE_SWIPE_LEFT => Gesture::SwipeLeft,
//             raw::Gestures::GESTURE_SWIPE_UP => Gesture::SwipeUp,
//             raw::Gestures::GESTURE_SWIPE_DOWN => Gesture::SwipeDown,
//             raw::Gestures::GESTURE_PINCH_IN => Gesture::PinchIn,
//             raw::Gestures::GESTURE_PINCH_OUT => Gesture::PinchOut,
//             _ => panic!("Invalid Gesture value `{}`.", raw),
//         }
//     }
//     fn into_raw(self) -> c_int {
//         self as c_int
//     }
// }

bitflags! {
    /// Gesture type
    ///
    /// NOTE: It could be used as flags to enable only some gestures
    pub struct Gesture: u32 {
        const NONE = raw::Gestures::GESTURE_NONE;
        const TAP = raw::Gestures::GESTURE_TAP;
        const DOUBLE_TAP = raw::Gestures::GESTURE_DOUBLETAP;
        const HOLD = raw::Gestures::GESTURE_HOLD;
        const DRAG = raw::Gestures::GESTURE_DRAG;
        const SWIPE_RIGHT = raw::Gestures::GESTURE_SWIPE_RIGHT;
        const SWIPE_LEFT = raw::Gestures::GESTURE_SWIPE_LEFT;
        const SWIPE_UP = raw::Gestures::GESTURE_SWIPE_UP;
        const SWIPE_DOWN = raw::Gestures::GESTURE_SWIPE_DOWN;
        const PINCH_IN = raw::Gestures::GESTURE_PINCH_IN;
        const PINCH_OUT = raw::Gestures::GESTURE_PINCH_OUT;
    }
}

/// Enable a set of gestures
pub fn set_gestures_enabled(gesture_flags: Gesture) {
    let raw_gesture_flags = gesture_flags.bits() as c_uint;
    unsafe { raw::SetGesturesEnabled(raw_gesture_flags) }
}
/// Check if a gesture has been detected
pub fn is_gesture_detected(gesture: Gesture) -> bool {
    let raw_gesture = gesture.bits() as i32;
    unsafe { raw::IsGestureDetected(raw_gesture) == raw::bool_::true_ }
}
/// Get latest detected gesture
pub fn get_gesture_detected() -> Gesture {
    Gesture::from_bits_truncate(unsafe { raw::GetGestureDetected() as u32 })
}
/// Get touch points count
pub fn get_touch_points_count() -> i32 {
    unsafe { raw::GetTouchPointsCount() }
}
/// Get gesture hold time in milliseconds
pub fn get_gesture_hold_duration() -> f32 {
    unsafe { raw::GetGestureHoldDuration() }
}
/// Get gesture drag vector
pub fn get_gesture_drag_vector() -> Vector2 {
    Vector2::from_raw(unsafe { raw::GetGestureDragVector() })
}
/// Get gesture drag angle
pub fn get_gesture_drag_angle() -> f32 {
    unsafe { raw::GetGestureDragAngle() }
}
/// Get gesture pinch delta
pub fn get_gesture_pinch_vector() -> Vector2 {
    Vector2::from_raw(unsafe { raw::GetGesturePinchVector() })
}
/// Get gesture pinch angle
pub fn get_gesture_pinch_angle() -> f32 {
    unsafe { raw::GetGesturePinchAngle() }
}
