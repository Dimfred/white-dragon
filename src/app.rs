use std::path::PathBuf;

use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2::{define_class, msg_send, DefinedClass, MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{
    NSApplication, NSApplicationActivationPolicy, NSApplicationDelegate,
};
use objc2_foundation::{NSNotification, NSObject, NSObjectProtocol};

use crate::window;

pub struct AppDelegateIvars {
    files: Vec<PathBuf>,
    and_exit: bool,
    icon_only: bool,
}

define_class!(
    #[unsafe(super(NSObject))]
    #[thread_kind = MainThreadOnly]
    #[name = "AppDelegate"]
    #[ivars = AppDelegateIvars]
    pub struct AppDelegate;

    unsafe impl NSObjectProtocol for AppDelegate {}

    unsafe impl NSApplicationDelegate for AppDelegate {
        #[unsafe(method(applicationDidFinishLaunching:))]
        fn did_finish_launching(&self, _notification: &NSNotification) {
            let ivars = self.ivars();
            window::create_window(
                MainThreadMarker::from(self),
                &ivars.files,
                ivars.and_exit,
                ivars.icon_only,
            );
        }

        #[unsafe(method(applicationShouldTerminateAfterLastWindowClosed:))]
        fn should_terminate_after_last_window_closed(&self, _sender: &NSApplication) -> bool {
            true
        }
    }
);

impl AppDelegate {
    fn new(mtm: MainThreadMarker, files: Vec<PathBuf>, and_exit: bool, icon_only: bool) -> Retained<Self> {
        let this = mtm.alloc::<Self>();
        let this = this.set_ivars(AppDelegateIvars {
            files,
            and_exit,
            icon_only,
        });
        unsafe { msg_send![super(this), init] }
    }
}

pub fn run(files: Vec<PathBuf>, and_exit: bool, icon_only: bool) {
    let mtm = MainThreadMarker::new().expect("Must be called from main thread");
    let app = NSApplication::sharedApplication(mtm);
    app.setActivationPolicy(NSApplicationActivationPolicy::Accessory);

    let delegate = AppDelegate::new(mtm, files, and_exit, icon_only);
    let delegate_obj: &ProtocolObject<dyn NSApplicationDelegate> =
        ProtocolObject::from_ref(&*delegate);
    app.setDelegate(Some(delegate_obj));

    app.run();
}
