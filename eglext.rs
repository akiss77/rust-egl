/* automatically generated by rust-bindgen */

use std::libc::*;
use egl::*;

// defines
pub type EGLImageKHR = *c_void;


pub static EGL_NATIVE_BUFFER_ANDROID: c_uint = 0x3140;  /* eglCreateImageKHR target */
pub static EGL_KHR_image_pixmap: c_uint = 1;
pub static EGL_KHR_image_base: c_uint = 1;
pub static EGL_KHR_image: c_uint = 1;

pub static EGL_NATIVE_PIXMAP_KHR: c_uint = 0x30B0;  /* eglCreateImageKHR target */
pub static EGL_MATCH_FORMAT_KHR: c_uint = 0x3043;  /* EGLConfig attribute */
pub static EGL_FORMAT_RGB_565_EXACT_KHR: c_uint = 0x30C0;  /* EGL_MATCH_FORMAT_KHR value */
pub static EGL_FORMAT_RGB_565_KHR: c_uint = 0x30C1;  /* EGL_MATCH_FORMAT_KHR value */
pub static EGL_FORMAT_RGBA_8888_EXACT_KHR: c_uint = 0x30C2;  /* EGL_MATCH_FORMAT_KHR value */
pub static EGL_FORMAT_RGBA_8888_KHR: c_uint = 0x30C3;  /* EGL_MATCH_FORMAT_KHR value */
pub static EGL_MAP_PRESERVE_PIXELS_KHR: c_uint = 0x30C4;  /* eglLockSurfaceKHR attribute */
pub static EGL_LOCK_USAGE_HINT_KHR: c_uint = 0x30C5;  /* eglLockSurfaceKHR attribute */
pub static EGL_IMAGE_PRESERVED_KHR: c_uint = 0x30D2;  /* eglCreateImageKHR attribute */
pub static EGL_GL_TEXTURE_2D_KHR: c_uint = 0x30B1;  /* eglCreateImageKHR target */
pub static EGL_GL_TEXTURE_LEVEL_KHR: c_uint = 0x30BC;  /* eglCreateImageKHR attribute */
pub static EGL_BITMAP_POINTER_KHR: c_uint = 0x30C6;  /* eglQuerySurface attribute */
pub static EGL_BITMAP_PITCH_KHR: c_uint = 0x30C7;  /* eglQuerySurface attribute */
pub static EGL_BITMAP_ORIGIN_KHR: c_uint =0x30C8;  /* eglQuerySurface attribute */
pub static EGL_BITMAP_PIXEL_RED_OFFSET_KHR: c_uint = 0x30C9;  /* eglQuerySurface attribute */
pub static EGL_BITMAP_PIXEL_GREEN_OFFSET_KHR: c_uint = 0x30CA;  /* eglQuerySurface attribute */
pub static EGL_BITMAP_PIXEL_BLUE_OFFSET_KHR: c_uint = 0x30CB;  /* eglQuerySurface attribute */
pub static EGL_BITMAP_PIXEL_ALPHA_OFFSET_KHR: c_uint = 0x30CC;  /* eglQuerySurface attribute */
pub static EGL_BITMAP_PIXEL_LUMINANCE_OFFSET_KHR: c_uint = 0x30CD;  /* eglQuerySurface attribute */




#[cfg(target_os = "android")]
#[link(name = "EGL")]
extern {}

pub fn CreateImageKHR(dpy: EGLDisplay, context: EGLContext, target: EGLenum,
                      buffer: EGLClientBuffer, attrib_list: *EGLint) -> EGLImageKHR {
    unsafe {
        return eglCreateImageKHR(dpy, context, target, buffer, attrib_list);
    }
}

pub fn DestroyImageKHR(dpy: EGLDisplay, image: EGLImageKHR) -> EGLBoolean {
    unsafe {
        return eglDestroyImageKHR(dpy, image);
    }
}

extern {
    fn eglCreateImageKHR(dpy: EGLDisplay, context: EGLContext, target: EGLenum,
                         buffer: EGLClientBuffer, attrib_list: *EGLint) -> EGLImageKHR; 
    fn eglDestroyImageKHR(dpy: EGLDisplay, image: EGLImageKHR) -> EGLBoolean;
}