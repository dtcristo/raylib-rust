use std::collections::HashSet;
use std::os::raw::c_int;

use {raw, Vector2};

/// Gesture type
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Gesture {
    None,
    Tap,
    DoubleTap,
    Hold,
    Drag,
    SwipeRight,
    SwipeLeft,
    SwipeUp,
    SwipeDown,
    PinchIn,
    PinchOut,
}
impl Gesture {
    fn from_raw(raw: c_int) -> Gesture {
        match raw as u32 {
            raw::Gestures::GESTURE_NONE => Gesture::None,
            raw::Gestures::GESTURE_TAP => Gesture::Tap,
            raw::Gestures::GESTURE_DOUBLETAP => Gesture::DoubleTap,
            raw::Gestures::GESTURE_HOLD => Gesture::Hold,
            raw::Gestures::GESTURE_DRAG => Gesture::Drag,
            raw::Gestures::GESTURE_SWIPE_RIGHT => Gesture::SwipeRight,
            raw::Gestures::GESTURE_SWIPE_LEFT => Gesture::SwipeLeft,
            raw::Gestures::GESTURE_SWIPE_UP => Gesture::SwipeUp,
            raw::Gestures::GESTURE_SWIPE_DOWN => Gesture::SwipeDown,
            raw::Gestures::GESTURE_PINCH_IN => Gesture::PinchIn,
            raw::Gestures::GESTURE_PINCH_OUT => Gesture::PinchOut,
            _ => panic!("Invalid Gesture value `{}`.", raw),
        }
    }
    fn into_raw(self) -> c_int {
        (match self {
            Gesture::None => raw::Gestures::GESTURE_NONE,
            Gesture::Tap => raw::Gestures::GESTURE_TAP,
            Gesture::DoubleTap => raw::Gestures::GESTURE_DOUBLETAP,
            Gesture::Hold => raw::Gestures::GESTURE_HOLD,
            Gesture::Drag => raw::Gestures::GESTURE_DRAG,
            Gesture::SwipeRight => raw::Gestures::GESTURE_SWIPE_RIGHT,
            Gesture::SwipeLeft => raw::Gestures::GESTURE_SWIPE_LEFT,
            Gesture::SwipeUp => raw::Gestures::GESTURE_SWIPE_UP,
            Gesture::SwipeDown => raw::Gestures::GESTURE_SWIPE_DOWN,
            Gesture::PinchIn => raw::Gestures::GESTURE_PINCH_IN,
            Gesture::PinchOut => raw::Gestures::GESTURE_PINCH_OUT,
        } as c_int)
    }
}

/// Enable a set of gestures
pub fn set_gestures_enabled(gesture_flags: HashSet<Gesture>) {
    let raw_flags = gesture_flags.iter().fold(0, |acc, g| acc | g.into_raw()) as u32;
    unsafe { raw::SetGesturesEnabled(raw_flags) }
}
/// Check if a gesture has been detected
pub fn is_gesture_detected(gesture: Gesture) -> bool {
    unsafe { raw::IsGestureDetected(gesture.into_raw() as i32) == raw::bool_::true_ }
}
/// Get latest detected gesture
pub fn get_gesture_detected() -> Gesture {
    Gesture::from_raw(unsafe { raw::GetGestureDetected() })
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
