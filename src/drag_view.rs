use std::cell::{Cell, RefCell};
use std::path::PathBuf;

use objc2::rc::Retained;
use objc2::runtime::{AnyObject, ProtocolObject};
use objc2::{define_class, msg_send, AnyThread, DefinedClass, MainThreadMarker, MainThreadOnly};
use objc2_app_kit::{
    NSApplication, NSDragOperation, NSDraggingContext, NSDraggingItem, NSDraggingSession,
    NSDraggingSource, NSEvent, NSImage, NSImageView, NSPasteboardItem, NSPasteboardWriting,
    NSStackView, NSTextField, NSUserInterfaceLayoutOrientation, NSView, NSWorkspace,
};
use objc2_foundation::{NSObjectProtocol, NSPoint, NSRect, NSSize, NSString, NSURL};

pub struct DragViewIvars {
    all_files: RefCell<Vec<PathBuf>>,
    and_exit: bool,
    drag_completed: Cell<bool>,
}

define_class!(
    #[unsafe(super(NSView))]
    #[thread_kind = MainThreadOnly]
    #[name = "DragView"]
    #[ivars = DragViewIvars]
    pub struct DragView;

    unsafe impl NSObjectProtocol for DragView {}

    unsafe impl NSDraggingSource for DragView {
        #[unsafe(method(draggingSession:sourceOperationMaskForDraggingContext:))]
        fn dragging_session_source_operation_mask(
            &self,
            _session: &NSDraggingSession,
            _context: NSDraggingContext,
        ) -> NSDragOperation {
            NSDragOperation::Copy
        }

        #[unsafe(method(draggingSession:endedAtPoint:operation:))]
        fn dragging_session_ended(
            &self,
            _session: &NSDraggingSession,
            _point: NSPoint,
            operation: NSDragOperation,
        ) {
            if operation != NSDragOperation::None {
                self.ivars().drag_completed.set(true);
                if self.ivars().and_exit {
                    let mtm = MainThreadMarker::from(self);
                    let app = NSApplication::sharedApplication(mtm);
                    app.terminate(None);
                }
            }
        }
    }

    impl DragView {
        #[unsafe(method(mouseDown:))]
        fn mouse_down(&self, event: &NSEvent) {
            self.start_drag(event);
        }

        #[unsafe(method(mouseDragged:))]
        fn mouse_dragged(&self, _event: &NSEvent) {
            // Drag is handled by the session started in mouseDown
        }
    }
);

impl DragView {
    pub fn new(
        mtm: MainThreadMarker,
        all_files: Vec<PathBuf>,
        and_exit: bool,
        _icon_only: bool,
    ) -> Retained<Self> {
        let frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(150.0, 40.0));

        let this = mtm.alloc::<Self>();
        let this = this.set_ivars(DragViewIvars {
            all_files: RefCell::new(all_files.clone()),
            and_exit,
            drag_completed: Cell::new(false),
        });
        let this: Retained<Self> = unsafe { msg_send![super(this), initWithFrame: frame] };

        this.setup_subviews(mtm, all_files.len());

        this
    }

    fn setup_subviews(&self, mtm: MainThreadMarker, file_count: usize) {
        // Get generic file icon
        let workspace = NSWorkspace::sharedWorkspace();
        let icon: Retained<NSImage> = workspace.iconForFileType(&NSString::from_str("public.data"));
        icon.setSize(NSSize::new(32.0, 32.0));

        let image_view = NSImageView::imageViewWithImage(&icon, mtm);
        image_view.setTranslatesAutoresizingMaskIntoConstraints(false);

        let text = if file_count == 1 {
            "1 file".to_string()
        } else {
            format!("{} files", file_count)
        };

        let label = NSTextField::labelWithString(&NSString::from_str(&text), mtm);
        label.setTranslatesAutoresizingMaskIntoConstraints(false);

        // Horizontal stack for icon + label
        let stack = NSStackView::new(mtm);
        stack.setOrientation(NSUserInterfaceLayoutOrientation::Horizontal);
        stack.setSpacing(8.0);
        stack.setTranslatesAutoresizingMaskIntoConstraints(false);

        stack.addArrangedSubview(&image_view);
        stack.addArrangedSubview(&label);

        self.addSubview(&stack);

        // Center the stack
        let stack_center_x = stack.centerXAnchor();
        let self_center_x = self.centerXAnchor();
        let constraint_x = stack_center_x.constraintEqualToAnchor(&self_center_x);
        constraint_x.setActive(true);

        let stack_center_y = stack.centerYAnchor();
        let self_center_y = self.centerYAnchor();
        let constraint_y = stack_center_y.constraintEqualToAnchor(&self_center_y);
        constraint_y.setActive(true);
    }

    fn start_drag(&self, event: &NSEvent) {
        let all_files = self.ivars().all_files.borrow();
        let workspace = NSWorkspace::sharedWorkspace();

        let mut dragging_items: Vec<Retained<NSDraggingItem>> = Vec::new();

        for (i, file_path) in all_files.iter().enumerate() {
            let path_str = NSString::from_str(&file_path.to_string_lossy());
            let file_url = NSURL::fileURLWithPath(&path_str);

            let pb_item = NSPasteboardItem::new();

            let url_string = file_url.absoluteString();
            if let Some(url_str) = url_string {
                unsafe {
                    pb_item.setString_forType(&url_str, objc2_app_kit::NSPasteboardTypeFileURL);
                }
            }

            let icon = workspace.iconForFile(&path_str);
            icon.setSize(NSSize::new(48.0, 48.0));

            let pb_writer: &ProtocolObject<dyn NSPasteboardWriting> =
                ProtocolObject::from_ref(&*pb_item);
            let dragging_item =
                NSDraggingItem::initWithPasteboardWriter(NSDraggingItem::alloc(), pb_writer);

            let drag_frame = NSRect::new(
                NSPoint::new(0.0, (i as f64) * 52.0),
                NSSize::new(48.0, 48.0),
            );
            let icon_obj: &AnyObject = unsafe { std::mem::transmute(&*icon) };
            unsafe { dragging_item.setDraggingFrame_contents(drag_frame, Some(icon_obj)) };

            dragging_items.push(dragging_item);
        }

        let items = objc2_foundation::NSArray::from_retained_slice(&dragging_items);
        let source: &ProtocolObject<dyn NSDraggingSource> = ProtocolObject::from_ref(self);
        self.beginDraggingSessionWithItems_event_source(&items, event, source);
    }
}
