use std::os::raw::{c_int, c_uint};

use {raw, Vector2};

bitflags! {
    /// Gesture type
    ///
    /// NOTE: It could be used as flags to enable only some gestures
    pub struct Gesture: u32 {
        const NONE = raw::Gestures::GESTURE_NONE;
        const TAP = raw::Gestures::GESTURE_TAP;
        const DOUBLETAP = raw::Gestures::GESTURE_DOUBLETAP;
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
    let raw_gesture = gesture.bits() as c_int;
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
