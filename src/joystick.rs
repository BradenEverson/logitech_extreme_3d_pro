//! Channel bindings for all buttons and axis the joystick has

/// All different rotational axis
pub enum Axis {
    /// Axis 1
    StickUpDown,
    /// Axis 0
    StickLeftRight,
    /// Axis 5
    StickYaw,
    /// Axis 17
    JoyPadUpDown,
    /// Axis 16
    JoyPadLeftRight,
    /// Axis 6
    Throttle,
}

/// All different buttons 
pub enum Button {
    /// 288
    Trigger,
    /// 289
    StickSide,
    /// 290
    Button3,
    /// 291
    Button4,
    /// 292
    Button5,
    /// 293
    Button6,
    /// 294
    Button7,
    /// 295
    Button8,
    /// 296
    Button9,
    /// 297
    Button10,
    /// 298
    Button11,
    /// 299
    Button12,
}
