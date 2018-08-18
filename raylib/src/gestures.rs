//! Gestures and Touch Handling Functions

use std::os::raw::{c_int, c_uint};

use num_traits::FromPrimitive;

use {raw, BitFlags, Vector2};

/// Gesture types
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumFlags, FromPrimitive)]
pub enum Gesture {
    Tap = 1,         // raw::Gestures::GESTURE_TAP
    DoubleTap = 2,   // raw::Gestures::GESTURE_DOUBLETAP
    Hold = 4,        // raw::Gestures::GESTURE_HOLD
    Drag = 8,        // raw::Gestures::GESTURE_DRAG
    SwipeRight = 16, // raw::Gestures::GESTURE_SWIPE_RIGHT
    SwipeLeft = 32,  // raw::Gestures::GESTURE_SWIPE_LEFT
    SwipeUp = 64,    // raw::Gestures::GESTURE_SWIPE_UP
    SwipeDown = 128, // raw::Gestures::GESTURE_SWIPE_DOWN
    PinchIn = 256,   // raw::Gestures::GESTURE_PINCH_IN
    PinchOut = 512,  // raw::Gestures::GESTURE_PINCH_OUT
}

/// Enable a set of gestures
pub fn set_gestures_enabled(gesture_flags: BitFlags<Gesture>) {
    let raw_gesture_flags = gesture_flags.bits() as c_uint;
    unsafe { raw::SetGesturesEnabled(raw_gesture_flags) }
}
/// Check if a gesture has been detected
pub fn is_gesture_detected(gesture: Gesture) -> bool {
    let raw_gesture = gesture as c_int;
    unsafe { raw::IsGestureDetected(raw_gesture) == raw::bool_::true_ }
}
/// Get latest detected gesture
pub fn get_gesture_detected() -> Option<Gesture> {
    Gesture::from_i32(unsafe { raw::GetGestureDetected() })
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
