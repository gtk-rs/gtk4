// Take a look at the license at the top of the repository in the LICENSE file.

use crate::{ShortcutController, ShortcutManager};
use glib::subclass::prelude::*;
use glib::translate::*;
use glib::Cast;

pub trait ShortcutManagerImpl: ObjectImpl {
    fn add_controller(&self, shortcut_manager: &Self::Type, controller: &ShortcutController);
    fn remove_controller(&self, shortcut_manager: &Self::Type, controller: &ShortcutController);
}

unsafe impl<T: ShortcutManagerImpl> IsImplementable<T> for ShortcutManager {
    fn interface_init(iface: &mut glib::Class<Self>) {
        let iface = iface.as_mut();

        iface.add_controller = Some(shortcut_manager_add_controller::<T>);
        iface.remove_controller = Some(shortcut_manager_remove_controller::<T>);
    }

    fn instance_init(_instance: &mut glib::subclass::InitializingObject<T>) {}
}

unsafe extern "C" fn shortcut_manager_add_controller<T: ShortcutManagerImpl>(
    shortcut_manager: *mut ffi::GtkShortcutManager,
    controller: *mut ffi::GtkShortcutController,
) {
    let instance = &*(shortcut_manager as *mut T::Instance);
    let imp = instance.get_impl();

    imp.add_controller(
        from_glib_borrow::<_, ShortcutManager>(shortcut_manager).unsafe_cast_ref(),
        &ShortcutController::from_glib_borrow(controller),
    )
}

unsafe extern "C" fn shortcut_manager_remove_controller<T: ShortcutManagerImpl>(
    shortcut_manager: *mut ffi::GtkShortcutManager,
    controller: *mut ffi::GtkShortcutController,
) {
    let instance = &*(shortcut_manager as *mut T::Instance);
    let imp = instance.get_impl();

    imp.remove_controller(
        from_glib_borrow::<_, ShortcutManager>(shortcut_manager).unsafe_cast_ref(),
        &ShortcutController::from_glib_borrow(controller),
    )
}
