use std::path::PathBuf;

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{define_class, msg_send, MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{
    NSApplication, NSBackingStoreType, NSColor, NSEvent,
    NSWindow, NSWindowDelegate, NSWindowStyleMask,
};
use objc2_foundation::{NSObject, NSObjectProtocol, NSPoint, NSRect, NSSize};

use crate::drag_view::DragView;

pub struct WindowDelegateIvars {}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "WindowDelegate"]
    #[ivars = WindowDelegateIvars]
    pub struct WindowDelegate;

    unsafe impl NSObjectProtocol for WindowDelegate {}

    unsafe impl NSWindowDelegate for WindowDelegate {
        #[unsafe(method(windowShouldClose:))]
        fn window_should_close(&self, _sender: &NSWindow) -> bool {
            true
        }
    }
);

impl WindowDelegate {
    fn new(mtm: MainThreadMarker) -> Retained<Self> {
        let this = mtm.alloc::<Self>();
        let this = this.set_ivars(WindowDelegateIvars {});
        unsafe { msg_send![super(this), init] }
    }
}

pub struct KeyHandlingWindowIvars {}

define_class!(
    #[unsafe(super(NSWindow))]
    #[thread_kind = MainThreadOnly]
    #[name = "KeyHandlingWindow"]
    #[ivars = KeyHandlingWindowIvars]
    pub struct KeyHandlingWindow;

    unsafe impl NSObjectProtocol for KeyHandlingWindow {}

    impl KeyHandlingWindow {
        #[unsafe(method(keyDown:))]
        fn key_down(&self, event: &NSEvent) {
            let key_code = event.keyCode();
            // Escape key = 53
            if key_code == 53 {
                self.close();
            }
        }

        #[unsafe(method(canBecomeKeyWindow))]
        fn can_become_key_window(&self) -> bool {
            true
        }
    }
);

impl KeyHandlingWindow {
    fn new(
        mtm: MainThreadMarker,
        frame: NSRect,
        style_mask: NSWindowStyleMask,
        backing: NSBackingStoreType,
        defer: bool,
    ) -> Retained<Self> {
        let this = mtm.alloc::<Self>();
        let this = this.set_ivars(KeyHandlingWindowIvars {});
        unsafe {
            msg_send![
                super(this),
                initWithContentRect: frame,
                styleMask: style_mask,
                backing: backing,
                defer: defer
            ]
        }
    }
}

pub fn create_window(
    mtm: MainThreadMarker,
    files: &[PathBuf],
    and_exit: bool,
    icon_only: bool,
) {
    let window_width: f64 = 150.0;
    let window_height: f64 = 60.0;

    // Get mouse location for window positioning
    let mouse_location = NSEvent::mouseLocation();

    // Calculate window frame near mouse
    let window_x = mouse_location.x - window_width / 2.0;
    let window_y = mouse_location.y - window_height / 2.0;

    let frame = NSRect::new(
        NSPoint::new(window_x, window_y),
        NSSize::new(window_width, window_height),
    );

    let style_mask = NSWindowStyleMask::Titled
        | NSWindowStyleMask::Closable
        | NSWindowStyleMask::Miniaturizable;

    let window = KeyHandlingWindow::new(
        mtm,
        frame,
        style_mask,
        NSBackingStoreType::Buffered,
        false,
    );

    // Set window properties
    window.setTitle(&objc2_foundation::NSString::from_str("White Dragon"));
    unsafe {
        window.setLevel(objc2_app_kit::NSFloatingWindowLevel);
        window.setReleasedWhenClosed(false);
    }

    // Create delegate
    let delegate = WindowDelegate::new(mtm);
    let delegate_obj: &ProtocolObject<dyn NSWindowDelegate> =
        ProtocolObject::from_ref(&*delegate);
    window.setDelegate(Some(delegate_obj));
    // Keep delegate alive by forgetting it (window will retain it)
    std::mem::forget(delegate);

    // Create single drag view with all files
    let drag_view = DragView::new(mtm, files.to_vec(), and_exit, icon_only);

    // Set the drag view as the content view
    window.setContentView(Some(&drag_view));

    // Set background color
    window.setBackgroundColor(Some(&NSColor::windowBackgroundColor()));

    // Show and activate
    window.makeKeyAndOrderFront(None);

    let app = NSApplication::sharedApplication(mtm);
    #[allow(deprecated)]
    {
        app.activateIgnoringOtherApps(true);
    }

    // Keep window alive
    std::mem::forget(window);
}
